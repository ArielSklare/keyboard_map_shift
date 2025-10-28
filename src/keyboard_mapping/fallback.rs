#![cfg(not(any(target_os = "windows", target_os = "linux")))]

pub fn get_layout(_index: usize) -> Option<KeyboardLayout> {
    panic!("keyboard_mapping: get_layout is not implemented for this OS");
}

pub fn list_layouts() -> Vec<KeyboardLayout> {
    panic!("keyboard_mapping: list_layouts is not implemented for this OS");
}

pub fn vk_to_char_map_for_layout(_index: u32) -> LayoutMap {
    panic!("keyboard_mapping: vk_to_char_map_for_layout is not implemented for this OS");
}

pub fn vk_to_char_map_default() -> LayoutMap {
    panic!("keyboard_mapping: vk_to_char_map_default is not implemented for this OS");
}

pub fn all_layout_vk_maps() -> Vec<LayoutMap> {
    panic!("keyboard_mapping: all_layout_vk_maps is not implemented for this OS");
}

#[cfg(test)]
mod tests;
