use std::fs;
use std::path::Path;

#[tauri::command]
fn read_directory(path: &str) -> Result<Vec<String>, String> {
    let entries = fs::read_dir(path).map_err(|e| e.to_string())?;

    let files: Vec<String> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_name = entry.file_name().to_string_lossy().to_string();
            // skip hidden files like .DS_Store
            if file_name.starts_with('.') {
                None
            } else {
                Some(file_name)
            }
        })
        .collect();

    Ok(files)
}

#[tauri::command]
fn rename_file(dir: &str, old_name: &str, new_name: &str) -> Result<(), String> {
    let old_path = Path::new(dir).join(old_name);
    let new_path = Path::new(dir).join(new_name);
    fs::rename(old_path, new_path).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![read_directory, rename_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
