## Beitrag leisten

- Repo klonen und Dependencies installieren (`npm install`, Rust Toolchain stable)
- Vor Änderungen:
  - `cd src-tauri && cargo test`
  - `npm run build`
- Neues Feature:
  - Architektur‑Plan in `docs/architecture/IMPLEMENTATION_PLAN.md` prüfen
  - Falls Scope ändert: Plan + `NEXT_PHASE_REVIEW.md` mit anpassen
- Vor Pull‑Request:
  - Alle Tests grün (Rust + Frontend)
  - `npm run build` ohne Warnungen

