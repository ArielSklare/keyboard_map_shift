mod cli;
#[cfg(target_os = "windows")]
mod hotkey_windows;
use clap::Parser;

fn main() {
    #[cfg(target_os = "windows")]
    {
        // Hide console window immediately to prevent it from interfering with text selection
        hide_console_window();
    }
    
    let cli = cli::Cli::parse();
    let result = cli::execute(cli);
    
    #[cfg(target_os = "windows")]
    {
        // Only show console on error
        if let Err(e) = result {
            show_console_window();
            eprintln!("Error: {}", e);
            use std::io::{self, Write};
            eprintln!("\nPress Enter to exit...");
            let _ = io::stdout().flush();
            let mut buffer = String::new();
            let _ = io::stdin().read_line(&mut buffer);
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        if let Err(e) = result {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(target_os = "windows")]
fn hide_console_window() {
    use windows::Win32::{
        System::Console::GetConsoleWindow,
        UI::WindowsAndMessaging::{ShowWindow, SW_HIDE},
    };
    
    unsafe {
        let console_window = GetConsoleWindow();
        if !console_window.is_invalid() {
            let _ = ShowWindow(console_window, SW_HIDE);
        }
        // Small delay to ensure window is hidden
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

#[cfg(target_os = "windows")]
fn show_console_window() {
    use windows::Win32::{
        System::Console::GetConsoleWindow,
        UI::WindowsAndMessaging::{ShowWindow, SW_SHOW},
    };
    
    unsafe {
        let console_window = GetConsoleWindow();
        if !console_window.is_invalid() {
            let _ = ShowWindow(console_window, SW_SHOW);
        }
    }
}
