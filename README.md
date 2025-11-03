## keyboard_map_shift

Shift highlighted text to the next keyboard layout (Linux, Windows).

### Overview

`keyboard_map_shift` reads the currently highlighted text, detects its keyboard layout, cycles to the next configured layout, and replaces the selection with the transformed text. It is designed for multilingual typing convenience when you accidentally type in the wrong layout.

### Features

- **Cross-platform**: Linux (GNOME, KDE) and Windows
- **Global hotkey**: Launch the transform with a key combo (default `Ctrl+Alt+K`)
- **Interactive setup**: First-run wizard to set your preferred hotkey
- **Config file**: Simple `config.toml` managed via the `directories` crate

### Supported platforms

- **Linux**: GNOME, KDE
- **Windows**: supported
- **macOS**: not yet supported

## Installation

### From source (current primary method)

Prerequisites:

- Rust toolchain (`rustup`, stable)
- Linux only: development headers for `xkbcommon`
  - Debian/Ubuntu: `sudo apt install libxkbcommon-dev`
  - Fedora: `sudo dnf install libxkbcommon-devel`
  - Arch: `sudo pacman -S libxkbcommon`

Build and run:

```bash
git clone https://github.com/ArielSklare/keyboard_map_shift.git
cd keyboard_map_shift
cargo build --release
./target/release/keyboard_map_shift run
```

### Prebuilt binaries (Releases)

Download from Releases: https://github.com/ArielSklare/keyboard_map_shift/releases

- Linux (x86_64): download `keyboard_map_shift-vX.Y.Z-x86_64-unknown-linux-gnu.tar.gz`
  - Verify: `sha256sum -c keyboard_map_shift-vX.Y.Z-x86_64-unknown-linux-gnu.tar.gz.sha256`
  - Extract: `tar -xzf keyboard_map_shift-vX.Y.Z-x86_64-unknown-linux-gnu.tar.gz`
  - Run: `./keyboard_map_shift-vX.Y.Z-x86_64-unknown-linux-gnu/keyboard_map_shift run`
  - Optional install: `install -m 0755 ./keyboard_map_shift-vX.Y.Z-x86_64-unknown-linux-gnu/keyboard_map_shift ~/.local/bin/`

- Windows (x86_64): download `keyboard_map_shift-vX.Y.Z-x86_64-pc-windows-msvc.zip`
  - Verify (PowerShell):
    ```powershell
    CertUtil -hashfile keyboard_map_shift-vX.Y.Z-x86_64-pc-windows-msvc.zip SHA256
    Get-Content keyboard_map_shift-vX.Y.Z-x86_64-pc-windows-msvc.zip.sha256
    ```
  - Extract the ZIP and run `keyboard_map_shift.exe`
  - Optional: add the extracted folder to your PATH

### crates.io

Coming soon. Once published, installation will look like:

```bash
cargo install keyboard_map_shift
```

## Quick start

- **Run once** (perform one transform):

```bash
keyboard_map_shift run
```

- **First-time setup wizard** (configure global hotkey):

```bash
keyboard_map_shift setup
```

- **Change hotkey non-interactively**:

```bash
keyboard_map_shift settings --hotkey "Ctrl+Alt+K"
```

## Configuration

- **Default hotkey**: `Ctrl+Alt+K`
- **Config file**: determined via `directories::ProjectDirs::from("com", "keyboard-map-shift", "keyboard_map_shift")` and named `config.toml`.
  - Linux (typical): `~/.config/keyboard_map_shift/config.toml`
  - Windows (typical): `%APPDATA%/keyboard_map_shift/config.toml`

Example `config.toml`:

```toml
hotkey = "Ctrl+Alt+K"
```

## Usage details

Subcommands:

- `run`: Detect layout for the highlighted text, shift to the next layout, and replace selection
- `setup`: Interactive wizard to set and apply the global hotkey
- `settings [--hotkey <DISPLAY>]`: Show or update the hotkey without the full wizard

Notes:

- The global hotkey is applied using the platform-specific binder under `src/platform/`.
- On Linux, ensure your desktop environment allows the app to register a global shortcut.
- On Windows, you may need to grant permission to create a system-wide hotkey.

## Troubleshooting

- **No text is currently highlighted**: The tool acts on the active selection. Select text and try again.
- **Could not determine the layout of the highlighted text**: The text might not match known layouts; try a different sample or verify supported layouts.
- **No next layout found**: There may be only one layout available; ensure multiple layouts are configured.
- **Linux: missing `xkbcommon`**: Install the development package (see Installation).

## Project structure

```
src/
  cli/                # CLI entrypoints (subcommands, interactive wizard)
  config/             # Config model, IO, and path resolution (via directories)
  get_highlighted/    # Platform-specific highlighted-text retrieval
  hotkey/             # Hotkey normalization and helpers
  keyboard_mapping/   # Layout maps and text shifting logic
  platform/           # Platform/Desktop-Env specific integration and binders
  lib.rs              # Library exports and high-level operations
  main.rs             # Binary entrypoint (clap-based CLI)
docs/
  shortcuts.md        # Additional documentation for shortcuts
```

## Development

```bash
cargo build
cargo run -- run
cargo test

# Optional
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt
```

## Security & privacy

- Local-only: no network communication
- Reads/writes only the application config path (to store hotkey)

## Roadmap

- Publish to crates.io
- Provide official Linux/Windows release binaries
- Expand Linux desktop environment coverage
- Explore macOS support

## License

MIT. See `LICENSE` in the repository root.


