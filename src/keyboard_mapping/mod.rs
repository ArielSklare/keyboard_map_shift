#![allow(unused)]
pub mod fallback;
pub mod linux;
pub mod types;
pub mod windows;
pub mod remap;
// Re-export types for convenience
pub use types::{LayoutMap, KeyboardLayout, KeyboardDirection};

// Re-export platform-specific functions
#[cfg(target_os = "windows")]
pub use windows::{
    all_layout_vk_maps, get_layout, list_layouts, vk_to_char_map_default, vk_to_char_map_for_layout,
};

#[cfg(target_os = "linux")]
pub use linux::{
    all_layout_vk_maps, get_layout, list_layouts, vk_to_char_map_default, vk_to_char_map_for_layout,
};

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub use fallback::{
    all_layout_vk_maps, get_layout, list_layouts, vk_to_char_map_default, vk_to_char_map_for_layout,
};

pub use remap::{get_text_leyaout_map, shift_text_language};