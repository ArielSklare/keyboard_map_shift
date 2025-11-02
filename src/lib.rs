pub mod config;
pub mod get_highlighted;
pub mod hotkey;
pub mod keyboard_mapping;
pub mod platform;

pub use get_highlighted::{get_highlighted_text, replace_highlighted_text};
pub use keyboard_mapping::{
    all_layout_vk_maps, get_layout, get_text_leyaout_map, list_layouts, shift_text_language,
    vk_to_char_map_default, vk_to_char_map_for_layout,
};

pub fn shift_highlighted_text_to_next_layout() -> Result<(), String> {
    let highlighted_text = match get_highlighted_text() {
        Some(text) => text,
        None => return Err("No text is currently highlighted".to_string()),
    };

    let layout_maps = all_layout_vk_maps();

    let current_layout_map = match get_text_leyaout_map(&highlighted_text, &layout_maps) {
        Some(layout) => layout,
        None => return Err("Could not determine the layout of the highlighted text".to_string()),
    };

    let shifted_text = layout_maps
        .iter()
        .cycle()
        .skip_while(|layout| layout.layout.lang_name != current_layout_map.layout.lang_name)
        .skip(1)
        .next()
        .map(|next_layout| shift_text_language(&highlighted_text, current_layout_map, next_layout))
        .ok_or_else(|| "No next layout found".to_string())?;

    replace_highlighted_text(&shifted_text)?;

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HotkeySpec {
    pub display: String,
}

impl HotkeySpec {
    pub fn from_display(value: &str) -> Result<Self, String> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err("Hotkey cannot be empty".to_string());
        }
        Ok(HotkeySpec {
            display: hotkey::normalize_display(trimmed),
        })
    }
}

pub fn update_hotkey(hotkey: &HotkeySpec) -> Result<(), String> {
    let mut cfg = config::load_config()?;
    cfg.hotkey = hotkey.display.clone();
    config::save_config(&cfg)?;
    let binder = crate::platform::get_binder();
    binder.apply_hotkey(&cfg.hotkey)
}

pub fn run_transform_once() -> Result<(), String> {
    shift_highlighted_text_to_next_layout()
}
