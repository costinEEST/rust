
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


use serde::Deserialize;
#[derive(Deserialize)]
enum JobStatus {
    Programmer(Vec<String>),
    Doctor(Vec<String>),
    Dancer(Vec<String>),
}

#[tauri::command]
fn skills(job: JobStatus) -> Vec<String> {
    match job {
        JobStatus::Programmer(_) => vec!["Perseverance".to_string(), "Analytical thinking".to_string(), "Self taught".to_string()],
        JobStatus::Doctor(_) => vec!["Kindness".to_string(), "Resilience".to_string()],    
        JobStatus::Dancer(_) => vec!["Self awareness".to_string(), "Perseverance".to_string(), "Wake up early".to_string()]    
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, skills])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
