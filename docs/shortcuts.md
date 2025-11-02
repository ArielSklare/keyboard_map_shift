# Global Shortcut Setup: Unsupported Environments

This project configures global shortcuts automatically for:
- Windows (Explorer-managed `.lnk` hotkey)
- Linux GNOME (gsettings custom keybindings)
- Linux KDE Plasma (KGlobalAccel via `kglobalshortcutsrc`)

For other desktop environments or window managers (e.g., i3, sway, awesome),
global shortcuts typically require a user-managed hotkey daemon (e.g., `sxhkd`).
Because this project avoids background processes, automatic setup is not provided.

You can still bind a key manually to run:

```
keyboard_map_shift run
```

Refer to your environment's documentation for creating custom keybindings.



