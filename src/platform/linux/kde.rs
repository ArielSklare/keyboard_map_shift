use crate::platform::constants::{APP_STRINGS, KDE_KEYS, KDE_PATHS, KDE_TEMPLATES};
use std::fs;
use std::path::PathBuf;

pub fn apply_kde_binding(display: &str) -> Result<(), String> {
    let home = std::env::var("HOME").map_err(|e| format!("HOME not set: {}", e))?;
    let home_path = PathBuf::from(home);
    let desktop_path = desktop_file_path_from_home(&home_path)?;
    let desktop_dir = desktop_path
        .parent()
        .ok_or_else(|| format!("Desktop path has no parent: {}", desktop_path.display()))?;
    fs::create_dir_all(desktop_dir)
        .map_err(|e| format!("Create applications dir failed: {}", e))?;
    let desktop_contents = desktop_file_contents();
    fs::write(&desktop_path, desktop_contents)
        .map_err(|e| format!("Write desktop file failed: {}", e))?;

    let kglobal = kglobalshortcuts_path_from_home(&home_path)?;
    let mut ini = load_ini(&kglobal);
    let group = KDE_PATHS.component_group.to_string();
    let entry_key = KDE_KEYS.friendly_name_key.to_string();
    ini_set(&mut ini, &group, &entry_key, APP_STRINGS.app_name);

    let trigger = display.to_string();
    ini_set(
        &mut ini,
        &group,
        KDE_KEYS.trigger_key,
        &format!("{},none,Trigger", trigger),
    );
    save_ini(&kglobal, &ini)?;

    let _ = std::process::Command::new("qdbus")
        .args([
            "org.kde.kglobalaccel",
            APP_STRINGS.kde_component_dbus_path,
            "reconfigure",
        ])
        .status();

    Ok(())
}

fn desktop_file_path_from_home(home: &PathBuf) -> Result<PathBuf, String> {
    let mut p = home.clone();
    p.push(KDE_PATHS.desktop_relative_path);
    Ok(p)
}

fn desktop_file_contents() -> String {
    KDE_TEMPLATES
        .desktop_entry_template
        .replace("{name}", APP_STRINGS.app_name)
        .replace("{exec}", APP_STRINGS.exec_run_cmd)
}

fn kglobalshortcuts_path_from_home(home: &PathBuf) -> Result<PathBuf, String> {
    let mut p = home.clone();
    p.push(KDE_PATHS.kglobalshortcuts_relative_path);
    Ok(p)
}

type Ini = std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>;

fn load_ini(path: &PathBuf) -> Ini {
    let mut ini: Ini = Ini::new();
    if let Ok(contents) = fs::read_to_string(path) {
        let mut current_group: Option<String> = None;
        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }
            if line.starts_with('[') && line.ends_with(']') {
                current_group = Some(line.trim_matches(&['[', ']'][..]).to_string());
            } else if let Some(eq) = line.find('=') {
                if let Some(group) = &current_group {
                    let (k, v) = line.split_at(eq);
                    let k = k.trim().to_string();
                    let v = v[1..].trim().to_string();
                    ini.entry(group.clone()).or_default().insert(k, v);
                }
            }
        }
    }
    ini
}

fn ini_set(ini: &mut Ini, group: &str, key: &str, value: &str) {
    ini.entry(group.to_string())
        .or_default()
        .insert(key.to_string(), value.to_string());
}

fn save_ini(path: &PathBuf, ini: &Ini) -> Result<(), String> {
    let mut out = String::new();
    for (group, entries) in ini {
        out.push('[');
        out.push_str(group);
        out.push_str("]\n");
        for (k, v) in entries {
            out.push_str(k);
            out.push('=');
            out.push_str(v);
            out.push('\n');
        }
    }
    fs::write(path, out).map_err(|e| format!("Write kglobalshortcutsrc failed: {}", e))
}

#[cfg(all(test, target_os = "linux"))]
mod tests;
