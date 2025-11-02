use crate::hotkey;
use crate::platform::constants::{APP_STRINGS, GNOME_KEYS, GNOME_PATHS};
use std::process::Command;

pub fn apply_gnome_binding(display: &str) -> Result<(), String> {
    let base = GNOME_PATHS.custom_keybinding_base;
    let list_key = GNOME_PATHS.media_keys_schema;
    let binding_schema = GNOME_PATHS.custom_keybinding_schema;

    let current = gsettings_get(list_key, GNOME_KEYS.custom_keybindings_key)
        .unwrap_or_else(|_| String::from("[]"));
    if !current.contains("keyboard-map-shift") {
        let new_list = format!("['{}']", base);
        gsettings_set(list_key, GNOME_KEYS.custom_keybindings_key, &new_list)?;
    }

    gsettings_set_kb(
        binding_schema,
        base,
        GNOME_KEYS.kb_name_key,
        &format!("'{}'", APP_STRINGS.app_name),
    )?;
    gsettings_set_kb(
        binding_schema,
        base,
        GNOME_KEYS.kb_command_key,
        &format!("'{}'", APP_STRINGS.exec_run_cmd),
    )?;
    let binding = format!("'{}'", hotkey::to_gnome_binding_from_display(display)?);
    gsettings_set_kb(binding_schema, base, GNOME_KEYS.kb_binding_key, &binding)?;
    Ok(())
}

fn gsettings_set(schema: &str, key: &str, value: &str) -> Result<(), String> {
    let status = Command::new("gsettings")
        .args(["set", schema, key, value])
        .status()
        .map_err(|e| format!("Failed to execute gsettings: {}", e))?;
    if !status.success() {
        return Err(format!("gsettings set {} {} failed", schema, key));
    }
    Ok(())
}

fn gsettings_get(schema: &str, key: &str) -> Result<String, String> {
    let out = Command::new("gsettings")
        .args(["get", schema, key])
        .output()
        .map_err(|e| format!("Failed to execute gsettings: {}", e))?;
    if !out.status.success() {
        return Err(format!("gsettings get {} {} failed", schema, key));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn gsettings_set_kb(schema: &str, base_path: &str, key: &str, value: &str) -> Result<(), String> {
    let target = format!("{}:{}", schema, base_path);
    let status = Command::new("gsettings")
        .arg("set")
        .arg(&target)
        .arg(key)
        .arg(value)
        .status()
        .map_err(|e| format!("Failed to execute gsettings: {}", e))?;
    if !status.success() {
        return Err(format!("gsettings set {} {} failed", target, key));
    }
    Ok(())
}
