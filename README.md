# Popup Bar

Cross‑platform Popup‑Bar für Windows, macOS und Linux.  
Always‑on‑top, Hotzone‑Trigger am oberen Bildschirmrand, Shelf für Dateien/Ordner/Apps/URLs mit nativen Icons.

## Features

- Hotzone‑Trigger mit Slide‑In/Out‑Animation
- Persistente Shelf‑Items (SQLite)
- Drag & Drop & Reorder
- Native Icon‑Extraktion (Win/macOS/Linux)
- Settings‑Panel (Hotzone, Leistenbreite & -höhe, Animation, Autostart, Multi‑Monitor)

## Entwicklung

- Voraussetzungen: Node 20, Rust stable, Tauri 2
- Start Dev‑Umgebung:
  - `npm install`
  - `npm run tauri dev`
- Tests:
  - Rust: `cd src-tauri && cargo test`
  - Frontend: `npm test`

