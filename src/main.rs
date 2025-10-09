mod get_highlighted;
mod keyboard_mapping;
use keyboard_mapping::{
    all_layout_vk_maps, get_text_leyaout_map, list_layouts, shift_text_language,
    vk_to_char_map_default,
};

fn main() {
    // Small delay so you can focus/select in target app before we read
    std::thread::sleep(std::time::Duration::from_millis(150));

    // Test get_highlighted_text
    match get_highlighted::get_highlighted_text() {
        Some(s) => {
            println!("Highlighted text: {}", s);

            {
                println!("Testing replace_highlighted_text...");
                match get_highlighted::replace_highlighted_text("REPLACED TEXT") {
                    Ok(()) => println!("Successfully replaced highlighted text!"),
                    Err(e) => println!("Failed to replace highlighted text: {}", e),
                }
            }
        }
        None => println!("Highlighted text: <none>"),
    }

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
