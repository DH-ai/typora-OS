# TyporaOS

Rust-first desktop Markdown editor architecture inspired by Typora-style live preview.

## Stack

- Tauri v2 desktop shell
- Rust workspace with split crates:
	- `crates/ipc-contracts`: shared frontend/backend command payloads
	- `crates/core`: markdown render + document stats logic
	- `apps/desktop/src-tauri`: desktop command host
- Web UI in `apps/desktop/web` (minimal first editor window)

## Workspace Layout

```
.
├── Cargo.toml
├── crates
│   ├── core
│   └── ipc-contracts
└── apps
		└── desktop
				├── src-tauri
				└── web
```

## Run (Development)

1. Install Rust toolchain and system dependencies (Linux):

```bash
# Ubuntu/Debian
sudo apt-get install libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf

# Fedora
sudo dnf install gtk3-devel webkit2gtk3-devel libappindicator-gtk3-devel librsvg2-devel
```

2. Install Tauri CLI:

```bash
cargo install tauri-cli --version '^2.0'
```

3. Start dev server from workspace root:

```bash
cargo tauri dev
```

Alternatively, run from the desktop app directory:

```bash
cd apps/desktop/src-tauri && cargo tauri dev
```

## Building (Production)

For production builds, you'll need to generate app icons. Place PNG icons in `apps/desktop/src-tauri/icons/`:

```bash
# Generate icons from SVG (requires imagemagick or similar)
convert -background none apps/desktop/src-tauri/icons/icon.svg -define icon:auto-resize=256,128,96,64,48,32,16 apps/desktop/src-tauri/icons/icon.ico
convert apps/desktop/src-tauri/icons/icon.svg apps/desktop/src-tauri/icons/icon.png
```

Then run:

```bash
cargo tauri build
```

## Next Steps

- Replace textarea editor with a ProseMirror-based WYSIWYG engine.
- Move preferences, file I/O, and search pipelines into Rust services.
- Add export pipeline (HTML/PDF first, then DOCX/ODT bridge).
