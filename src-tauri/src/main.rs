// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::File, io::Read, path::PathBuf};

use tauri::Manager;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
        #[cfg(debug_assertions)]
        {
            let window = app.get_window("main").unwrap();
            window.open_devtools();
            window.close_devtools();
        }
        Ok(())
    })
    .invoke_handler(tauri::generate_handler![upload_file])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]

fn upload_file(path: PathBuf) -> Result<String, String> {
    let file_name = match path.file_name() {
        Some(name) => name.to_string_lossy().into_owned(),
        None => return Err("Failed to get file name.".to_string()),
    };
    println!("Uploaded file name: {}", file_name);
    // Check if the file has an MP4 extension
    if let Some(extension) = path.extension() {
        if extension.to_string_lossy().to_lowercase() != "mp4" {
            return Err("Only MP4 files are allowed.".to_string());
        }
    } else {
        return Err("File does not have an extension.".to_string());
    }

    // Attempt to open the file
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return Err("Failed to open file.".to_string()),
    };

    // Read the file contents into a string
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(_) => Err("Failed to read file contents.".to_string()),
    }
}

