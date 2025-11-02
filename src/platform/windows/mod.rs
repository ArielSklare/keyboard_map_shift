use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;

use windows::Win32::Foundation::{HWND, MAX_PATH};
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx,
    CoUninitialize,
};
use windows::Win32::System::Ole::IPersistFile;
use windows::Win32::UI::Shell::{IShellLinkW, ShellExecuteW, ShellLink};
use windows::core::Interface;

use crate::hotkey;
use crate::platform::HotkeyBinder;
use crate::platform::constants::{APP_STRINGS, WINDOWS_PATHS};

pub struct WindowsBinder;

impl WindowsBinder {
    pub fn new() -> Self {
        WindowsBinder
    }
}

impl HotkeyBinder for WindowsBinder {
    fn apply_hotkey(&self, display: &str) -> Result<(), String> {
        unsafe {
            CoInitializeEx(None, COINIT_APARTMENTTHREADED)
                .map_err(|e| format!("COM init failed: {}", e))?;
        }
        let res = (|| {
            let exe = std::env::current_exe().map_err(|e| format!("Exe path: {}", e))?;
            let shortcut_path = shortcut_path()?;

            let shell_link: IShellLinkW = unsafe {
                CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)
                    .map_err(|e| format!("Create ShellLink failed: {}", e))?
            };

            set_link_path(&shell_link, &exe)?;
            set_link_arguments(&shell_link, APP_STRINGS.app_run_subcommand)?;

            let hotkey_word = hotkey::to_windows_hotkey_word_from_display(display)?;
            unsafe { shell_link.SetHotkey(hotkey_word) }
                .map_err(|e| format!("SetHotkey failed: {}", e))?;

            let persist: IPersistFile = shell_link
                .cast()
                .map_err(|e| format!("Persist cast: {}", e))?;
            let wide = to_wide_null(shortcut_path);
            unsafe { persist.Save(wide.as_ptr(), true) }
                .map_err(|e| format!("Save .lnk failed: {}", e))?;

            Ok(())
        })();
        unsafe { CoUninitialize() };
        res
    }

    fn remove_hotkey(&self) -> Result<(), String> {
        let shortcut_path = shortcut_path()?;
        if shortcut_path.exists() {
            std::fs::remove_file(&shortcut_path)
                .map_err(|e| format!("Failed to remove shortcut: {}", e))?;
        }
        Ok(())
    }
}

fn shortcut_path() -> Result<PathBuf, String> {
    let appdata =
        std::env::var(WINDOWS_PATHS.env_appdata).map_err(|e| format!("APPDATA not set: {}", e))?;
    let mut p = PathBuf::from(appdata);
    p.push(WINDOWS_PATHS.start_menu_programs_rel);
    std::fs::create_dir_all(&p).map_err(|e| format!("Create Start Menu dir failed: {}", e))?;
    p.push(WINDOWS_PATHS.shortcut_filename);
    Ok(p)
}

fn set_link_path(link: &IShellLinkW, exe: &PathBuf) -> Result<(), String> {
    let wide = to_wide_null(exe);
    unsafe { link.SetPath(wide.as_ptr()) }.map_err(|e| format!("SetPath failed: {}", e))
}

fn set_link_arguments(link: &IShellLinkW, args: &str) -> Result<(), String> {
    let wide: Vec<u16> = OsStr::new(args)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    unsafe { link.SetArguments(wide.as_ptr()) }.map_err(|e| format!("SetArguments failed: {}", e))
}

fn to_wide_null(path: impl AsRef<OsStr>) -> Vec<u16> {
    let s = path.as_ref();
    s.encode_wide().chain(std::iter::once(0)).collect()
}
