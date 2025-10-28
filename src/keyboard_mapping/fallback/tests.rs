use super::*;

#[test]
#[should_panic(expected = "keyboard_mapping: get_layout is not implemented for this OS")]
fn test_get_layout_panics() {
    get_layout(0);
}

#[test]
#[should_panic(expected = "keyboard_mapping: list_layouts is not implemented for this OS")]
fn test_list_layouts_panics() {
    list_layouts();
}

#[test]
#[should_panic(
    expected = "keyboard_mapping: vk_to_char_map_for_layout is not implemented for this OS"
)]
fn test_vk_to_char_map_for_layout_panics() {
    vk_to_char_map_for_layout(0);
}

#[test]
#[should_panic(
    expected = "keyboard_mapping: vk_to_char_map_default is not implemented for this OS"
)]
fn test_vk_to_char_map_default_panics() {
    vk_to_char_map_default();
}

#[test]
#[should_panic(expected = "keyboard_mapping: all_layout_vk_maps is not implemented for this OS")]
fn test_all_layout_vk_maps_panics() {
    all_layout_vk_maps();
}
