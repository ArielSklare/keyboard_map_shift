#![cfg(target_os = "linux")]
use xkbcommon::xkb::{CONTEXT_NO_FLAGS, Context, KEYMAP_COMPILE_NO_FLAGS, Keymap, State};

use quick_xml::de::from_str;
use serde::Deserialize;
use std::process::Command;

use crate::keyboard_mapping::types::{KeyboardDirection, KeyboardLayout, LayoutMap};
use std::collections::HashMap;
use std::fs;

// ===== Linux (helpers + public) =====
#[derive(Debug, Deserialize)]
struct XkbConfigRegistry {
    #[serde(rename = "layoutList")]
    layout_list: LayoutList,
}

#[derive(Debug, Deserialize)]
struct LayoutList {
    #[serde(rename = "layout", default)]
    layouts: Vec<Layout>,
}

#[derive(Debug, Deserialize)]
struct Layout {
    #[serde(rename = "configItem")]
    config_item: ConfigItem,
}

#[derive(Debug, Deserialize)]
struct ConfigItem {
    name: String,
}

fn is_rtl_char(c: char) -> bool {
    let u = c as u32;
    (0x0590..=0x08FF).contains(&u)
        || (0xFB1D..=0xFDFF).contains(&u)
        || (0xFE70..=0xFEFF).contains(&u)
}
fn get_registry_from_xml() -> Result<XkbConfigRegistry, quick_xml::DeError> {
    let xml_data =
        fs::read_to_string("/usr/share/X11/xkb/rules/evdev.xml").unwrap_or(String::from(""));

    let registry: XkbConfigRegistry = from_str(&xml_data)?;
    Ok(registry)
}

fn get_locale_layout_and_variant_strs(registry: XkbConfigRegistry) -> String {
    let output = Command::new("locale")
        .arg("-a")
        .output()
        .expect("Failed to run localectl");
    let installed_locales_str: String = String::from_utf8_lossy(&output.stdout).to_string();
    let installed_locales: Vec<String> = installed_locales_str
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    let active_countries: Vec<String> = installed_locales
        .iter()
        .filter_map(|loc| {
            if loc.starts_with("C") || loc.starts_with("POSIX") {
                None
            } else {
                // Split at _ and take the country (if present)
                let parts: Vec<&str> = loc.split(&['_', '.'][..]).collect();
                if parts.len() >= 2 {
                    Some(parts[1].to_lowercase()) // "US" -> "us"
                } else {
                    None
                }
            }
        })
        .collect();
    let filtered_layouts: Vec<&Layout> = registry
        .layout_list
        .layouts
        .iter()
        .filter(|layout| active_countries.contains(&layout.config_item.name.to_lowercase()))
        .collect();

    let layout_string: String = filtered_layouts
        .iter()
        .map(|layout| layout.config_item.name.clone())
        .collect::<Vec<_>>()
        .join(", ");
    layout_string
}

fn get_keymap() -> Result<Option<Keymap>, quick_xml::DeError> {
    let registry: XkbConfigRegistry = get_registry_from_xml()?;
    let layout_str = get_locale_layout_and_variant_strs(registry);
    let context = Context::new(CONTEXT_NO_FLAGS);
    let keymap = Keymap::new_from_names(
        &context,
        "",
        "",
        &layout_str,
        "",
        None::<String>,
        KEYMAP_COMPILE_NO_FLAGS,
    );
    Ok(keymap)
}
pub fn get_layout(index: u32) -> Option<KeyboardLayout> {
    let keymap = get_keymap().expect("failed to get key map")?;
    if index >= keymap.num_layouts() {
        return None;
    }

    let name = keymap.layout_get_name(index).to_string();
    let lang_name = if name.is_empty() {
        index.to_string()
    } else {
        name.clone()
    };

    let mut state = State::new(&keymap);
    state.update_mask(0, 0, 0, index, 0, 0);
    let mut dir = KeyboardDirection::LTR;
    for keycode in 8u16..=255u16 {
        let s = state.key_get_utf8(keycode.into());
        if let Some(first) = s.chars().next() {
            if is_rtl_char(first) {
                dir = KeyboardDirection::RTL;
                break;
            }
        }
    }
    Some(KeyboardLayout {
        lang_name,
        direction: dir,
    })
}

pub fn list_layouts() -> Vec<KeyboardLayout> {
    let Some(keymap) = get_keymap().expect("failed to get key map") else {
        return vec![KeyboardLayout {
            lang_name: String::from("current"),
            direction: KeyboardDirection::LTR,
        }];
    };
    let mut result = Vec::new();
    for i in 0..keymap.num_layouts() {
        if let Some(l) = get_layout(i) {
            result.push(l);
        }
    }
    if result.is_empty() {
        result.push(KeyboardLayout {
            lang_name: String::from("current"),
            direction: KeyboardDirection::LTR,
        });
    }
    result
}

pub fn vk_to_char_map_for_layout(layout_index: u32) -> LayoutMap {
    let layout = get_layout(layout_index).unwrap_or(KeyboardLayout {
        lang_name: layout_index.to_string(),
        direction: KeyboardDirection::LTR,
    });
    let Some(keymap) = get_keymap().expect("failed to get key map") else {
        return LayoutMap {
            layout,
            map: HashMap::new(),
        };
    };
    let mut state = State::new(&keymap);
    state.update_mask(0, 0, 0, layout_index, 0, 0);
    let mut map: HashMap<u16, String> = HashMap::new();
    for keycode in 8u16..=255u16 {
        let s = state.key_get_utf8(keycode.into());
        if !s.is_empty() {
            map.entry(keycode).or_insert(s);
        }
    }
    LayoutMap { layout, map }
}

pub fn vk_to_char_map_default() -> LayoutMap {
    vk_to_char_map_for_layout(0)
}

pub fn all_layout_vk_maps() -> Vec<LayoutMap> {
    let total = list_layouts().len() as u32;
    (0..total).map(|i| vk_to_char_map_for_layout(i)).collect()
}
