# Popup Bar

Cross‑platform Popup‑Bar für Windows, macOS und Linux.  
Always‑on‑top, Hotzone‑Trigger am oberen Bildschirmrand, Shelf für Dateien/Ordner/Apps/URLs mit nativen Icons.

## Features

- Hotzone‑Trigger mit Slide‑In/Out‑Animation
- Persistente Shelf‑Items (SQLite)
- Drag & Drop & Reorder
- Native Icon‑Extraktion (Win/macOS/Linux)
- **Separate Settings Window**: Öffnet dynamisch an der Mausposition.
- **Brand Design**: Neon Cyberpunk Edition (Void Background, Cyan/Violet Accents).
- Multi‑Monitor Support & Autostart via Settings.

## Entwicklung

- Voraussetzungen: Node 20, Rust stable, Tauri 2
- Start Dev‑Umgebung:
  - `npm install`
  - `npm run tauri dev`
- Tests:
  - Rust: `cd src-tauri && cargo test`
  - Frontend: `npm test`

