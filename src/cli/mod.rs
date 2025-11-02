use clap::{Parser, Subcommand};
use keyboard_map_shift::{HotkeySpec, run_transform_once, update_hotkey};
use std::io::{self, Write};
mod wizard;

#[derive(Parser, Debug)]
#[command(name = "keyboard_map_shift")]
#[command(about = "Shift highlighted text to the next keyboard layout", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Run,
    Setup,
    Settings {
        #[arg(long)]
        hotkey: Option<String>,
    },
}

pub fn execute(cli: Cli) -> Result<(), String> {
    match cli.command.unwrap_or(Commands::Run) {
        Commands::Run => run_transform_once(),
        Commands::Setup => wizard::run_wizard(),
        Commands::Settings { hotkey } => {
            if let Some(hk) = hotkey {
                let spec = HotkeySpec::from_display(&hk)?;
                update_hotkey(&spec)
            } else {
                let cfg = keyboard_map_shift::config::load_config()?;
                println!("Current hotkey: {}", cfg.hotkey);
                print!("Change it now via interactive wizard? [Y/n]: ");
                io::stdout().flush().map_err(|e| e.to_string())?;
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .map_err(|e| e.to_string())?;
                let answer = input.trim().to_ascii_lowercase();
                if answer == "y" || answer == "yes" {
                    wizard::run_wizard()
                } else {
                    println!(
                        "Tip: pass --hotkey \"Ctrl+Alt+K\" to set non-interactively, or run `keyboard_map_shift setup`"
                    );
                    Ok(())
                }
            }
        }
    }
}
