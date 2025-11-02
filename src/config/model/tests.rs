use super::*;

#[test]
fn defaults_match_constant() {
    let cfg = Config::with_defaults();
    assert_eq!(cfg.hotkey, DEFAULT_HOTKEY_DISPLAY);
}

#[test]
fn toml_round_trip() {
    let cfg = Config {
        hotkey: "Ctrl+Alt+K".to_string(),
    };
    let s = toml::to_string_pretty(&cfg).unwrap();
    let back: Config = toml::from_str(&s).unwrap();
    assert_eq!(cfg, back);
}
