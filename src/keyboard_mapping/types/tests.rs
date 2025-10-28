use super::*;
use std::collections::HashMap;

#[test]
fn test_keyboard_direction_variants() {
    assert_eq!(KeyboardDirection::LTR, KeyboardDirection::LTR);
    assert_eq!(KeyboardDirection::RTL, KeyboardDirection::RTL);
    assert_ne!(KeyboardDirection::LTR, KeyboardDirection::RTL);
}

#[test]
fn test_keyboard_direction_copy() {
    let dir1 = KeyboardDirection::LTR;
    let dir2 = dir1;
    assert_eq!(dir1, dir2);
}

#[test]
fn test_keyboard_layout_creation() {
    let layout = KeyboardLayout {
        lang_name: "English".to_string(),
        direction: KeyboardDirection::LTR,
    };

    assert_eq!(layout.lang_name, "English");
    assert_eq!(layout.direction, KeyboardDirection::LTR);
}

#[test]
fn test_keyboard_layout_equality() {
    let layout1 = KeyboardLayout {
        lang_name: "Hebrew".to_string(),
        direction: KeyboardDirection::RTL,
    };

    let layout2 = KeyboardLayout {
        lang_name: "Hebrew".to_string(),
        direction: KeyboardDirection::RTL,
    };

    let layout3 = KeyboardLayout {
        lang_name: "Arabic".to_string(),
        direction: KeyboardDirection::RTL,
    };

    assert_eq!(layout1, layout2);
    assert_ne!(layout1, layout3);
}

#[test]
fn test_keyboard_layout_clone() {
    let original = KeyboardLayout {
        lang_name: "French".to_string(),
        direction: KeyboardDirection::LTR,
    };

    let cloned = original.clone();
    assert_eq!(original, cloned);
    assert_eq!(original.lang_name, cloned.lang_name);
    assert_eq!(original.direction, cloned.direction);
}

#[test]
fn test_keyboard_layout_debug() {
    let layout = KeyboardLayout {
        lang_name: "German".to_string(),
        direction: KeyboardDirection::LTR,
    };

    let debug_str = format!("{:?}", layout);
    assert!(debug_str.contains("German"));
    assert!(debug_str.contains("LTR"));
}

#[test]
fn test_layout_map_creation() {
    let mut map = HashMap::new();
    map.insert(0x41, "A".to_string());
    map.insert(0x42, "B".to_string());

    let layout = KeyboardLayout {
        lang_name: "English".to_string(),
        direction: KeyboardDirection::LTR,
    };

    let layout_map = LayoutMap { layout, map };

    assert_eq!(layout_map.layout.lang_name, "English");
    assert_eq!(layout_map.map.len(), 2);
    assert_eq!(layout_map.map.get(&0x41), Some(&"A".to_string()));
    assert_eq!(layout_map.map.get(&0x42), Some(&"B".to_string()));
}

#[test]
fn test_layout_map_equality() {
    let mut map1 = HashMap::new();
    map1.insert(0x41, "A".to_string());

    let mut map2 = HashMap::new();
    map2.insert(0x41, "A".to_string());

    let layout1 = KeyboardLayout {
        lang_name: "English".to_string(),
        direction: KeyboardDirection::LTR,
    };

    let layout2 = KeyboardLayout {
        lang_name: "English".to_string(),
        direction: KeyboardDirection::LTR,
    };

    let layout_map1 = LayoutMap {
        layout: layout1,
        map: map1,
    };

    let layout_map2 = LayoutMap {
        layout: layout2,
        map: map2,
    };

    assert_eq!(layout_map1, layout_map2);
}

#[test]
fn test_layout_map_clone() {
    let mut map = HashMap::new();
    map.insert(0x41, "A".to_string());

    let layout = KeyboardLayout {
        lang_name: "English".to_string(),
        direction: KeyboardDirection::LTR,
    };

    let original = LayoutMap {
        layout: layout.clone(),
        map: map.clone(),
    };

    let cloned = original.clone();
    assert_eq!(original, cloned);
    assert_eq!(original.layout.lang_name, cloned.layout.lang_name);
    assert_eq!(original.map.len(), cloned.map.len());
}

#[test]
fn test_layout_map_debug() {
    let mut map = HashMap::new();
    map.insert(0x41, "A".to_string());

    let layout = KeyboardLayout {
        lang_name: "English".to_string(),
        direction: KeyboardDirection::LTR,
    };

    let layout_map = LayoutMap { layout, map };

    let debug_str = format!("{:?}", layout_map);
    assert!(debug_str.contains("English"));
    assert!(debug_str.contains("LTR"));
}

#[test]
fn test_empty_layout_map() {
    let layout = KeyboardLayout {
        lang_name: "Empty".to_string(),
        direction: KeyboardDirection::LTR,
    };

    let layout_map = LayoutMap {
        layout,
        map: HashMap::new(),
    };

    assert_eq!(layout_map.map.len(), 0);
    assert_eq!(layout_map.layout.lang_name, "Empty");
}
