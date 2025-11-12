use clap::{Parser, Subcommand};
use keyboard_map_shift::shift_highlighted_text_to_next_layout;

#[derive(Parser, Debug)]
#[command(name = "keyboard_map_shift")]
#[command(about = "Shift highlighted text to the next keyboard layout", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run the text transformation
    Run,
    /// Set global hotkey (Windows only)
    #[cfg(target_os = "windows")]
    SetHotkey {
        /// Hotkey in format: Ctrl+Alt+K
        hotkey: String,
    },
}

pub fn execute(cli: Cli) -> Result<(), String> {
    match cli.command.unwrap_or(Commands::Run) {
        Commands::Run => shift_highlighted_text_to_next_layout(),
        #[cfg(target_os = "windows")]
        Commands::SetHotkey { hotkey } => {
            crate::hotkey_windows::set_hotkey(&hotkey)?;
            println!("Hotkey set: {}", hotkey);
            Ok(())
        }
    }
}

