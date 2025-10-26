use keyboard_map_shift::shift_highlighted_text_to_next_layout;

fn main() {
    match shift_highlighted_text_to_next_layout() {
        Ok(()) => println!("Successfully shifted highlighted text to next layout"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
