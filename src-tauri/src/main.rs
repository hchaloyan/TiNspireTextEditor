// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tauri_plugin_dialog::DialogExt;

// =============================================================================
// 1. XML GENERATION
// =============================================================================

/// Builds the single Problem XML file that luna converts to .tns.
///
/// Correct usage: luna problem.xml output.tns
///
/// Schema taken from the reference Notepad document in the Luna v0.2
/// announcement. Critical details:
///   - wdgt ver="2.0"  (not "1.0")
///   - <np:mFlags>1024</np:mFlags> and <np:value>3</np:value> are REQUIRED
///   - <sym></sym> must be an empty inline tag before <card>
///   - Luna takes this ONE file directly -- no Document.xml, no directory
fn build_problem_xml(text: &str, is_bold: bool, hex_color: &str) -> String {
    let escape = |s: &str| {
        s.replace('&', "&amp;")
         .replace('<', "&lt;")
         .replace('>', "&gt;")
         .replace('"', "&quot;")
         .replace('\'', "&apos;")
    };

    // Determine style attributes
    let bold_attr = if is_bold { " bold=\"1\"" } else { "" };
    let color_attr = format!(" color=\"{}\"", hex_color); // e.g., rgb(255,0,0)

    let mut tree = String::from("<r2dtotree><node name=\"1doc\">");
    
    for line in text.split('\n') {
        let content = if line.is_empty() { " " } else { line };
        
        // Inject attributes into the LEAF tag
        tree.push_str(&format!(
            "<node name=\"1para\"><node name=\"1rtline\"><leaf name=\"1word\"{}{}>{}</leaf></node></node>",
            bold_attr,
            color_attr,
            escape(content)
        ));
    }
    
    tree.push_str("</node></r2dtotree>");

    // Double-escape the entire tree for the final XML
    let escaped_tree = escape(&tree);

    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<prob xmlns="urn:TI.Problem" ver="1.0" pbname="">
  <sym></sym>
  <card clay="0" h1="10000" h2="10000" w1="10000" w2="10000">
    <isDummyCard>0</isDummyCard>
    <wdgt xmlns:np="urn:TI.Notepad" type="TI.Notepad" ver="2.0">
      <np:mFlags>1024</np:mFlags>
      <np:value>3</np:value>
      <np:fmtxt>{}</np:fmtxt>
    </wdgt>
  </card>
</prob>"#,
        escaped_tree
    )
}

// =============================================================================
// 2. LUNA BINARY -- RESOLVE BUNDLED PATH
// =============================================================================

/// Returns the path to the bundled `luna` binary.
fn resolve_luna_binary(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    use tauri::Manager;

    // Production: bundled resource
    if let Ok(p) = app.path().resolve("luna.exe", tauri::path::BaseDirectory::Resource) {
        if p.exists() {
            return Ok(p);
        }
    }

    // Dev fallback: look relative to the working directory
    let dev_candidates = [
        "Luna-master/luna.exe",
        "Luna-master/luna",
        "../Luna-master/luna.exe",
        "../Luna-master/luna",
        "../../Luna-master/luna.exe",
        "../../Luna-master/luna",
    ];
    for candidate in &dev_candidates {
        let p = PathBuf::from(candidate);
        if p.exists() {
            return Ok(p.canonicalize().map_err(|e| e.to_string())?);
        }
    }

    Err("luna binary not found. Place luna.exe at Luna-master/luna.exe (dev) \
         or add it to tauri.conf.json resources.".to_string())
}

// =============================================================================
// 3. TAURI COMMANDS
// =============================================================================

/// Export result returned to the React frontend.
#[derive(serde::Serialize)]
pub struct ExportResult {
    saved_path: String,
}

