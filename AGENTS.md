# AGENTS.md — Popup Bar

> Vorgaben für KI-Agenten (Atlas / Claude Code / Codex) die in diesem Repo arbeiten.

---

## Wer bin ich

- **Name:** Matthias, Alias **Druid**
- **Rolle:** Solo-Dev, Full-Stack
- **Ton:** Locker, knapp, direkt. Kein Gelaber.

---

## Kommunikation

- Antworten: **locker, knapp, direkt**
- **Immer proaktiv Verbesserungen vorschlagen** — nicht nur abarbeiten, sondern mitdenken
- Deutsche Erklärungen, englische Fachbegriffe und Code
- Keine Marketing-Sprache, keine Füllwörter

---

## Code-Standards

- **Rock-solid** — robustes Error-Handling, Edge-Cases abgedeckt, keine Shortcuts
- **Produktionsreif ab Zeile 1** — kein "machen wir später"
- **Sauber dokumentiert:**
  - Rust: `///` doc comments auf allen pub items
  - TypeScript: JSDoc auf allen exports
  - Architektur-Dokumente mit Mermaid-Diagrammen (`docs/architecture/`)
- **Modular** — klare Modul-Grenzen, definierte Schnittstellen, austauschbare Komponenten
- **Neueste Design-Standards** — aktuelle Best Practices für Rust, TypeScript, React, Tauri 2
- Kein `todo!()` das panicked — immer Stubs mit `Ok(())`, `Err("not implemented")`, oder Defaults
- `cargo clippy -- -D warnings` muss clean sein
- `npx tsc --noEmit` muss clean sein

---

## Workflow-Regeln (STRIKT)

### 1. Planung vor Implementierung

- **VOR** jeder Implementierung die Planungsdokumente lesen:
  - `docs/architecture/ARCHITECTURE.md` — System-Architektur
  - `docs/architecture/MODULES.md` — Modulstruktur
  - `docs/architecture/IMPLEMENTATION_PLAN.md` — Phasenplan mit Aufgaben
  - `docs/adrs/` — Architektur-Entscheidungen
- Falls keine Planung existiert: **erst planen, dann coden**

### 2. Lesen vor Schreiben

- **IMMER** eine Datei lesen bevor sie überschrieben wird
- Verstehen was da ist → dann gezielt ändern
- Keine blinden Überschreibungen

### 3. Commit-Messages

- Conventional Commits: `feat(phase-X):`, `fix:`, `docs:`, `refactor:`
- Kurze Zusammenfassung + Aufzählung der Änderungen
- Referenz auf Phase/Aufgabe aus dem Implementierungsplan

---

## Projekt-Kontext

- **Tech-Stack:** Tauri 2 (Rust) + React + TypeScript + Vite + Zustand
- **Ziel:** Cross-Platform Desktop Popup-Bar (Windows, macOS, Linux)
- **Design:** Glasmorphism/Blur, Slide-Down vom oberen Bildschirmrand
- **Architektur:** Event-Driven, Plugin-fähig, PlatformProvider-Trait für OS-Abstraktion

### Rust-Module (`src-tauri/src/modules/`)
| Modul | Verantwortung |
|-------|---------------|
| `hotzone` | Maus-Tracking, Hotzone-Erkennung |
| `window_manager` | Fensterposition, Animation, Transparenz |
| `shelf_store` | Datenmodell, CRUD, SQLite |
| `icon_resolver` | Icon-Extraktion, Caching |
| `dnd_handler` | Drag & Drop |
| `config` | Settings-Management |
| `launcher` | Datei/App/URL öffnen |
| `platform/` | OS-Abstraktion (Windows/macOS/Linux) |

### Frontend-Komponenten (`src/components/`)
`App → DropZone → ShelfBar → ShelfGrid → ShelfItem / ItemGroup`

### Schnittstelle Rust ↔ Frontend

- **Commands** (Request-Response): `get_shelf_items`, `add_shelf_item`, `get_settings`, etc.
- **Events** (Pub-Sub): `hotzone:enter`, `hotzone:leave`, `shelf:updated`, `window:show`, etc.

---

## CI/CD

- GitHub Actions: `.github/workflows/ci.yml`
- 3-OS Rust Matrix (ubuntu, windows, macos): clippy + tests
- Frontend: tsc + vite build
- **Kein Push auf main der CI bricht**

---

## Phasenplan (Kurzversion)

| Phase | Was | Status |
|-------|-----|--------|
| 0 | Fundament: Scaffold, CI/CD, Vibrancy, Glasmorphism | ✅ Done |
| 1 | Hotzone-Detection + Slide-Animation | ✅ Done |
| 2 | Shelf Core: SQLite, CRUD, Grid-UI | ✅ Done |
| 3 | Drag & Drop | ✅ Done |
| 4 | Icons & Visuals | ✅ Done |
| 5 | Settings-UI, Autostart, Performance | ✅ Done |
| 6 | Testing & Release | ⬜ Offen |

Details: `docs/architecture/IMPLEMENTATION_PLAN.md`
