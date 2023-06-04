#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn get_legal_moves() {}

fn main() {
  tauri::Builder::default().invoke_handler(tauri::generate_handler![
    get_legal_moves
    ]).run(tauri::generate_context!()).expect("error while running tauri application");
}