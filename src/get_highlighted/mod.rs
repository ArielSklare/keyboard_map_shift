#![allow(unused)]
pub mod fallback;
pub mod linux;
pub mod windows;

// Re-export platform-specific function
#[cfg(target_os = "windows")]
pub use windows::get_highlighted_text;

#[cfg(target_os = "linux")]
pub use linux::get_highlighted_text;

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub use fallback::get_highlighted_text;
