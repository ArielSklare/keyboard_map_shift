use std::io::{self, Write};

use keyboard_map_shift::config::{DEFAULT_HOTKEY_DISPLAY, load_config};
use keyboard_map_shift::{HotkeySpec, update_hotkey};

pub fn run_wizard() -> Result<(), String> {
    let cfg = load_config().unwrap_or_else(|_| keyboard_map_shift::config::Config::with_defaults());
    println!("First-time setup: configure a global hotkey to launch keyboard_map_shift");
    println!("Current/default hotkey: {}", cfg.hotkey);
    print!(
        "Enter a new hotkey (or press Enter to use {}): ",
        DEFAULT_HOTKEY_DISPLAY
    );
    io::stdout().flush().map_err(|e| e.to_string())?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;
    let chosen = input.trim();
    let display = if chosen.is_empty() {
        DEFAULT_HOTKEY_DISPLAY
    } else {
        chosen
    };
    let spec = HotkeySpec::from_display(display)?;
    update_hotkey(&spec)?;
    println!("Hotkey applied: {}", spec.display);
    Ok(())
}
