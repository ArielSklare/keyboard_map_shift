use super::*;

#[test]
fn gnome_constants_non_empty() {
    assert!(!GNOME_PATHS.custom_keybinding_base.is_empty());
    assert!(!GNOME_PATHS.media_keys_schema.is_empty());
    assert!(!GNOME_PATHS.custom_keybinding_schema.is_empty());
    assert!(!GNOME_KEYS.custom_keybindings_key.is_empty());
    assert!(!GNOME_KEYS.kb_name_key.is_empty());
    assert!(!GNOME_KEYS.kb_command_key.is_empty());
    assert!(!GNOME_KEYS.kb_binding_key.is_empty());
}

#[test]
fn kde_constants_and_template() {
    assert!(!KDE_PATHS.desktop_relative_path.is_empty());
    assert!(!KDE_PATHS.kglobalshortcuts_relative_path.is_empty());
    assert!(!KDE_PATHS.component_group.is_empty());
    assert!(!KDE_KEYS.friendly_name_key.is_empty());
    assert!(!KDE_KEYS.trigger_key.is_empty());

    let filled = KDE_TEMPLATES
        .desktop_entry_template
        .replace("{name}", APP_STRINGS.app_name)
        .replace("{exec}", APP_STRINGS.exec_run_cmd);
    assert!(filled.contains("Name="));
    assert!(filled.contains("Exec="));
}

#[test]
fn windows_constants_non_empty() {
    assert!(!WINDOWS_PATHS.env_appdata.is_empty());
    assert!(!WINDOWS_PATHS.start_menu_programs_rel.is_empty());
    assert!(!WINDOWS_PATHS.shortcut_filename.is_empty());
}
