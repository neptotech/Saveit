use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde_json::{json, Value};

fn default_storage_path() -> Result<PathBuf, String> {
    let profile =
        std::env::var_os("USERPROFILE").ok_or("Could not find the Windows user profile")?;
    let profile = PathBuf::from(profile);
    let onedrive_documents = profile.join("OneDrive").join("Documents");
    if onedrive_documents.is_dir() {
        Ok(onedrive_documents)
    } else {
        Ok(profile.join("Documents"))
    }
}

fn storage_root(path: &str) -> Result<PathBuf, String> {
    let root = if path.trim().is_empty() {
        default_storage_path()?
    } else {
        PathBuf::from(path)
    };
    if !root.is_absolute() {
        return Err("Storage path must be an absolute path".to_string());
    }
    Ok(root)
}

#[tauri::command]
fn get_default_storage_path() -> Result<String, String> {
    Ok(default_storage_path()?.to_string_lossy().into_owned())
}

#[tauri::command]
fn load_data(storage_path: String) -> Result<Value, String> {
    let path = storage_root(&storage_path)?.join("saveit.json");
    if !path.exists() {
        return Ok(json!({ "items": [], "folders": [] }));
    }

    let contents = fs::read_to_string(path).map_err(|error| error.to_string())?;
    serde_json::from_str(&contents).map_err(|error| format!("Invalid saveit.json: {error}"))
}

#[tauri::command]
fn save_data(storage_path: String, data: Value) -> Result<(), String> {
    let path = storage_root(&storage_path)?.join("saveit.json");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }

    let contents = serde_json::to_string_pretty(&data).map_err(|error| error.to_string())?;
    let temporary_path = path.with_extension("json.tmp");
    fs::write(&temporary_path, contents).map_err(|error| error.to_string())?;
    fs::rename(temporary_path, path).map_err(|error| error.to_string())
}

#[tauri::command]
fn store_file(storage_path: String, filename: String, contents: String) -> Result<Value, String> {
    let source_name = Path::new(&filename)
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or("file");
    let safe_name: String = source_name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || ".-_ ()[]".contains(character) {
                character
            } else {
                '_'
            }
        })
        .collect();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_nanos();
    let files_directory = storage_root(&storage_path)?.join("SaveItFiles");
    let destination = files_directory.join(format!("{timestamp}_{safe_name}"));
    let bytes = BASE64
        .decode(contents)
        .map_err(|error| format!("Invalid file data: {error}"))?;

    fs::create_dir_all(files_directory).map_err(|error| error.to_string())?;
    fs::write(&destination, bytes).map_err(|error| error.to_string())?;
    Ok(json!({
        "path": destination.to_string_lossy(),
        "filename": source_name
    }))
}

#[tauri::command]
fn open_file(storage_path: String, path: String) -> Result<(), String> {
    let requested = PathBuf::from(&path);
    let root = storage_root(&storage_path)?.join("SaveItFiles");
    let requested = requested
        .canonicalize()
        .map_err(|error| error.to_string())?;
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

fn copy_directory(source: &Path, destination: &Path) -> Result<(), String> {
    fs::create_dir_all(destination).map_err(|error| error.to_string())?;
    for entry in fs::read_dir(source).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let target = destination.join(entry.file_name());
        if entry
            .file_type()
            .map_err(|error| error.to_string())?
            .is_dir()
        {
            copy_directory(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target).map_err(|error| error.to_string())?;
        }
    }
    Ok(())
}

#[tauri::command]
fn migrate_storage(from_path: String, to_path: String) -> Result<(), String> {
    let source = storage_root(&from_path)?;
    let destination = storage_root(&to_path)?;
    if source == destination {
        return Err("Choose a different storage path".to_string());
    }
    if destination.exists() {
        let mut entries = fs::read_dir(&destination).map_err(|error| error.to_string())?;
        if entries.next().is_some() {
            return Err("The new storage path must not exist or must be empty".to_string());
        }
    } else {
        fs::create_dir_all(&destination).map_err(|error| error.to_string())?;
    }

    let source_data = source.join("saveit.json");
    if source_data.is_file() {
        fs::copy(source_data, destination.join("saveit.json"))
            .map_err(|error| error.to_string())?;
    }
    let source_files = source.join("SaveItFiles");
    if source_files.is_dir() {
        copy_directory(&source_files, &destination.join("SaveItFiles"))?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            load_data,
            save_data,
            store_file,
            open_file,
            get_default_storage_path,
            migrate_storage
        ])
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
