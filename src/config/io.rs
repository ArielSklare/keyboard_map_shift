use super::{Config, config_file_path};

pub fn load_config() -> Result<Config, String> {
    let path = config_file_path()?;
    if !path.exists() {
        return Ok(Config::with_defaults());
    }
    let data =
        std::fs::read_to_string(&path).map_err(|e| format!("Failed to read config: {}", e))?;
    let cfg: Config =
        toml::from_str(&data).map_err(|e| format!("Failed to parse config: {}", e))?;
    Ok(cfg)
}

pub fn save_config(cfg: &Config) -> Result<(), String> {
    let path = config_file_path()?;
    let toml_str =
        toml::to_string_pretty(cfg).map_err(|e| format!("Failed to serialize config: {}", e))?;
    std::fs::write(&path, toml_str).map_err(|e| format!("Failed to write config: {}", e))
}
