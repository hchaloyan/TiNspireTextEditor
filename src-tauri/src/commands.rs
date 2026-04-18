use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use tauri_plugin_dialog::DialogExt;

use crate::luna::resolve_luna_binary;
use crate::models::ExportResult;
use crate::utils::sanitize_filename;
use crate::xml::build_problem_xml;

#[tauri::command]
pub async fn export_tns(
    app: tauri::AppHandle,
    content: String,
    filename: String,
) -> Result<ExportResult, String> {
    let luna_bin = resolve_luna_binary(&app)?;

    let tmp_dir = std::env::temp_dir().join(format!(
        "tns_export_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    ));
    fs::create_dir_all(&tmp_dir).map_err(|e| format!("Failed to create temp dir: {e}"))?;

    let prob_xml_path = tmp_dir.join("Problem1.xml");
    let tns_tmp_path = tmp_dir.join("output.tns");

    let xml_content = build_problem_xml(&content, false, "rgb(0,0,0)");
    fs::write(&prob_xml_path, xml_content)
        .map_err(|e| format!("Failed to write Problem1.xml: {e}"))?;

    let output = Command::new(&luna_bin)
        .arg("Problem1.xml")
        .arg("output.tns")
        .current_dir(&tmp_dir)
        .output()
        .map_err(|e| format!("Failed to launch luna: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let _ = fs::remove_dir_all(&tmp_dir);
        return Err(format!(
            "luna failed (exit {:?})\nstdout: {stdout}\nstderr: {stderr}",
            output.status.code()
        ));
    }

    let tns_bytes = fs::read(&tns_tmp_path).map_err(|e| {
        let _ = fs::remove_dir_all(&tmp_dir);
        format!("Failed to read luna output: {e}")
    })?;

    let suggested = format!("{}.tns", sanitize_filename(&filename));
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
        None => {
            let _ = fs::remove_dir_all(&tmp_dir);
            return Err("Save cancelled by user.".to_string());
        }
    };

    let mut file = fs::File::create(&destination)
        .map_err(|e| format!("Failed to create destination file: {e}"))?;
    file.write_all(&tns_bytes)
        .map_err(|e| format!("Failed to write .tns bytes: {e}"))?;

    let _ = fs::remove_dir_all(&tmp_dir);

    Ok(ExportResult {
        saved_path: destination.to_string_lossy().to_string(),
    })
}

#[derive(serde::Serialize)]
pub struct CalculatorInfo {
    name: String,
    usb_id: String,
}

#[tauri::command]
pub async fn list_calculators() -> Result<Vec<CalculatorInfo>, String> {
    Ok(vec![])
}

#[tauri::command]
pub async fn send_to_calculator(
    _file_path: String,
    _usb_id: String,
    _remote_path: String,
) -> Result<String, String> {
    Err("Calculator transfer not yet implemented.".to_string())
}