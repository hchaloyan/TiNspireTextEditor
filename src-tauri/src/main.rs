#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod luna;
mod models;
mod utils;
mod xml;

#[cfg(test)]
mod tests;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::export_tns,
            commands::list_calculators,
            commands::send_to_calculator,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}