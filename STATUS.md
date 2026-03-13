## projektstatus – popup bar

### überblick

- **ziel**: cross‑platform popup bar (windows, macOS, linux) mit hotzone am oberen bildschirmrand, persistenter shelf-leiste und nativen icons.
- **stack**: react 18 + vite + typescript + zustand, tauri 2 (rust), sqlite via sqlx, window-vibrancy, tauri plugins (sql, shell, autostart).
- **stand**: kernfunktionalität für windows ist weitgehend vorhanden (hotzone, shelf, drag & drop, settings, launcher, gruppen); auf macOS/linux gibt es bei hotzone/launcher noch lücken.

### frontend-status

- **app & hotzone**
  - `App.tsx` verwendet `useHotzoneState`, um auf `hotzone:enter` / `hotzone:leave` events aus dem backend zu reagieren.
  - window-lifecycle (show/hide/complete) ist mit tauri-commands verdrahtet (`show_window`, `hide_window`, `complete_*`).
  - **tests**: `useHotzoneState.test.ts` deckt das token-/lifecycle-verhalten gut ab.

- **shelf-daten & zustand**
  - `useShelfStore` verwaltet `items`, `groups` und `errorMessage` sowie CRUD- und reorder-funktionen.
  - `useShelfItems` lädt items (und weiterhin groups im Store für Kompatibilität); UI zeigt nur eine flache Item-Liste, **keine Gruppierung** mehr.
  - CRUD/remove über `addShelfItem` / `removeShelfItem`; Löschen per ×-Button am Item.

- **drag & drop und reordering**
  - `useDragDrop` + `DropZone`: Datei-/Ordner-Drops nur über Tauri `onDragDropEvent` (vermeidet doppelte Ablage); HTML5-`onDrop` nur noch für URL-Daten (text/uri-list, text/plain).
  - `addDroppedPaths` und `addShelfItem` aktualisieren den store.
  - `useItemReorder` + `ShelfGrid` + `ShelfItem` unterstützen drag‑based reorder (wo möglich) mit optimistic update; **Löschen** nur per ×-Button am Item.
  - **Doppelte Ablage** vermieden: HTML5-`onDrop` ignoriert `file://`-URLs (nur Tauri verarbeitet Dateien); Tauri-Drop mit Debounce (gleiche Pfade innerhalb 800 ms werden nur einmal hinzugefügt).
  - **Exit**: Knopf (✕) über dem Zahnrad ruft `exit_app` auf → App beendet sich.
  - **Leiste**: rechteckig (keine Schrägen/Ecken mehr); Breite und **Höhe** in den Einstellungen einstellbar (Leistenbreite, Leistenhöhe).

- **settings & UI**
  - `settingsStore` + `SettingsPanel` bilden `AppSettings` aus dem backend ab (`hotzoneSize`, `barWidthPx`, `barHeightPx`, `animationSpeed`, `multiMonitor`, `autostart`). **Blur-Option entfernt** (funktionierte nicht; Darstellung nutzt plattform-spezifische CSS-Variablen aus `glassmorphism.css`).
  - settings werden initial über `getSettings` geladen und per `updateSettings` gespeichert, inklusive autostart-toggle (`setLaunchAtLogin`).
  - frontend hört aktuell nicht explizit auf das `settings_changed` event, sondern verlässt sich auf den rückgabewert der update-funktion.

- **utils & sonstiges**
  - `tauri-bridge.ts` bündelt alle `invoke`-calls typisiert.
  - `useGlassmorphism` nutzt nur noch CSS-Variablen aus `glassmorphism.css` (kein dynamischer Blur aus Settings).
  - `useTauriCommand` ist bewusst als stub („Phase 0“) belassen und wird nirgendwo produktiv verwendet.

### backend-status (rust/tauri)

- **app-setup**
  - `popup_bar_lib::run()` konfiguriert tauri, registriert plugins (sql, shell, autostart) und alle commands (`settings_*`, `shelf_*`, `system_*`).
  - im `setup` werden das main-window, window-vibrancy, der `HotzoneTracker` und der sqlite-basierte `ShelfStore` initialisiert.

- **hotzone-tracking**
  - `HotzoneTracker` in `modules/hotzone.rs` pollt die mausposition über das `PlatformProvider`-interface und emittiert `hotzone:enter` / `hotzone:leave`.
  - die zustandsmaschine (`evaluate_hotzone_transition`) ist mit unit-tests abgesichert.
  - **status nach plattform**:
    - **windows**: `get_mouse_position` ist implementiert → hotzone funktioniert.
    - **macOS / linux**: `get_mouse_position` ist noch nicht implementiert (liefert errors) → hotzone ist dort de facto inaktiv.

- **datenmodell & sqlite**
  - `ShelfStore` definiert `ShelfItem`, `ItemGroup`, `ItemType` und nutzt einen globalen `SqlitePool` via `OnceCell`.
  - CRUD- und reorder-ops für items sowie gruppen-ops sind vorhanden; migrationen werden in `init_db()` ausgeführt.
  - `ConfigManager` in `config.rs` verwaltet `AppSettings` in einer eigenen `settings`-tabelle (separate sqlite-verbindung).

