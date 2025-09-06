#![cfg(target_os = "windows")]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyboardLayout, GetKeyboardLayoutList, HKL, MAPVK_VK_TO_VSC_EX, MapVirtualKeyExW,
    ToUnicodeEx,
};

use crate::keyboard_mapping::types::{KeyboardDirection, KeyboardLayout, LayoutMap};
use std::collections::HashMap;

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

pub fn get_layout(index: usize) -> Option<KeyboardLayout> {
    let hkls = enumerate_hkls();
    hkls.get(index)
        .map(|&hkl| keyboard_layout_from_hkl(hkl, index))
}

pub fn list_layouts() -> Vec<KeyboardLayout> {
    let hkls = enumerate_hkls();
    hkls.into_iter()
        .enumerate()
        .map(|(i, h)| keyboard_layout_from_hkl(h, i))
        .collect()
}

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

pub fn vk_to_char_map_default() -> LayoutMap {
    unsafe { vk_to_char_map_for_layout(GetKeyboardLayout(0)) }
}

pub fn all_layout_vk_maps() -> Vec<LayoutMap> {
    let hkls = enumerate_hkls();
    hkls.into_iter()
        .map(|hkl| vk_to_char_map_for_layout(hkl))
        .collect()
}
