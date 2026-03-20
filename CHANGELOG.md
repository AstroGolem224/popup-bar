# changelog

## unreleased
- **icon layout**: icons lassen sich jetzt frei anordnen; `grid` snappt am raster, `centered` und `start` erlauben freie platzierung mit unterschiedlichem startlayout.
- **position persistence**: die bestehende item-`position` wird nun wirklich im frontend genutzt und bei drag-ende ueber `update_shelf_item` gespeichert.
- **runtime controls**: `globalShortcut` ist jetzt persistiert, im settings-panel editierbar und kann leer gesetzt werden, um den shortcut zu deaktivieren.
- **tray status**: tray-toggle-text folgt jetzt dem echten fensterzustand und wechselt zwischen `Show Popup Bar` und `Hide Popup Bar`.
- **monitor strategy**: monitor-routing auf `primary | cursor | last-active` erweitert; legacy-`multiMonitor` wird kompatibel auf die neue strategie gemappt.
- **tauri priority pass**: tray-icon, global shortcut `CommandOrControl+Shift+Space` und echtes multi-monitor-positioning ueber den monitor unter dem mauszeiger eingebaut.
- **settings clarity**: system- und multi-monitor-verhalten im settings-panel erklaert, damit shortcut/tray nicht versteckt bleiben.
- **cleanup + performance**: worktree-altlasten bereinigt, rust-settings-state gecacht, batch-insert fuer bulk-drops eingefuehrt und redundante frontend-refetches/media-loads reduziert.

- **release-hardening**: kaputte frontend-tests repariert, rust-build/clippy auf grün gebracht und den lokalen quality-gate wiederhergestellt.
- **windows installer**: verifizierter msi-build via `.\scripts\build-installer.ps1` und neuer github workflow `Windows Installer`.
- **bundle metadata**: feste wix-upgrade-code, release-metadaten und generierte plattform-icons in `src-tauri/icons/`.
- **doku**: README, CONTRIBUTING und `docs/release/INSTALLER.md` für den installer- und release-flow ausgebaut.
- **leistenhöhe einstellbar**: einstellung „leistenhöhe (px)“ 56–180, standard 72; backend `bar_height_px` in `AppSettings`, fenster-lifecycle nutzt sie.
- **blur-option entfernt**: slider „blur“ aus dem settings-panel entfernt; darstellung läuft jetzt über die vorhandenen plattform-spezifischen styles.
- **schrägen entfernt**: leiste wieder rechteckig, kein `clip-path` mehr an den ecken.

## vorher

- phase 1–5 implementiert: hotzone, shelf, drag and drop, icons, settings
- phase 6 basis:
  - rust-unit-tests für hotzone, windowstate, configmanager, dndhandler
  - smoke-test-checkliste in `docs/testing/SMOKE_TEST_PHASE6.md`
  - bundling-targets in `src-tauri/tauri.conf.json`
