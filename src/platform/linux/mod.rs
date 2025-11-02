mod gnome;
mod kde;

use crate::platform::HotkeyBinder;

pub struct LinuxBinder;

impl LinuxBinder {
    pub fn new() -> Self {
        LinuxBinder
    }
}

impl HotkeyBinder for LinuxBinder {
    fn apply_hotkey(&self, display: &str) -> Result<(), String> {
        match detect_de() {
            DesktopEnvironment::Gnome => gnome::apply_gnome_binding(display),
            DesktopEnvironment::Kde => kde::apply_kde_binding(display),
            DesktopEnvironment::Unknown => {
                Err("Unsupported Linux desktop environment for automatic binding".to_string())
            }
        }
    }

    fn remove_hotkey(&self) -> Result<(), String> {
        Ok(())
    }
}

enum DesktopEnvironment {
    Gnome,
    Kde,
    Unknown,
}

fn detect_de() -> DesktopEnvironment {
    let val = std::env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_default()
        .to_ascii_lowercase();
    if val.contains("gnome") {
        return DesktopEnvironment::Gnome;
    }
    if val.contains("kde") || val.contains("plasma") {
        return DesktopEnvironment::Kde;
    }
    let val2 = std::env::var("DESKTOP_SESSION")
        .unwrap_or_default()
        .to_ascii_lowercase();
    if val2.contains("gnome") {
        return DesktopEnvironment::Gnome;
    }
    if val2.contains("kde") || val2.contains("plasma") {
        return DesktopEnvironment::Kde;
    }
    DesktopEnvironment::Unknown
}

#[cfg(all(test, target_os = "linux"))]
mod tests;
