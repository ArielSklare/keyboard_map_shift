use super::*;

#[test]
fn parse_default_run() {
    let cli = Cli::parse_from(["keyboard_map_shift"]);
    assert!(cli.command.is_none());
}

#[test]
fn parse_run_subcommand() {
    let cli = Cli::parse_from(["keyboard_map_shift", "run"]);
    match cli.command.unwrap() {
        Commands::Run => {}
        _ => panic!("expected Run"),
    }
}

#[test]
fn parse_setup_subcommand() {
    let cli = Cli::parse_from(["keyboard_map_shift", "setup"]);
    match cli.command.unwrap() {
        Commands::Setup => {}
        _ => panic!("expected Setup"),
    }
}

#[test]
fn parse_settings_with_hotkey() {
    let cli = Cli::parse_from(["keyboard_map_shift", "settings", "--hotkey", "Ctrl+Alt+K"]);
    match cli.command.unwrap() {
        Commands::Settings { hotkey } => {
            assert_eq!(hotkey, Some("Ctrl+Alt+K".to_string()));
        }
        _ => panic!("expected Settings"),
    }
}
