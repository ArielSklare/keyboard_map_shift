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

pub fn replace_highlighted_text(new_text: &str) -> Result<(), String> {
    // Avoid clipboard. Type text via input synthesis tools.
    // Wayland: use wtype if present
    if Command::new("wtype")
        .args(["--"])
        .status()
        .map(|_| true)
        .unwrap_or(false)
    {
        // wtype types args after "--" literally
        let status = Command::new("wtype")
            .arg("--")
            .arg(new_text)
            .status()
            .map_err(|e| e.to_string())?;
        return if status.success() {
            Ok(())
        } else {
            Err("wtype failed".to_string())
        };
    }

    // X11: xdotool type --clearmodifiers
    if Command::new("xdotool")
        .arg("version")
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        let status = Command::new("xdotool")
            .args(["type", "--clearmodifiers", "--"])
            .arg(new_text)
            .status()
            .map_err(|e| e.to_string())?;
        return if status.success() {
            Ok(())
        } else {
            Err("xdotool failed".to_string())
        };
    }

    // WSL: no universal keystroke injector; return error to avoid clipboard hacks
    if is_wsl() {
        return Err("WSL typing not supported without GUI input tool (wtype/xdotool)".to_string());
    }

    Err("no typing tool available (wtype or xdotool)".to_string())
}
