#[cfg(target_os = "windows")]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyboardLayout, GetKeyboardLayoutList, HKL, MAPVK_VK_TO_VSC_EX, MapVirtualKeyExW,
    ToUnicodeEx,
};

#[cfg(target_os = "linux")]
use quick_xml::de::from_str;
#[cfg(target_os = "linux")]
use serde::Deserialize;
#[cfg(target_os = "linux")]
use std::process::Command;
#[cfg(target_os = "linux")]
use xkbcommon::xkb::{CONTEXT_NO_FLAGS, Context, KEYMAP_COMPILE_NO_FLAGS, Keymap, State};

use std::fs;

use std::collections::HashMap;

// ===== Types (shared) =====
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardLayout {
    pub lang_name: String,
    pub direction: KeyboardDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardDirection {
    LTR,
    RTL,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutMap {
    pub layout: KeyboardLayout,
    pub map: HashMap<u16, String>,
}

// ===== Windows (helpers + public) =====
#[cfg(target_os = "windows")]
fn windows_langid_is_rtl(langid: u16) -> bool {
    let primary = langid & 0x03FF;
    matches!(
        primary,
        0x01 | 0x0D | 0x29 | 0x20 | 0x5A | 0x65 | 0x63 | 0x3D | 0x92
    )
}

#[cfg(target_os = "windows")]
fn enumerate_hkls() -> Vec<HKL> {
    unsafe {
        let count = GetKeyboardLayoutList(None);
        let mut layouts_hkl = vec![HKL(std::ptr::null_mut()); count as usize];
        let _ = GetKeyboardLayoutList(Some(&mut layouts_hkl[..]));
        layouts_hkl
    }
}

#[cfg(target_os = "windows")]
fn keyboard_layout_from_hkl(hkl: HKL, idx: usize) -> KeyboardLayout {
    let langid = (hkl.0 as usize & 0xFFFF) as u16;
    let direction = if windows_langid_is_rtl(langid) {
        KeyboardDirection::RTL
    } else {
        KeyboardDirection::LTR
    };
    KeyboardLayout {
        lang_name: idx.to_string(),
        direction,
    }
}

#[cfg(target_os = "windows")]
pub fn get_layout(index: usize) -> Option<KeyboardLayout> {
    let hkls = enumerate_hkls();
    hkls.get(index)
        .map(|&hkl| keyboard_layout_from_hkl(hkl, index))
}

#[cfg(target_os = "windows")]
pub fn list_layouts() -> Vec<KeyboardLayout> {
    let hkls = enumerate_hkls();
    hkls.into_iter()
        .enumerate()
        .map(|(i, h)| keyboard_layout_from_hkl(h, i))
        .collect()
}

#[cfg(target_os = "windows")]
pub fn vk_to_char_map_for_layout(hkl: HKL) -> LayoutMap {
    let hkls = enumerate_hkls();
    let idx = hkls.iter().position(|&x| x.0 == hkl.0).unwrap_or(0);
    let layout = keyboard_layout_from_hkl(hkl, idx);
    unsafe {
        let mut map: HashMap<u16, String> = HashMap::new();
        let state = [0u8; 256];
        let mut buf = [0u16; 8];
        for vk in 0u16..=255u16 {
            let sc = MapVirtualKeyExW(vk as u32, MAPVK_VK_TO_VSC_EX, Some(hkl)) as u32;
            if sc == 0 {
                continue;
            }
            let written = ToUnicodeEx(vk as u32, sc, &state, &mut buf, 0, Some(hkl));
            if written > 0 {
                let s = String::from_utf16_lossy(&buf[..written as usize]);
                map.entry(vk).or_insert(s);
            }
        }
        LayoutMap { layout, map }
    }
}

#[cfg(target_os = "windows")]
pub fn vk_to_char_map_default() -> LayoutMap {
    unsafe { vk_to_char_map_for_layout(GetKeyboardLayout(0)) }
}

#[cfg(target_os = "windows")]
pub fn all_layout_vk_maps() -> Vec<LayoutMap> {
    let hkls = enumerate_hkls();
    hkls.into_iter()
        .map(|hkl| vk_to_char_map_for_layout(hkl))
        .collect()
}

// ===== Linux (helpers + public) =====
#[cfg(target_os = "linux")]
#[derive(Debug, Deserialize)]
struct XkbConfigRegistry {
    #[serde(rename = "layoutList")]
    layout_list: LayoutList,
}

#[cfg(target_os = "linux")]
#[derive(Debug, Deserialize)]
struct LayoutList {
    #[serde(rename = "layout", default)]
    layouts: Vec<Layout>,
}

#[cfg(target_os = "linux")]
#[derive(Debug, Deserialize)]
struct Layout {
    #[serde(rename = "configItem")]
    config_item: ConfigItem,
}

#[cfg(target_os = "linux")]
#[derive(Debug, Deserialize)]
struct ConfigItem {
    name: String,
}

#[cfg(target_os = "linux")]
fn is_rtl_char(c: char) -> bool {
    let u = c as u32;
    (0x0590..=0x08FF).contains(&u)
        || (0xFB1D..=0xFDFF).contains(&u)
        || (0xFE70..=0xFEFF).contains(&u)
}
#[cfg(target_os = "linux")]
fn get_registry_from_xml() -> Result<XkbConfigRegistry, quick_xml::DeError> {
    let xml_data =
        fs::read_to_string("/usr/share/X11/xkb/rules/evdev.xml").unwrap_or(String::from(""));

    let registry: XkbConfigRegistry = from_str(&xml_data)?;
    Ok(registry)
}

#[cfg(target_os = "linux")]
fn get_locale_layout_and_variant_strs(registry: XkbConfigRegistry) -> String{
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

#[cfg(target_os = "linux")]
fn get_keymap() -> Result<Option<Keymap>, quick_xml::DeError> {
    let registry: XkbConfigRegistry = get_registry_from_xml()?;
    let layout_str  = get_locale_layout_and_variant_strs(registry);
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
#[cfg(target_os = "linux")]
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

#[cfg(target_os = "linux")]
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

#[cfg(target_os = "linux")]
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

#[cfg(target_os = "linux")]
pub fn vk_to_char_map_default() -> LayoutMap {
    vk_to_char_map_for_layout(0)
}

#[cfg(target_os = "linux")]
pub fn all_layout_vk_maps() -> Vec<LayoutMap> {
    let total = list_layouts().len() as u32;
    (0..total).map(|i| vk_to_char_map_for_layout(i)).collect()
}

// ===== Fallbacks (not implemented) =====
#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn get_layout(_index: usize) -> Option<KeyboardLayout> {
    None
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn list_layouts() -> Vec<KeyboardLayout> {
    panic!("keyboard_mapping: list_layouts is not implemented for this OS");
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn vk_to_char_map_for_layout(_index: u32) -> LayoutMap {
    panic!("keyboard_mapping: vk_to_char_map_for_layout is not implemented for this OS");
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn vk_to_char_map_default() -> LayoutMap {
    panic!("keyboard_mapping: vk_to_char_map_default is not implemented for this OS");
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn all_layout_vk_maps() -> Vec<LayoutMap> {
    panic!("keyboard_mapping: all_layout_vk_maps is not implemented for this OS");
}