- **commands**
  - `shelf_commands.rs`: shelf CRUD, `add_dropped_paths`, `reorder_shelf_items`; gruppen; `get_icon_data` (liest Icon-Cache als Base64 für Frontend, um asset-protocol zu umgehen). Icon-resolving bei add.
  - `settings_commands.rs`: `get_settings`, `update_settings` (inkl. `settings_changed`-event), `set_launch_at_login` via autostart-plugin.
  - `system_commands.rs`: `get_platform_info`; window-lifecycle; `open_shelf_item` (launcher).

- **fenster-lifecycle & vibrancy**
  - `PopupWindowManager` in `window_manager.rs` kapselt den window-state (`Hidden`, `Showing`, `Visible`, `Hiding`) mit tokens gegen race conditions, inkl. tests.
  - `configure_window_vibrancy` in `lib.rs` nutzt `window_vibrancy` für windows/mac; linux bekommt nur CSS-fallback.
  - Fenstergröße: Breite und Höhe kommen aus `AppSettings` (`bar_width_px`, `bar_height_px`); bei geöffnetem Settings-Panel wird die Höhe temporär auf `BAR_HEIGHT_SETTINGS` gesetzt.
  - settings-werten (`blur_intensity`, `tint_color`) werden aktuell noch nicht in die vibrancy-konfiguration zurückgespielt.

- **drag & drop backend**
  - `DndHandler` klassifiziert & validiert paths und baut `ShelfItem`s für dropps (`build_items_from_paths`), inkl. tests.
  - listener-registrierung und `handle_drop` sind derzeit stubs (nur logs / `accepted: false`).

- **icon-resolver & anzeige**
  - `IconResolver` cached icons als png/svg im temp-icon-cache; sqlite `icon_cache`-tabelle.
  - Frontend lädt Icons über `get_icon_data` (Base64 + Mime) und zeigt sie als Data-URL, damit kein asset-protocol-Scope nötig ist (Original-Icons sichtbar).
  - platform-spezifische `extract_icon` (windows: powershell + System.Drawing; macos/linux: OS-typisch).

- **launcher**
  - `Launcher` in `launcher.rs`: unter Windows `cmd /c start "" "path"` für zuverlässiges Öffnen (inkl. .lnk, Leerzeichen); sonst `tauri-plugin-shell` (open deprecated → ggf. später tauri-plugin-opener).
  - `open_shelf_item` parst `ItemType` und ruft `Launcher::open` auf.
  - frontend: `openShelfItemViaLauncher`; double-click/enter/space auf `ShelfItem` triggert den launcher.

### tests & qualität

- **frontend**:
  - vitest-konfiguration vorhanden; aktuell nur `useHotzoneState.test.ts`.
  - ESLint/TS-lints: `ReadLints` meldet keine fehler in `src`.

- **backend**:
  - gezielte unit-tests in `hotzone.rs`, `config.rs`, `window_manager.rs`, `dnd_handler.rs` u.a.
  - die tests konzentrieren sich auf logik und zustandsautomaten, nicht auf integration mit tauri.

### bekannte lücken / offene TODOs

**mvp-funktionalität**

- **launcher-flow** — erledigt
  - `Launcher` + `open_shelf_item`-command + frontend `openShelfItemViaLauncher`; shelf-items per double-click/enter/space öffnen.

- **gruppen (basic + komfort)** — erledigt
  - tauri-commands: `get_item_groups`, `create_item_group`, `update_item_group`, `delete_item_group`; `ShelfStore::update_group` im backend.
  - UI: „+“ neue gruppe, doppelklick auf gruppenname zum umbenennen, farbchips zum ändern der rahmenfarbe, „×“ zum löschen.

- **gruppen: items zuweisen** — erledigt
  - item auf gruppe ziehen setzt `groupId`; `useShelfItems.updateItem`, `ShelfItem` setzt drag-data, `ItemGroup` ist drop-target mit visueller markierung.

**plattformunterstützung**

- **hotzone auf macOS / linux**
  - `get_mouse_position` in den jeweiligen `PlatformProvider`-implementierungen fertigstellen.
  - optional: echte register/unregister-hotzone-implementierungen statt polling.

- **item-launching plattform-spezifisch** (optional)
  - `PlatformProvider::launch_item` implementieren, falls feinere kontrolle statt nur `tauri-plugin-shell` gewünscht ist.

**architektur & qualität**

- **sqlite-verbindungen vereinheitlichen**
  - einen gemeinsamen pool/connector für `ShelfStore`, `ConfigManager` und `IconResolver` einführen, um mehrfach-verbindungen zu vermeiden.

- **events & settings**
  - frontend auf `settings_changed` hören und settings-store aktualisieren, statt nur auf rückgaben von `updateSettings` zu vertrauen.
  - optional: hotzone-/vibrancy-parameter zur laufzeit aus settings nachziehen.

### kurzfazit

- **windows**: feature-stand hoch — hotzone, shelf, drag & drop, settings, launcher, gruppen (anlegen/umbenennen/farbe/löschen) umgesetzt; code modular und typisiert, unit-tests vorhanden.
- **mac/linux**: hotzone (`get_mouse_position`) noch nicht implementiert; launcher per shell-plugin plattformübergreifend nutzbar.
- **priorität**: hotzone auf macOS/linux; danach architektur (sqlite-pooling, `settings_changed`-listener).

