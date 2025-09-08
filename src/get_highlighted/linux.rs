#![cfg(target_os = "linux")]

use std::process::Command;

// Tries to get the currently highlighted/selected text on X11/Wayland environments
// Strategy:
// - Try `wl-paste -p` (primary selection) and fallback to `wl-paste` if on Wayland
// - If that fails, try `xclip -o -selection primary` then fallback to `xclip -o`
// - If that fails, try `xsel -o` then fallback to `xsel -o -b`
pub fn get_highlighted_text() -> Option<String> {
    // If running in WSL, try Windows clipboard via powershell.exe
    if is_wsl() {
        if let Ok(out) = Command::new("powershell.exe")
            .args(["-NoProfile", "-Command", "Get-Clipboard"])
            .output()
        {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout).to_string();
                if !s.is_empty() {
                    return Some(s.replace("\r\n", "\n"));
                }
            }
        }
    }
    // Wayland primary selection
    if let Ok(out) = Command::new("wl-paste").arg("-p").output() {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout).to_string();
            if !s.is_empty() {
                return Some(s);
            }
        }
    }

    // Wayland clipboard
    if let Ok(out) = Command::new("wl-paste").output() {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout).to_string();
            if !s.is_empty() {
                return Some(s);
            }
        }
    }

    // X11 primary selection via xclip
    if let Ok(out) = Command::new("xclip")
        .args(["-o", "-selection", "primary"])
        .output()
    {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout).to_string();
            if !s.is_empty() {
                return Some(s);
            }
        }
    }

    // X11 clipboard via xclip
    if let Ok(out) = Command::new("xclip").args(["-o"]).output() {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout).to_string();
            if !s.is_empty() {
                return Some(s);
            }
        }
    }

    // X11 via xsel
    if let Ok(out) = Command::new("xsel").arg("-o").output() {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout).to_string();
            if !s.is_empty() {
                return Some(s);
            }
        }
    }

    if let Ok(out) = Command::new("xsel").args(["-o", "-b"]).output() {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout).to_string();
            if !s.is_empty() {
                return Some(s);
            }
        }
    }

    None
}

fn is_wsl() -> bool {
    std::env::var("WSL_INTEROP").is_ok()
        || std::env::var("WSL_DISTRO_NAME").is_ok()
        || std::fs::read_to_string("/proc/sys/kernel/osrelease")
            .map(|s| s.to_lowercase().contains("microsoft"))
            .unwrap_or(false)
}
