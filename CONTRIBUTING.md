## beitrag leisten

1. repo klonen und dependencies installieren:
   `npm install`
2. vor änderungen immer den plan lesen:
   `docs/architecture/IMPLEMENTATION_PLAN.md`
3. lokale checks vor jedem pr:
   `npm test`
   `npm run build`
   `cargo test --manifest-path src-tauri/Cargo.toml`
   `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings`
4. wenn du den installer anfässt:
   `.\scripts\build-installer.ps1`
5. release-flow und artifact-pfade stehen in:
   `docs/release/INSTALLER.md`
