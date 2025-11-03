use super::*;
use crate::test_utils::EnvVarGuard;

#[test]
fn detect_de_identifies_unknown() {
    let _g1 = EnvVarGuard::set_str("XDG_CURRENT_DESKTOP", "unknown-desktop");
    let _g2 = EnvVarGuard::set_str("DESKTOP_SESSION", "unknown-session");
    let de = detect_de();
    match de {
        DesktopEnvironment::Unknown => {}
        _ => panic!("expected Unknown desktop environment"),
    }
}

#[test]
#[should_panic]
fn apply_hotkey_unknown_de_panics_on_unwrap() {
    let _g1 = EnvVarGuard::set_str("XDG_CURRENT_DESKTOP", "unknown-desktop");
    let _g2 = EnvVarGuard::set_str("DESKTOP_SESSION", "unknown-session");
    let binder = LinuxBinder::new();
    binder.apply_hotkey("Ctrl+Alt+K").unwrap();
}

#[test]
fn remove_hotkey_is_ok() {
    let binder = LinuxBinder::new();
    let result = binder.remove_hotkey();
    assert!(result.is_ok());
}
