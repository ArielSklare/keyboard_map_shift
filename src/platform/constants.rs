pub struct GnomeKeybindingPaths {
    pub custom_keybinding_base: &'static str,
    pub media_keys_schema: &'static str,
    pub custom_keybinding_schema: &'static str,
}

pub const GNOME_PATHS: GnomeKeybindingPaths = GnomeKeybindingPaths {
    custom_keybinding_base: "/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/keyboard-map-shift/",
    media_keys_schema: "org.gnome.settings-daemon.plugins.media-keys",
    custom_keybinding_schema: "org.gnome.settings-daemon.plugins.media-keys.custom-keybinding",
};

pub struct GnomeKeyNames {
    pub custom_keybindings_key: &'static str,
    pub kb_name_key: &'static str,
    pub kb_command_key: &'static str,
    pub kb_binding_key: &'static str,
}

pub const GNOME_KEYS: GnomeKeyNames = GnomeKeyNames {
    custom_keybindings_key: "custom-keybindings",
    kb_name_key: "name",
    kb_command_key: "command",
    kb_binding_key: "binding",
};

pub struct KdeKeybindingPaths {
    pub desktop_relative_path: &'static str,
    pub kglobalshortcuts_relative_path: &'static str,
    pub component_group: &'static str,
}

pub const KDE_PATHS: KdeKeybindingPaths = KdeKeybindingPaths {
    desktop_relative_path: ".local/share/applications/keyboard_map_shift.desktop",
    kglobalshortcuts_relative_path: ".config/kglobalshortcutsrc",
    component_group: "keyboard_map_shift.desktop",
};

pub struct KdeKeyNames {
    pub friendly_name_key: &'static str,
    pub trigger_key: &'static str,
}

pub const KDE_KEYS: KdeKeyNames = KdeKeyNames {
    friendly_name_key: "_k_friendly_name",
    trigger_key: "Trigger",
};

pub struct KdeTemplates {
    pub desktop_entry_template: &'static str,
}

pub const KDE_TEMPLATES: KdeTemplates = KdeTemplates {
    desktop_entry_template: "[Desktop Entry]\nType=Application\nName={name}\nExec={exec}\nTerminal=false\nCategories=Utility;\n",
};

pub struct AppStrings {
    pub app_name: &'static str,
    pub exec_run_cmd: &'static str,
    pub kde_component_dbus_path: &'static str,
    pub app_run_subcommand: &'static str,
}

pub const APP_STRINGS: AppStrings = AppStrings {
    app_name: "Keyboard Map Shift",
    exec_run_cmd: "keyboard_map_shift run",
    kde_component_dbus_path: "/component/keyboard_map_shift.desktop",
    app_run_subcommand: "run",
};

pub struct WindowsPaths {
    pub env_appdata: &'static str,
    pub start_menu_programs_rel: &'static str,
    pub shortcut_filename: &'static str,
}

pub const WINDOWS_PATHS: WindowsPaths = WindowsPaths {
    env_appdata: "APPDATA",
    start_menu_programs_rel: "Microsoft\\Windows\\Start Menu\\Programs",
    shortcut_filename: "Keyboard Map Shift.lnk",
};
