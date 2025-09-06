mod keyboard_mapping;

use keyboard_mapping::{all_layout_vk_maps, list_layouts, vk_to_char_map_default};

fn main() {
    let layouts = list_layouts();
    println!("layouts found: {}", layouts.len());

    let current = vk_to_char_map_default();
    println!("current layout mapped keys: {}", current.map.len());

    let all_maps = all_layout_vk_maps();
    println!("all layouts maps: {}", all_maps.len());

    // Pretty print all mappings
    for (idx, lm) in all_maps.iter().enumerate() {
        println!(
            "\nLayout {} (lang: {:?}, dir: {:?}):",
            idx, lm.layout.lang_name, lm.layout.direction
        );
        let mut pairs: Vec<(&u16, &String)> = lm.map.iter().collect();
        pairs.sort_by_key(|(vk, _)| **vk);
        for (vk, ch) in pairs {
            println!("  VK {:#04X} -> {}", vk, ch);
        }
    }
}
