// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::{self, File}, io::{Read, Write}, path::{Path, PathBuf}, process::{Command, Stdio}};

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
    // Log the name of the uploaded file
    let file_name = match path.file_stem() {
        Some(name) => name.to_string_lossy().into_owned(),
        None => return Err("Failed to get file name.".to_string()),
    };

    // Check if the file has an MP4 extension
    if let Some(extension) = path.extension() {
        if extension.to_string_lossy().to_lowercase() != "mp4" {
            return Err("Only MP4 files are allowed.".to_string());
        }
    } else {
        return Err("File does not have an extension.".to_string());
    }

    // Get the parent directory of the uploaded file
    let parent_dir = match path.parent() {
        Some(parent) => parent,
        None => return Err("Failed to get parent directory.".to_string()),
    };

    // Attempt to open the file
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return Err("Failed to open file.".to_string()),
    };

    // Read the file contents into a vector of bytes
    let mut mp4_data = Vec::new();
    if let Err(_) = file.read_to_end(&mut mp4_data) {
        return Err("Failed to read file contents.".to_string());
    }

    // Perform MP4 to GIF conversion
    let mut index = 1;
    let mut gif_path = parent_dir.join(format!("{}.gif", file_name));
    while gif_path.exists() {
        gif_path = parent_dir.join(format!("{}_{}.gif", file_name, index));
        index += 1;
    }

    let mut convert_command = Command::new("ffmpeg")
        .args(&["-i", "-", "-vf", "scale=320:-1", "-r", "10", &gif_path.to_string_lossy()])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Failed to start ffmpeg process: {}", e))?;

    if let Some(ref mut stdin) = convert_command.stdin {
        if let Err(_) = stdin.write_all(&mp4_data) {
            return Err("Failed to write MP4 data to ffmpeg process.".to_string());
        }
    } else {
        return Err("Failed to get stdin for ffmpeg process.".to_string());
    }

    // Wait for ffmpeg process to complete
    let status = convert_command.wait().map_err(|e| format!("ffmpeg process failed: {}", e))?;
    if !status.success() {
        return Err("Failed to convert MP4 to GIF.".to_string());
    }

    Ok(format!("File uploaded and converted successfully. GIF created in: {}", gif_path.display()))
}

