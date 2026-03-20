# popup bar

popup bar ist eine tauri-2 desktop-app fuer windows, macos und linux: eine launcher-leiste, die sich per hotzone am bildschirmrand oeffnet und dateien, ordner, apps oder urls persistent in einer shelf speichert.

## status

1. qualitaetsgates sind lokal verifiziert:
   `npm test`, `npm run build`, `cargo test`, `cargo clippy -- -D warnings`
2. ein windows-installer wird real gebaut:
   `src-tauri\target\release\bundle\msi\Popup Bar_0.1.0_x64_en-US.msi`
3. github actions pruefen die frontend-tests, und ein separater workflow baut das windows-msi als artifact.

## features

1. hotzone-trigger mit show/hide-lifecycle
2. persistente shelf-items via sqlite
3. drag-and-drop fuer dateipfade
4. native icon-aufloesung mit cache und fallback
5. tray-icon und global shortcut `Strg/Cmd + Umschalt + Leertaste`
6. echtes multi-monitor-handling ueber den monitor unter dem mauszeiger
7. konfigurierbarer shortcut mit persistenz und optionaler deaktivierung
8. monitor-strategie `primary | cursor | last-active`
9. freie icon-anordnung oder snap-to-grid mit persistenter position
10. settings-panel fuer bar-breite, hoehe, theme, autostart und mehr

## lokal starten

1. voraussetzungen installieren:
   node 20+, rust stable, webview2-runtime unter windows
2. dependencies holen:
   `npm install`
3. dev-app starten:
   `npm run tauri dev`

## qualitaet pruefen

1. frontend-tests:
   `npm test`
2. frontend-build:
   `npm run build`
3. rust-tests:
   `cargo test --manifest-path src-tauri/Cargo.toml`
4. rust-clippy:
   `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings`

## installer bauen

1. voller lokaler release-build:
   `.\scripts\build-installer.ps1`
2. nur bundling ohne vorchecks:
   `.\scripts\build-installer.ps1 -SkipChecks`
3. direkter tauri-call:
   `npx tauri build --bundles msi`

## release-doku

1. installer-flow und artifact-details:
   `docs/release/INSTALLER.md`
2. beitragsregeln:
   `CONTRIBUTING.md`
3. aenderungshistorie:
   `CHANGELOG.md`
4. cleanup- und performance-entscheidungen:
   `docs/architecture/CLEANUP-PERFORMANCE-PASS.md`
5. tray/shortcut/multi-monitor-pass:
   `docs/architecture/TAURI-PRIORITY-PASS.md`
6. runtime-controls und kompatibilitaets-pass:
   `docs/architecture/RUNTIME-CONTROLS-PASS.md`
7. icon-layout und positions-pass:
   `docs/architecture/ICON-LAYOUT-PASS.md`

## produktionsnotizen

1. die bundle-config enthaelt feste release-metadaten, generierte icons und eine stabile wix-upgrade-code.
2. macos- und linux-bundles sind vorbereitet, aber auf diesem windows-host nicht verifiziert.
3. code-signing ist bewusst noch offen; fuer oeffentliche releases gehoert das als naechster schritt dazu.
