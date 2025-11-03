use super::*;

#[test]
fn test_list_layouts_returns_non_empty() {
    let layouts = list_layouts();
    assert!(
        !layouts.is_empty(),
        "Windows should have at least one keyboard layout"
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
    let layout = get_layout(invalid_index);
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

    let common_vks = vec![0x41, 0x42, 0x43, 0x20, 0x0D];

    for vk in common_vks {
        if layout_map.map.contains_key(&vk) {
            let output = &layout_map.map[&vk];
            assert!(
                !output.is_empty(),
                "VK_{:02X} should have non-empty output",
                vk
            );
        }
    }
}

#[test]
fn test_vk_to_char_map_for_layout_structure() {
    let layouts = list_layouts();
    if !layouts.is_empty() {
        let hkls = super::enumerate_hkls();
        if let Some(hkl) = hkls.first().copied() {
            let layout_map = vk_to_char_map_for_layout(hkl);

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
        if let Some(layout) = get_layout(i) {
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
