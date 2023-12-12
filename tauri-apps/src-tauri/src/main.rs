
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;
#[derive(Deserialize)]
enum JobStatus {
    Programmer,
    Doctor,
    Dancer,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn skills(job: JobStatus) -> Vec<String> {
    match job {
        JobStatus::Programmer => vec!["Coding".to_string(), "Debugging".to_string()],
        JobStatus::Doctor => vec!["Empathy".to_string(), "Medical Knowledge".to_string()],
        JobStatus::Dancer => vec!["Rhythm".to_string(), "Flexibility".to_string()]
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![skills])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
