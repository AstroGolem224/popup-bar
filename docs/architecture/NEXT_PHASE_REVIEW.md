# Next Phase Review

> Date: 2026-03-12  
> Scope: Abgleich von Plan (`IMPLEMENTATION_PLAN.md`) mit aktuellem Code-Stand (Rust + Frontend)

---

## Ergebnis

**Empfohlene naechste Phase: Phase 5.5 (Performance) / Phase 6 (Testing & Release)**

Begruendung: Phase 1-5.4 sind umgesetzt. Phase 5.1–5.4 (ConfigManager/SQLite, SettingsPanel mit Zahnrad, Autostart, Multi-Monitor-Position bei show) sind implementiert. Offen: 5.5 Performance-Profiling; danach Phase 6.

---

## Prioritaet und Status

| ID | Thema | Prioritaet | Status | Ziel |
|---|---|---|---|---|
| P1-01 | Hotzone Event-Flow (Backend) | P0 | done | `hotzone:enter/leave` robust emittieren |
| P1-02 | Frontend Event-Wiring (`useHotzoneState`) | P0 | done | Sichtbarkeit durch echte Events steuern |
| P1-03 | Slide-In/Out Lifecycle Sync | P0 | done | `show_window`/`hide_window` + completion-ack sauber gekoppelt |
| P1-04 | WindowState Guardrails | P1 | done | Ungueltige Transitions verhindern |
| P1-05 | Tests fuer Event-Flow/FSM | P1 | done | Regressionsschutz fuer Phase 1 (Rust + Hook-Tests) |
| P2-01 | SQLite-Schema & Migration Setup | P0 | done | Persistente DB-Basis inkl. Constraints |
| P2-02 | Shelf CRUD (Rust Commands + Store) | P0 | done | Add/Get/Update/Delete mit SQLite |
| P2-03 | Frontend Shelf Wiring | P0 | done | `useShelfItems` nutzt Tauri + Zustand |
| P2-04 | Shelf UX Vervollstaendigung | P1 | in progress | Vollstaendige Add/Delete-Flows im UI |
| P2-05 | Shelf UX: Error-State + Toasts | P1 | done | Nutzerfeedback bei Backend-Fehlern |
| P3-01 | Drop-Path Handling | P0 | in progress | Externe Drops in persistente Items ueberfuehren |
| P3-02 | Reorder Persistenz | P0 | in progress | Reihenfolge in DB speichern |
| P4-01 | Icon Resolver + Cache Baseline | P0 | done | Caching + Fallback-Icons ausliefern |
| P4-02 | ShelfItem Icon Rendering + Fallback | P0 | done | Sichtbare Item-Icons inkl. Fallback im Grid |
| P4-03 | Native Icon Extraction (Win/macOS/Linux) | P0 | done | System-Icons plattformspezifisch |
| P4-04 | Icon-Cache-Invalidierung | P1 | done | source_path + Existenzpruefung |
| P4-05 | Glassmorphism plattformspezifisch | P1 | done | platform-* Klasse + CSS-Variablen |
| P5-01 | AppSettings + ConfigManager (SQLite) | P0 | done | settings-Tabelle, load/save, settings_changed |
| P5-02 | SettingsPanel (Zahnrad, Backdrop, Live-Preview) | P0 | done | Hotzone/Darstellung/System, DE-Labels |
| P5-03 | Autostart (tauri-plugin-autostart) | P0 | done | set_launch_at_login, launch_at_login-Setting |
| P5-04 | Multi-Monitor (position_on_monitor bei show) | P0 | done | primary_only aus Settings, Bar auf Primärmonitor |
| P5-05 | Performance-Profiling | P1 | open | RAM/CPU/60fps-Ziele |
| DOC-01 | Plan-vs-Code Sync-Prozess | P0 | done | Doku-Drift zwischen Plan und Implementierung vermeiden |

---

## Evidenz aus dem Repo

### 1) Phase 0 ist groesstenteils vorhanden

