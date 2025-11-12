#![cfg(target_os = "windows")]

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use windows::Win32::System::Com::IPersistFile;
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx,
    CoUninitialize,
};
use windows::Win32::UI::Shell::{IShellLinkW, ShellLink};
use windows::core::{Interface, PCWSTR};

pub fn set_hotkey(hotkey: &str) -> Result<(), String> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED)
            .ok()
            .map_err(|e| format!("COM init failed: {}", e))?;
    }
    
    let res = (|| -> Result<(), String> {
        // Get the actual executable path
        // When run via cargo run, current_exe() returns cargo/rustc, so we need to find the actual exe
        let exe = get_executable_path()?;
        let shortcut_path = shortcut_path()?;

        let shell_link: IShellLinkW = unsafe {
            CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)
                .map_err(|e| format!("Create ShellLink failed: {}", e))?
        };

        set_link_path(&shell_link, &exe)?;
        set_link_arguments(&shell_link, "run")?;
        
        // Set working directory to the executable's directory
        if let Some(work_dir) = exe.parent() {
            set_working_directory(&shell_link, work_dir)?;
        }

        let hotkey_word = parse_hotkey(hotkey)?;
        unsafe { shell_link.SetHotkey(hotkey_word) }
            .map_err(|e| format!("SetHotkey failed: {}", e))?;

        let persist: IPersistFile = shell_link
            .cast()
            .map_err(|e| format!("Persist cast: {}", e))?;
        let wide = to_wide_null(shortcut_path);
        unsafe { persist.Save(PCWSTR(wide.as_ptr()), true) }
            .map_err(|e| format!("Save .lnk failed: {}", e))?;

        Ok(())
    })();
    
    unsafe { CoUninitialize() };
    res
}

fn shortcut_path() -> Result<PathBuf, String> {
    let appdata = std::env::var("APPDATA").map_err(|e| format!("APPDATA not set: {}", e))?;
    let mut p = PathBuf::from(appdata);
    p.push("Microsoft\\Windows\\Start Menu\\Programs");
    std::fs::create_dir_all(&p).map_err(|e| format!("Create Start Menu dir failed: {}", e))?;
    p.push("Keyboard Map Shift.lnk");
    Ok(p)
}

fn set_link_path(link: &IShellLinkW, exe: &PathBuf) -> Result<(), String> {
    let wide = to_wide_null(exe);
    unsafe { link.SetPath(PCWSTR(wide.as_ptr())) }.map_err(|e| format!("SetPath failed: {}", e))
}

fn set_link_arguments(link: &IShellLinkW, args: &str) -> Result<(), String> {
    let wide: Vec<u16> = OsStr::new(args)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    unsafe { link.SetArguments(PCWSTR(wide.as_ptr())) }
        .map_err(|e| format!("SetArguments failed: {}", e))
}

fn set_working_directory(link: &IShellLinkW, dir: &std::path::Path) -> Result<(), String> {
    let wide = to_wide_null(dir);
    unsafe { link.SetWorkingDirectory(PCWSTR(wide.as_ptr())) }
        .map_err(|e| format!("SetWorkingDirectory failed: {}", e))
}

fn get_executable_path() -> Result<PathBuf, String> {
    let current_exe = std::env::current_exe().map_err(|e| format!("Exe path: {}", e))?;
    
    // If running via cargo run, current_exe() points to cargo/rustc
    // Try to find the actual built executable in target/debug or target/release
    let exe_name = current_exe.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("keyboard_map_shift.exe");
    
    // Check if current_exe is actually our executable
    if exe_name == "keyboard_map_shift.exe" || exe_name == "keyboard_map_shift" {
        return Ok(current_exe);
    }
    
    // Otherwise, try to find it in target directories
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .or_else(|_| std::env::current_dir().map_err(|_| "Cannot determine project directory".to_string()))
        .map_err(|_| "Cannot determine project directory".to_string())?;
    
    // Try target/debug first (for development)
    let debug_path = manifest_dir.join("target").join("debug").join("keyboard_map_shift.exe");
    if debug_path.exists() {
        return Ok(debug_path);
    }
    
    // Try target/release (for release builds)
    let release_path = manifest_dir.join("target").join("release").join("keyboard_map_shift.exe");
    if release_path.exists() {
        return Ok(release_path);
    }
    
    // Fallback to current_exe (might work in some cases)
    Ok(current_exe)
}

fn to_wide_null(path: impl AsRef<OsStr>) -> Vec<u16> {
    let s = path.as_ref();
    s.encode_wide().chain(std::iter::once(0)).collect()
}

fn parse_hotkey(hotkey: &str) -> Result<u16, String> {
    // Windows hotkey format for IShellLink::SetHotkey:
    // Low byte (bits 0-7): Virtual key code
    // High byte (bits 8-15): Modifiers (MOD_ALT=0x01, MOD_CONTROL=0x02, MOD_SHIFT=0x04)
    let parts: Vec<&str> = hotkey.split('+').map(|p| p.trim()).collect();
    
    let mut modifiers = 0u8;
    let mut key: Option<char> = None;
    
    for part in parts {
        let p = part.to_ascii_lowercase();
        match p.as_str() {
            "ctrl" | "control" => modifiers |= 0x02, // MOD_CONTROL
            "alt" => modifiers |= 0x01,              // MOD_ALT
            "shift" => modifiers |= 0x04,            // MOD_SHIFT
            k if k.len() == 1 => {
                key = Some(k.chars().next().unwrap().to_ascii_uppercase());
            }
            _ => return Err(format!("Unknown hotkey part: {}", part)),
        }
    }
    
    let vk_char = key.ok_or_else(|| "Missing key in hotkey".to_string())?;
    
    // Convert character to virtual key code
    // For letters A-Z, the VK code is the same as the ASCII uppercase value
    let vk_code = if vk_char.is_ascii_alphabetic() {
        vk_char as u8
    } else {
        // For other keys, we'd need a mapping table, but for now just use the char value
        vk_char as u8
    };
    
    // Windows hotkey word format: (modifiers << 8) | vk_code
    let hotkey_word = ((modifiers as u16) << 8) | (vk_code as u16);
    
    // Debug output to verify
    eprintln!("[DEBUG] Parsed hotkey '{}': modifiers=0x{:02x}, vk='{}' (0x{:02x}), word=0x{:04x}", 
        hotkey, modifiers, vk_char, vk_code, hotkey_word);
    
    Ok(hotkey_word)
}

