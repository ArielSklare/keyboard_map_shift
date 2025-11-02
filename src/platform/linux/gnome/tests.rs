use super::*;

struct GsettingsGuard {
    list_key_value: Option<String>,
    name_value: Option<String>,
    command_value: Option<String>,
    binding_value: Option<String>,
}

impl GsettingsGuard {
    fn capture() -> Self {
        let list_key_value = gsettings_get(
            GNOME_PATHS.media_keys_schema,
            GNOME_KEYS.custom_keybindings_key,
        )
        .ok();
        let name_value = gsettings_get(
            &format!(
                "{}:{}",
                GNOME_PATHS.custom_keybinding_schema, GNOME_PATHS.custom_keybinding_base
            ),
            GNOME_KEYS.kb_name_key,
        )
        .ok();
        let command_value = gsettings_get(
            &format!(
                "{}:{}",
                GNOME_PATHS.custom_keybinding_schema, GNOME_PATHS.custom_keybinding_base
            ),
            GNOME_KEYS.kb_command_key,
        )
        .ok();
        let binding_value = gsettings_get(
            &format!(
                "{}:{}",
                GNOME_PATHS.custom_keybinding_schema, GNOME_PATHS.custom_keybinding_base
            ),
            GNOME_KEYS.kb_binding_key,
        )
        .ok();
        Self {
            list_key_value,
            name_value,
            command_value,
            binding_value,
        }
    }
}

impl Drop for GsettingsGuard {
    fn drop(&mut self) {
        if let Some(ref v) = self.list_key_value {
            let _ = gsettings_set(
                GNOME_PATHS.media_keys_schema,
                GNOME_KEYS.custom_keybindings_key,
                v,
            );
        }
        let target = GNOME_PATHS.custom_keybinding_base;
        if let Some(ref v) = self.name_value {
            let _ = gsettings_set_kb(
                GNOME_PATHS.custom_keybinding_schema,
                target,
                GNOME_KEYS.kb_name_key,
                v,
            );
        }
        if let Some(ref v) = self.command_value {
            let _ = gsettings_set_kb(
                GNOME_PATHS.custom_keybinding_schema,
                target,
                GNOME_KEYS.kb_command_key,
                v,
            );
        }
        if let Some(ref v) = self.binding_value {
            let _ = gsettings_set_kb(
                GNOME_PATHS.custom_keybinding_schema,
                target,
                GNOME_KEYS.kb_binding_key,
                v,
            );
        }
    }
}

#[test]
fn apply_gnome_binding_smoke() {
    let _guard = GsettingsGuard::capture();
    let _ = apply_gnome_binding("Ctrl+Alt+K");
}
