#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod models;
mod services;
mod utils;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            api::trainer::fetch_trainers,
            api::trainer::search_trainers,
            api::trainer::get_trainer_detail,
            api::trainer::download_trainer,
            api::trainer::delete_trainer,
            api::trainer::launch_trainer
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 