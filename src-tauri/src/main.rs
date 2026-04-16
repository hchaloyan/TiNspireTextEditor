// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tempfile::tempdir;
use tauri_plugin_dialog::DialogExt;

// ─────────────────────────────────────────────────────────────────────────────
// 1. XML GENERATION
// ─────────────────────────────────────────────────────────────────────────────

/// Wraps plain text in the TI Notepad XML schema expected by luna.
fn build_document_xml(text: &str) -> String {
    // Escape the five XML special characters so arbitrary user text is safe.
    let escaped = text
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;");

    // Preserve newlines: each \n becomes a new <np:p> paragraph element.
    let paragraphs: String = escaped
        .split('\n')
        .map(|line| {
            format!(
                "<np:p><np:s>{}</np:s></np:p>",
                if line.is_empty() { " ".to_string() } else { line.to_string() }
            )
        })
        .collect::<Vec<_>>()
        .join("");

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<prob xmlns="urn:TI.Problem" ver="1.0" pbname="">
  <sym>
    <card clay="0" h1="10000" h2="10000" w1="10000" w2="10000">
      <isDummyCard>0</isDummyCard>
      <flag>0</flag>
      <wdgt xmlns:np="urn:TI.Notepad" type="TI.Notepad" ver="1.0">
        <np:fmtxt>{}</np:fmtxt>
      </wdgt>
    </card>
  </sym>
</prob>"#,
        paragraphs
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. LUNA BINARY — RESOLVE BUNDLED PATH
// ─────────────────────────────────────────────────────────────────────────────

/// Returns the path to the bundled `luna` binary.
///
/// In production Tauri builds the binary lives in the "resources" directory
/// next to the app executable, accessible via `app.path_resolver().resolve_resource()`.
/// In development we fall back to the repo-relative path so `cargo tauri dev` works.
fn resolve_luna_binary(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    // Try the Tauri resource path first (production & `tauri build`).
    use tauri::Manager;
if let Ok(resource_path) = app.path().resolve("luna.exe", tauri::path::BaseDirectory::Resource) {
        if resource_path.exists() {
            return Ok(resource_path);
        }
    }

    // Dev fallback: look for luna relative to the workspace root.
    let dev_candidates = [
        "Luna-master/luna",
        "../Luna-master/luna",
        "../../Luna-master/luna",
    ];
    for candidate in &dev_candidates {
        let p = PathBuf::from(candidate);
        if p.exists() {
            return Ok(p.canonicalize().map_err(|e| e.to_string())?);
        }
    }

    Err(
        "luna binary not found. \
         Place it at Luna-master/luna (dev) or add it to tauri.conf.json resources."
            .to_string(),
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. TAURI COMMANDS
// ─────────────────────────────────────────────────────────────────────────────

/// Export result returned to the React frontend.
#[derive(serde::Serialize)]
pub struct ExportResult {
    /// Absolute path where the .tns file was saved.
    saved_path: String,
}

/// Core export command: generates Document.xml → runs luna → shows save dialog.
///
/// # Arguments
/// * `content`  – raw note text from the editor
/// * `filename` – suggested stem for the output file (without extension)
#[tauri::command]
async fn export_tns(
    app: tauri::AppHandle,
    content: String,
    filename: String,
) -> Result<ExportResult, String> {
    // ── 3a. Resolve luna ────────────────────────────────────────────────────
    let luna_bin = resolve_luna_binary(&app)?;

    // ── 3b. Build a temp working directory ──────────────────────────────────
    let tmp = tempdir().map_err(|e| format!("Failed to create temp dir: {e}"))?;
    let xml_path = tmp.path().join("Document.xml");
    let tns_tmp_path = tmp.path().join("output.tns");

    // ── 3c. Write Document.xml ───────────────────────────────────────────────
    let xml = build_document_xml(&content);
    fs::write(&xml_path, &xml)
        .map_err(|e| format!("Failed to write Document.xml: {e}"))?;

    // ── 3d. Invoke luna ──────────────────────────────────────────────────────
    //
    //   luna <input.xml> <output.tns>
    //
    // luna is a CLI tool; we capture stderr for error reporting.
    let output = Command::new(&luna_bin)
        .arg(&xml_path)
        .arg(&tns_tmp_path)
        .output()
        .map_err(|e| {
            format!(
                "Failed to launch luna at '{}': {e}",
                luna_bin.display()
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!(
            "luna exited with status {}.\nstdout: {stdout}\nstderr: {stderr}",
            output.status
        ));
    }

    if !tns_tmp_path.exists() {
        return Err(
            "luna exited successfully but output.tns was not created. \
             Check luna version compatibility."
                .to_string(),
        );
    }

    // ── 3e. Read the produced .tns bytes ────────────────────────────────────
    let tns_bytes = fs::read(&tns_tmp_path)
        .map_err(|e| format!("Failed to read luna output: {e}"))?;

    // ── 3f. Show save-file dialog ────────────────────────────────────────────
    let safe_stem = sanitize_filename(&filename);
    let suggested = format!("{safe_stem}.tns");

    // v2 save_file is callback-based; use a oneshot channel to await the result.
    let (tx, rx) = std::sync::mpsc::channel();

    app.dialog()
        .file()
        .set_title("Save .tns file")
        .set_file_name(&suggested)
        .add_filter("TI-Nspire file", &["tns"])
        .save_file(move |file_path| {
            let _ = tx.send(file_path);
        });

    let file_path = rx.recv().map_err(|e| format!("Dialog error: {e}"))?;

    let destination: PathBuf = match file_path {
        Some(p) => p.into_path().map_err(|e| format!("Invalid path: {e}"))?,
        None => return Err("Save cancelled by user.".to_string()),
    };

    // ── 3g. Write bytes to destination ──────────────────────────────────────
    let mut file = fs::File::create(&destination)
        .map_err(|e| format!("Failed to create destination file: {e}"))?;
    file.write_all(&tns_bytes)
        .map_err(|e| format!("Failed to write .tns bytes: {e}"))?;

    Ok(ExportResult {
        saved_path: destination.to_string_lossy().to_string(),
    })
}

/// Returns a list of connected TI-Nspire calculators.
///
/// Currently a stub — returns an empty list with a helpful message.
/// Real implementation via libnspire shown in comments below.
#[tauri::command]
async fn list_calculators() -> Result<Vec<CalculatorInfo>, String> {
    // ── STUB ─────────────────────────────────────────────────────────────────
    // To implement for real on Linux/WSL:
    //
    //   1. Add to Cargo.toml:
    //        libnspire = "0.1"   (or whichever crate wraps libnspire)
    //
    //   2. Replace this stub with:
    //
    //      use libnspire::{Context, DeviceInfo};
    //      let ctx = Context::new().map_err(|e| e.to_string())?;
    //      let devices: Vec<_> = ctx
    //          .devices()
    //          .map_err(|e| e.to_string())?
    //          .into_iter()
    //          .map(|d: DeviceInfo| CalculatorInfo {
    //              name: d.name().unwrap_or("TI-Nspire").to_string(),
    //              usb_id: format!("{:04x}:{:04x}", d.vendor_id(), d.product_id()),
    //          })
    //          .collect();
    //      return Ok(devices);
    //
    // ─────────────────────────────────────────────────────────────────────────
    Ok(vec![]) // placeholder: no devices
}

/// Transfer a .tns file at `file_path` to the calculator at `usb_id`.
///
/// Stub — logs intent; real libnspire transfer shown in comments.
#[tauri::command]
async fn send_to_calculator(
    _file_path: String,
    _usb_id: String,
    _remote_path: String,
) -> Result<String, String> {
    // ── STUB ─────────────────────────────────────────────────────────────────
    // Real implementation sketch:
    //
    //   let ctx = libnspire::Context::new().map_err(|e| e.to_string())?;
    //   let device = ctx
    //       .open_by_usb_id(&usb_id)
    //       .map_err(|e| format!("Could not open calculator: {e}"))?;
    //   let bytes = std::fs::read(&file_path)
    //       .map_err(|e| format!("Could not read file: {e}"))?;
    //   device
    //       .send_file(&remote_path, &bytes)
    //       .map_err(|e| format!("Transfer failed: {e}"))?;
    //   return Ok(format!("Sent {} to {}", file_path, usb_id));
    // ─────────────────────────────────────────────────────────────────────────
    Err("Calculator transfer not yet implemented. Connect via USB and use TI's own software for now.".to_string())
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. HELPERS
// ─────────────────────────────────────────────────────────────────────────────

/// Strip characters that are illegal in filenames on Windows/macOS/Linux.
fn sanitize_filename(name: &str) -> String {
    let sanitized: String = name
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c => c,
        })
        .collect();
    if sanitized.is_empty() {
        "untitled".to_string()
    } else {
        sanitized
    }
}

/// Info returned for each connected calculator.
#[derive(serde::Serialize)]
pub struct CalculatorInfo {
    name: String,
    usb_id: String,
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. ENTRY POINT
// ─────────────────────────────────────────────────────────────────────────────

fn main() {
    tauri::Builder::default()
        // 1. Register the v2 Plugins
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        // 2. Your existing custom commands
        .invoke_handler(tauri::generate_handler![
            export_tns,
            list_calculators,
            send_to_calculator,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}

// ─────────────────────────────────────────────────────────────────────────────
// UNIT TESTS
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xml_wraps_plain_text() {
        let xml = build_document_xml("Hello, world!");
        assert!(xml.contains("<np:s>Hello, world!</np:s>"));
        assert!(xml.contains(r#"xmlns="urn:TI.Problem""#));
    }

    #[test]
    fn xml_escapes_special_chars() {
        let xml = build_document_xml("a < b & c > d");
        assert!(xml.contains("a &lt; b &amp; c &gt; d"));
    }

    #[test]
    fn xml_splits_newlines_into_paragraphs() {
        let xml = build_document_xml("line1\nline2");
        assert!(xml.contains("<np:p><np:s>line1</np:s></np:p>"));
        assert!(xml.contains("<np:p><np:s>line2</np:s></np:p>"));
    }

    #[test]
    fn sanitize_strips_illegal_chars() {
        assert_eq!(sanitize_filename("my/file:name"), "my_file_name");
        assert_eq!(sanitize_filename(""), "untitled");
        assert_eq!(sanitize_filename("normal"), "normal");
    }
}

