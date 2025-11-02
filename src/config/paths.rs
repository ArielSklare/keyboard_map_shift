use directories::ProjectDirs;
use std::path::PathBuf;

pub fn config_file_path() -> Result<PathBuf, String> {
    let dirs = ProjectDirs::from("com", "keyboard-map-shift", "keyboard_map_shift")
        .ok_or_else(|| "Could not determine configuration directory".to_string())?;
    let dir = dirs.config_dir();
    std::fs::create_dir_all(dir)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;
    let mut path = PathBuf::from(dir);
    path.push("config.toml");
    Ok(path)
}
