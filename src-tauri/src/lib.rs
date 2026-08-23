use std::{fs, path::{Path, PathBuf}, time::{SystemTime, UNIX_EPOCH}};

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde_json::{json, Value};
use tauri::AppHandle;

fn data_path(_app: &AppHandle) -> PathBuf {
    PathBuf::from(r"C:\Users\Admin\OneDrive\Documents\saveit.json")
}

fn files_path() -> PathBuf {
    PathBuf::from(r"C:\Users\Admin\OneDrive\Documents\SaveItFiles")
}

#[tauri::command]
fn load_data(app: AppHandle) -> Result<Value, String> {
    let path = data_path(&app);
    if !path.exists() {
        return Ok(json!({ "items": [], "folders": [] }));
    }

    let contents = fs::read_to_string(path).map_err(|error| error.to_string())?;
    serde_json::from_str(&contents).map_err(|error| format!("Invalid saveit.json: {error}"))
}

#[tauri::command]
fn save_data(app: AppHandle, data: Value) -> Result<(), String> {
    let path = data_path(&app);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }

    let contents = serde_json::to_string_pretty(&data).map_err(|error| error.to_string())?;
    let temporary_path = path.with_extension("json.tmp");
    fs::write(&temporary_path, contents).map_err(|error| error.to_string())?;
    fs::rename(temporary_path, path).map_err(|error| error.to_string())
}

#[tauri::command]
fn store_file(filename: String, contents: String) -> Result<Value, String> {
    let source_name = Path::new(&filename)
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or("file");
    let safe_name: String = source_name
        .chars()
        .map(|character| if character.is_ascii_alphanumeric() || ".-_ ()[]".contains(character) { character } else { '_' })
        .collect();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_nanos();
    let destination = files_path().join(format!("{timestamp}_{safe_name}"));
    let bytes = BASE64.decode(contents).map_err(|error| format!("Invalid file data: {error}"))?;

    fs::create_dir_all(files_path()).map_err(|error| error.to_string())?;
    fs::write(&destination, bytes).map_err(|error| error.to_string())?;
    Ok(json!({
        "path": destination.to_string_lossy(),
        "filename": source_name
    }))
}

#[tauri::command]
fn open_file(path: String) -> Result<(), String> {
    let requested = PathBuf::from(&path);
    let root = files_path();
    let requested = requested.canonicalize().map_err(|error| error.to_string())?;
    let root = root.canonicalize().map_err(|error| error.to_string())?;
    if !requested.starts_with(&root) {
        return Err("Only files stored by SaveIt can be opened".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", requested.to_string_lossy().as_ref()])
            .spawn()
            .map_err(|error| error.to_string())?;
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("Opening saved files is currently supported on Windows".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![load_data, save_data, store_file, open_file])
        .run(tauri::generate_context!())
        .expect("error while running SaveIt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_data_has_expected_shape() {
        let data = json!({ "items": [], "folders": [] });
        assert_eq!(data["items"].as_array().map(Vec::len), Some(0));
        assert_eq!(data["folders"].as_array().map(Vec::len), Some(0));
    }
}
