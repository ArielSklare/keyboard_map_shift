use super::*;

#[test]
fn test_list_layouts_returns_non_empty() {
    let layouts = list_layouts();
    assert!(
        !layouts.is_empty(),
        "Linux should have at least one keyboard layout"
    );
}

#[test]
fn test_get_layout_valid_index() {
    let layouts = list_layouts();
    if !layouts.is_empty() {
        let layout = get_layout(0);
        assert!(
            layout.is_some(),
            "get_layout(0) should return Some for valid index"
        );

        let layout = layout.unwrap();
        assert!(
            !layout.lang_name.is_empty(),
            "Layout should have non-empty language name"
        );
    }
}

#[test]
fn test_get_layout_invalid_index() {
    let layouts = list_layouts();
    let invalid_index = layouts.len() + 100;
    let layout = get_layout(invalid_index as u32);
    assert!(
        layout.is_none(),
        "get_layout with invalid index should return None"
    );
}

#[test]
fn test_vk_to_char_map_default_structure() {
    let layout_map = vk_to_char_map_default();

    assert!(
        !layout_map.layout.lang_name.is_empty(),
        "Default layout should have language name"
    );
    assert!(
        !layout_map.map.is_empty(),
        "Default layout should have key mappings"
    );
}

#[test]
fn test_vk_to_char_map_default_contains_common_keys() {
    let layout_map = vk_to_char_map_default();

    let common_keys = vec![
        0x26, // KEY_A
        0x38, // KEY_B
        0x36, // KEY_C
        0x39, // KEY_SPACE
        0x1C, // KEY_ENTER
    ];

    for key in common_keys {
        if layout_map.map.contains_key(&key) {
            let output = &layout_map.map[&key];
            assert!(
                !output.is_empty(),
                "KEY_{:02X} should have non-empty output",
                key
            );
        }
    }
}

#[test]
fn test_vk_to_char_map_for_layout_structure() {
    let layouts = list_layouts();
    if !layouts.is_empty() {
        let layout_map = vk_to_char_map_for_layout(0);

        assert!(
            !layout_map.layout.lang_name.is_empty(),
            "Layout should have language name"
        );
        assert!(
            !layout_map.map.is_empty(),
            "Layout should have key mappings"
        );
    }
}

#[test]
fn test_all_layout_vk_maps_structure() {
    let layout_maps = all_layout_vk_maps();

    assert!(
        !layout_maps.is_empty(),
        "Should have at least one layout map"
    );

    for layout_map in &layout_maps {
        assert!(
            !layout_map.layout.lang_name.is_empty(),
            "Each layout should have language name"
        );
        assert!(
            !layout_map.map.is_empty(),
            "Each layout should have key mappings"
        );
    }
}

#[test]
fn test_all_layout_vk_maps_consistency() {
    let layouts = list_layouts();
    let layout_maps = all_layout_vk_maps();

    assert_eq!(
        layouts.len(),
        layout_maps.len(),
        "Number of layouts should match number of layout maps"
    );

    for (i, layout_map) in layout_maps.iter().enumerate() {
        if let Some(layout) = get_layout(i as u32) {
            assert_eq!(
                layout_map.layout.lang_name, layout.lang_name,
                "Layout names should match"
            );
            assert_eq!(
                layout_map.layout.direction, layout.direction,
                "Layout directions should match"
            );
        }
    }
}
