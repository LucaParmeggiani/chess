// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_hello_world() -> String {
  "Hello, world from Rust!".to_owned()
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_hello_world])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}