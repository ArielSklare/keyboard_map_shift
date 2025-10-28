#![cfg(target_os = "windows")]
use windows::Win32::{
    Globalization::{GetLocaleInfoEx, LCIDToLocaleName, LOCALE_SLANGUAGE},
    UI::Input::KeyboardAndMouse::{
        GetKeyboardLayout, GetKeyboardLayoutList, HKL, MAPVK_VK_TO_VSC_EX, MapVirtualKeyExW,
        ToUnicodeEx,
    },
};

use super::types::{KeyboardDirection, KeyboardLayout, LayoutMap};
use std::collections::HashMap;

// TODO: get meaningfull names for layouts
// ===== Windows (helpers + public) =====
#[cfg(target_os = "windows")]
fn windows_langid_is_rtl(langid: u16) -> bool {
    let primary = langid & 0x03FF;
    matches!(
        primary,
        0x01 | 0x0D | 0x29 | 0x20 | 0x5A | 0x65 | 0x63 | 0x3D | 0x92
    )
}

fn enumerate_hkls() -> Vec<HKL> {
    unsafe {
        let count = GetKeyboardLayoutList(None);
        let mut layouts_hkl = vec![HKL(std::ptr::null_mut()); count as usize];
        let _ = GetKeyboardLayoutList(Some(&mut layouts_hkl[..]));
        layouts_hkl
    }
}

fn lang_name_from_langid(langid: u16) -> String {
    unsafe {
        let lcid = langid as u32; // SORT_DEFAULT == 0, so LCID == LANGID

        // First, get BCP-47 locale name (e.g., "en-US") from LCID
        const LOCALE_NAME_MAX_LENGTH: usize = 85; // per Win32 API docs
        let mut locale_name_buf = [0u16; LOCALE_NAME_MAX_LENGTH];
        let len = LCIDToLocaleName(lcid, Some(&mut locale_name_buf), 0);

        if len <= 0 {
            return format!("0x{:04X}", langid);
        }

        // len includes the terminating NUL
        let bcp47 = String::from_utf16_lossy(&locale_name_buf[..(len as usize - 1)]);

        // Try to get the localized language display name for that locale
        // First call to get required size
        let needed = GetLocaleInfoEx(
            windows::core::PCWSTR(locale_name_buf.as_ptr()),
            LOCALE_SLANGUAGE,
            None,
        );

        if needed <= 0 {
            return bcp47;
        }

        let mut display_buf = vec![0u16; needed as usize];
        let written = GetLocaleInfoEx(
            windows::core::PCWSTR(locale_name_buf.as_ptr()),
            LOCALE_SLANGUAGE,
            Some(display_buf.as_mut_slice()),
        );

        if written <= 0 {
            return bcp47;
        }

        // written includes terminating NUL
        let name = String::from_utf16_lossy(&display_buf[..(written as usize - 1)]);
        if name.is_empty() {
            return bcp47;
        }
        name
    }
}

fn keyboard_layout_from_hkl(hkl: HKL) -> KeyboardLayout {
    let langid = (hkl.0 as usize & 0xFFFF) as u16;
    let direction = if windows_langid_is_rtl(langid) {
        KeyboardDirection::RTL
    } else {
        KeyboardDirection::LTR
    };
    KeyboardLayout {
        lang_name: lang_name_from_langid(langid),
        direction,
    }
}

pub fn get_layout(index: usize) -> Option<KeyboardLayout> {
    let hkls = enumerate_hkls();
    hkls.get(index).map(|&hkl| keyboard_layout_from_hkl(hkl))
}

pub fn list_layouts() -> Vec<KeyboardLayout> {
    let hkls = enumerate_hkls();
    hkls.into_iter()
        .map(|(h)| keyboard_layout_from_hkl(h))
        .collect()
}

pub fn vk_to_char_map_for_layout(hkl: HKL) -> LayoutMap {
    let hkls = enumerate_hkls();
    let layout = keyboard_layout_from_hkl(hkl);
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

pub fn vk_to_char_map_default() -> LayoutMap {
    unsafe { vk_to_char_map_for_layout(GetKeyboardLayout(0)) }
}
// TODO: decide if this should be a vec or a hash map with lang name as keys
pub fn all_layout_vk_maps() -> Vec<LayoutMap> {
    let hkls = enumerate_hkls();
    hkls.into_iter()
        .map(|hkl| vk_to_char_map_for_layout(hkl))
        .collect()
}

#[cfg(all(test, target_os = "windows"))]
mod tests;
