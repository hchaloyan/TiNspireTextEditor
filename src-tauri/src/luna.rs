
use std::path::PathBuf;
use tauri::Manager;

pub fn resolve_luna_binary(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    if let Ok(p) = app.path().resolve("luna.exe", tauri::path::BaseDirectory::Resource) {
        if p.exists() {
            return Ok(p);
        }
    }

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
            return p.canonicalize().map_err(|e| e.to_string());
        }
    }

    Err(
        "luna binary not found. Place luna.exe at Luna-master/luna.exe (dev) or add it to tauri.conf.json resources."
            .to_string(),
    )
}