- CI Pipeline existiert mit Rust + Frontend Checks in `.github/workflows/ci.yml`.
- Tauri Fenster ist transparent/always-on-top/hidden initial konfiguriert in `src-tauri/tauri.conf.json`.
- Basis-App-Struktur mit `App`, `DropZone`, `ShelfBar`, `ShelfGrid`, `ShelfItem` existiert in `src/`.
- Commands sind in `src-tauri/src/lib.rs` registriert.

### 2) Phase 1 ist funktionsfaehig umgesetzt

- `src-tauri/src/modules/hotzone.rs`: Event-Flow vorhanden (`hotzone:enter/leave`) inkl. Debounce-Logik.
- `src-tauri/src/modules/window_manager.rs`: Token-basierte State-Guardrails fuer Show/Hide aktiv.
- `src/hooks/useHotzoneState.ts`: echte Event-Subscription + Animation Completion-Handshake vorhanden.

### 3) Phase 2 Kernstrecke ist umgesetzt

- `src-tauri/migrations/0001_initial.sql`: Schema fuer `shelf_items`, `item_groups`, `settings`, `icon_cache`.
- `src-tauri/src/modules/shelf_store.rs`: SQLite-Init via `sqlx::migrate!()` plus CRUD-Operationen.
- `src-tauri/src/commands/shelf_commands.rs`: `get/add/remove/update_shelf_item` implementiert.
- `src/hooks/useShelfItems.ts`: Laden aus Backend + Add/Remove Wiring auf Zustand-Store.

### 4) Offene Blocker liegen jetzt in Phase 3+

- `src-tauri/src/modules/dnd_handler.rs`: DnD-Flows weiterhin Stub (Phase 3).
- Reorder-Persistenz-Command fuer geordnete IDs fehlt noch (Phase 3.5).
- `src-tauri/src/modules/icon_resolver.rs`: Icon-Pipeline weiterhin Stub (Phase 4).

---

## Kritische Beobachtung

Die Architektur- und Modul-Dokumentation beschreibt teils bereits "vollstaendige" Flows, waehrend der Code aktuell noch stark scaffold-lastig ist. Das ist nicht falsch fuer Planung, aber es erzeugt ein Risiko von Scope-Illusionen bei Priorisierung.

**Verbindliche Massnahme (aufgenommen):**

- Pro Feature-PR wird ein kurzer "Status Sync" in `docs/architecture/IMPLEMENTATION_PLAN.md` eingetragen.
- Statuswerte nur aus diesem Set: `open`, `in progress`, `blocked`, `done`.
- Wenn Code und Plan abweichen, gilt der Code-Stand als Source of Truth und der Plan wird im selben PR nachgezogen.

**Status dieser Kritik:** `accepted` (wird ab jetzt aktiv angewendet).

---

## Konkrete naechste Tasks (Phase 3)

1. `dnd_handler` aktivieren und Drag-Events (`drag-enter`, `drop`, `drag-leave`) in Rust emittieren.
2. Drop-Validierung und ItemType-Klassifizierung fuer Datei/Ordner/App/URL implementieren.
3. Frontend `DropZone` mit visuellem Feedback und Error-Handling verbinden.
4. Reorder-Flow im Frontend + persistenter `reorder_shelf_items` Command im Backend ergaenzen.
5. Smoke-Test fuer DnD + Persistenz mit Protokoll in `docs/testing/SMOKE_TEST_YYYY-MM-DD.md` erfassen.

---

## Entscheidungsbedarf

Vor Implementierungsstart von Phase 3 sollte entschieden werden:

- **Strategie A (schneller):** zuerst Datei/Ordner-Drop auf Windows inkl. Reorder finalisieren.
- **Strategie B (breiter):** URL-Drop und Datei-Drop parallel fuer mehrere Plattformen angehen.

Fuer Solo-Dev-Tempo und schnelles Feedback bleibt **Strategie A** meist effizienter.
