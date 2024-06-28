// src-tauri/src/main.rs
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use std::process::Command;

#[tauri::command]
fn launch_calculator() {
    #[cfg(target_os = "windows")]
    {
        Command::new("calc.exe")
            .spawn()
            .expect("Failed to launch calculator");
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg("-a")
            .arg("Calculator")
            .spawn()
            .expect("Failed to launch calculator");
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![launch_calculator])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
