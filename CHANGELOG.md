# Changelog

## Unreleased

- **Leistenhöhe einstellbar**: Einstellung „Leistenhöhe (px)“ 56–180, Standard 72; Backend `bar_height_px` in `AppSettings`, Fenster-Lifecycle nutzt sie.
- **Blur-Option entfernt**: Slider „Blur“ aus dem Settings-Panel entfernt (funktionierte nicht); Darstellung über plattform-spezifische CSS-Variablen in `glassmorphism.css`.
- **Schrägen entfernt**: Leiste wieder rechteckig (kein `clip-path` mehr an den Ecken).

## Vorher (Session)

- Phase 1–5 implementiert (Hotzone, Shelf, Drag & Drop, Icons, Settings)
- Phase 6 Basis:
  - Rust‑Unit‑Tests für Hotzone, WindowState, ConfigManager, DndHandler
  - Smoke‑Test‑Checkliste (`docs/testing/SMOKE_TEST_PHASE6.md`)
  - Bundling‑Targets in `src-tauri/tauri.conf.json` (`msi`, `dmg`, `appimage`, `deb`)

