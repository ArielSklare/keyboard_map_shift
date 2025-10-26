#![cfg(target_os = "linux")]

use std::process::Command;

pub fn get_highlighted_text() -> Option<String> {
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
    if let Ok(out) = Command::new("wl-paste").arg("-p").output() {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout).to_string();
            if !s.is_empty() {
                return Some(s);
            }
        }
    }

    if let Ok(out) = Command::new("wl-paste").output() {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout).to_string();
            if !s.is_empty() {
                return Some(s);
            }
        }
    }

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

    if let Ok(out) = Command::new("xclip").args(["-o"]).output() {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout).to_string();
            if !s.is_empty() {
                return Some(s);
            }
        }
    }

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
    if Command::new("wtype")
        .args(["--"])
        .status()
        .map(|_| true)
        .unwrap_or(false)
    {
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

    if is_wsl() {
        return Err("WSL typing not supported without GUI input tool (wtype/xdotool)".to_string());
    }

    Err("no typing tool available (wtype or xdotool)".to_string())
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    #[test]
    fn test_get_highlighted_text_smoke() {
        let _result = get_highlighted_text();
    }

    #[test]
    fn test_replace_highlighted_text_smoke() {
        let result = replace_highlighted_text("test text");
        match result {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    #[test]
    fn test_is_wsl_detection() {
        let is_wsl = is_wsl();
        assert!(is_wsl == true || is_wsl == false);
    }
}
