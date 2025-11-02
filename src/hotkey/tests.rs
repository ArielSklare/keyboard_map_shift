use super::*;

#[test]
fn normalize_display_variants() {
    assert_eq!(normalize_display("ctrl+alt+k"), "Ctrl+Alt+K");
    assert_eq!(normalize_display("Shift + a"), "Shift+A");
}

#[test]
fn parse_display_ok() {
    let hk = parse_display("Ctrl+Alt+K").unwrap();
    assert!(hk.ctrl);
    assert!(hk.alt);
    assert!(!hk.shift);
    assert_eq!(hk.key, 'K');
}

#[test]
#[should_panic]
fn parse_display_err_panics_on_unwrap() {
    let _ = parse_display("Meta+K").unwrap();
}

#[test]
fn gnome_binding_from_display() {
    let s = to_gnome_binding_from_display("Ctrl+Alt+K").unwrap();
    assert_eq!(s, "<Control><Alt>K");
}

#[test]
fn windows_hotkey_word_from_display() {
    let w = to_windows_hotkey_word_from_display("Ctrl+Alt+K").unwrap();
    assert_eq!(w, 0x064B);
}
