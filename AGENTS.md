# AGENTS.md — Popup Bar

> Vorgaben für KI-Agenten (Atlas / Claude Code / Codex) die in diesem Repo arbeiten.

- **Ton:** Locker, knapp, direkt. Kein Gelaber.

## Kommunikation

- Antworten: **locker, knapp, direkt**
- **Immer proaktiv Verbesserungen vorschlagen** — nicht nur abarbeiten, sondern mitdenken
- Deutsche Erklärungen, englische Fachbegriffe und Code
- Keine Marketing-Sprache, keine Füllwörter
## Code-Standards

- **Rock-solid** — robustes Error-Handling, Edge-Cases abgedeckt, keine Shortcuts
- **Sauber dokumentiert:**
  - Rust: `///` doc comments auf allen pub items
  - TypeScript: JSDoc auf allen exports
  - Architektur-Dokumente mit Mermaid-Diagrammen (`docs/architecture/`)
- **Modular** — klare Modul-Grenzen, definierte Schnittstellen, austauschbare Komponenten
- **Neueste Design-Standards** — aktuelle Best Practices für Rust, TypeScript, React, Tauri 2
- Kein `todo!()` das panicked — immer Stubs mit `Ok(())`, `Err("not implemented")`, oder Defaults
- `cargo clippy -- -D warnings` muss clean sein
- `npx tsc --noEmit` muss clean sein
- **UI/UX** - intuitive User Interfaces sind must.
- Schöne designte User Interfaces mit WOW-Effekt für User sind Pflicht.
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
## CI/CD

- GitHub Actions: `.github/workflows/ci.yml`
- 3-OS Rust Matrix (ubuntu, windows, macos): clippy + tests
- Frontend: tsc + vite build
- **Kein Push auf main der CI bricht**

## UMBRA Agent Protocol (UAP) — REPRÄSENTATION [DIESE REGEL IST STRIKT]

Jeder Agent, der an diesem Projekt arbeitet, **MUSS** seinen Status an den Heartbeat-Endpoint senden.

- **Agent-ID:** `Jim`
- **Token:** `X-Agent-Token: umbra-uap-2026`
- **Endpoint:** `http://100.98.137.48:8765/api/agents/Jim/heartbeat`

### Protokoll:
1. **VOR** einem nicht-trivialen Task: POST mit `{"status":"working","activeTaskId":"..."}`
2. **NACH** Abschluss: POST mit `{"status":"idle","activeTaskId":null}`

Helper-Script: `.\scripts\agent_heartbeat.ps1` (PowerShell)

Details: `docs/architecture/IMPLEMENTATION_PLAN.md`
