pub mod constants;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub trait HotkeyBinder {
    fn apply_hotkey(&self, display: &str) -> Result<(), String>;

    fn remove_hotkey(&self) -> Result<(), String>;
}

pub fn get_binder() -> Box<dyn HotkeyBinder> {
    #[cfg(target_os = "windows")]
    {
        Box::new(windows::WindowsBinder::new())
    }

    #[cfg(target_os = "linux")]
    {
        Box::new(linux::LinuxBinder::new())
    }
}