/// Core export command: generates problem.xml -> runs luna -> shows save dialog.
#[tauri::command]
async fn export_tns(
    app: tauri::AppHandle,
    content: String,
    filename: String,
) -> Result<ExportResult, String> {
    eprintln!("[export_tns] starting export for '{filename}'");

    // 3a. Resolve luna
    let luna_bin = resolve_luna_binary(&app)
        .map_err(|e| { eprintln!("[export_tns] luna resolve failed: {e}"); e })?;
    eprintln!("[export_tns] luna binary: {}", luna_bin.display());

    // 3b. Create a plain temp directory (NOT TempDir -- it auto-deletes on drop
    //     which is unsafe across async suspension points)
    let tmp_dir = std::env::temp_dir().join(format!(
        "tns_export_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    ));
    fs::create_dir_all(&tmp_dir)
        .map_err(|e| format!("Failed to create temp dir: {e}"))?;
    eprintln!("[export_tns] temp dir: {}", tmp_dir.display());

    // 3c. Write the file with the EXACT name Luna expects
    // Ensure prob_xml_path is set to "Problem1.xml" as discussed!
    let prob_xml_path = tmp_dir.join("Problem1.xml");
    let tns_tmp_path  = tmp_dir.join("output.tns");

    // Update line 149: Passing default 'false' for bold and black for color
    let xml_content = build_problem_xml(&content, false, "rgb(0,0,0)");

    fs::write(&prob_xml_path, xml_content)
        .map_err(|e| format!("Failed to write Problem1.xml: {e}"))?;

    // 3d. Invoke luna with the correct working directory
    let output = Command::new(&luna_bin)
        .arg("Problem1.xml") // Use relative path if setting current_dir
        .arg("output.tns")
        .current_dir(&tmp_dir) // Run inside the temp dir
        // Alternatively, if Luna is bundled with DLLs:
        // .current_dir(luna_bin.parent().unwrap()) 
        .output()
        .map_err(|e| format!("Failed to launch luna: {e}"))?;

    // 3e. Check luna succeeded
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let _ = fs::remove_dir_all(&tmp_dir);
        return Err(format!(
            "luna failed (exit {:?})\nstdout: {stdout}\nstderr: {stderr}",
            output.status.code()
        ));
    }

    // 3f. Read the produced .tns bytes
    eprintln!("[export_tns] output.tns exists: {}", tns_tmp_path.exists());
    let tns_bytes = fs::read(&tns_tmp_path).map_err(|e| {
        let _ = fs::remove_dir_all(&tmp_dir);
        format!("Failed to read luna output: {e}")
    })?;
    eprintln!("[export_tns] tns size: {} bytes", tns_bytes.len());

    // 3g. Show save-file dialog
    let safe_stem = sanitize_filename(&filename);
    let suggested  = format!("{safe_stem}.tns");
    let (tx, rx)   = std::sync::mpsc::channel();

    app.dialog()
        .file()
        .set_title("Save .tns file")
        .set_file_name(&suggested)
        .add_filter("TI-Nspire file", &["tns"])
        .save_file(move |file_path| { let _ = tx.send(file_path); });

    let file_path = rx.recv().map_err(|e| format!("Dialog error: {e}"))?;

    let destination: PathBuf = match file_path {
        Some(p) => p.into_path().map_err(|e| format!("Invalid path: {e}"))?,
        None => {
            let _ = fs::remove_dir_all(&tmp_dir);
            return Err("Save cancelled by user.".to_string());
        }
    };

    // 3h. Write to destination
    let mut file = fs::File::create(&destination)
        .map_err(|e| format!("Failed to create destination file: {e}"))?;
    file.write_all(&tns_bytes)
        .map_err(|e| format!("Failed to write .tns bytes: {e}"))?;

    let _ = fs::remove_dir_all(&tmp_dir);
    eprintln!("[export_tns] done -- saved to {}", destination.display());

    Ok(ExportResult {
        saved_path: destination.to_string_lossy().to_string(),
    })
}

/// Returns a list of connected TI-Nspire calculators (stub).
#[tauri::command]
async fn list_calculators() -> Result<Vec<CalculatorInfo>, String> {
    Ok(vec![])
}

/// Transfer a .tns file to the calculator (stub).
#[tauri::command]
async fn send_to_calculator(
    _file_path: String,
    _usb_id: String,
    _remote_path: String,
) -> Result<String, String> {
    Err("Calculator transfer not yet implemented.".to_string())
}

// =============================================================================
// 4. HELPERS
// =============================================================================

fn sanitize_filename(name: &str) -> String {
    let s: String = name
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c => c,
        })
        .collect();
    if s.is_empty() { "untitled".to_string() } else { s }
}

#[derive(serde::Serialize)]
pub struct CalculatorInfo {
    name: String,
    usb_id: String,
}

// =============================================================================
// 5. ENTRY POINT
// =============================================================================

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            export_tns,
            list_calculators,
            send_to_calculator,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}

// =============================================================================
// UNIT TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xml_wraps_plain_text() {
        let xml = build_problem_xml("Hello, world!");
        assert!(xml.contains("<np:s>Hello, world!</np:s>"));
        assert!(xml.contains(r#"xmlns="urn:TI.Problem""#));
    }

    #[test]
    fn xml_escapes_special_chars() {
        let xml = build_problem_xml("a < b & c > d");
        assert!(xml.contains("a &lt; b &amp; c &gt; d"));
    }

    #[test]
    fn xml_splits_newlines_into_paragraphs() {
        let xml = build_problem_xml("line1\nline2");
        assert!(xml.contains("<np:p><np:s>line1</np:s></np:p>"));
        assert!(xml.contains("<np:p><np:s>line2</np:s></np:p>"));
    }

    #[test]
    fn xml_has_correct_widget_version() {
        let xml = build_problem_xml("test");
        assert!(xml.contains(r#"ver="2.0""#));
        assert!(xml.contains("<np:mFlags>1024</np:mFlags>"));
        assert!(xml.contains("<np:value>3</np:value>"));
    }

    #[test]
    fn sanitize_strips_illegal_chars() {
        assert_eq!(sanitize_filename("my/file:name"), "my_file_name");
        assert_eq!(sanitize_filename(""), "untitled");
        assert_eq!(sanitize_filename("normal"), "normal");
    }
}