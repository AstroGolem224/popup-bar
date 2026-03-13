## Smoke-Test Phase 6 — Popup Bar

**Ziel:** Manuelle End-to-End-Checks für einen Release-Build auf Windows, macOS und Linux.

- **Hotzone & Sichtbarkeit**
  - [ ] Hotzone-Trigger funktioniert nach System-Neustart
  - [ ] Slide-In/Out Animation läuft ohne sichtbares Ruckeln
- **Autostart & Tray**
  - [ ] App startet automatisch bei aktiviertem Autostart
  - [ ] Deaktivierter Autostart wird nach Neustart respektiert
- **Drag & Drop & Reorder**
  - [ ] Drag & Drop von `.pdf` aus Downloads auf die Bar
  - [ ] Drag & Drop von `.app`-Bundle (macOS) bzw. `.lnk`/`.exe` (Windows) auf die Bar
  - [ ] URL-Drop aus Chrome/Firefox/Safari
  - [ ] Items persistieren nach App-Neustart
  - [ ] Reorder-Drag funktioniert stabil (kein Item „verschwindet“)
- **Settings & Multi-Monitor**
  - [ ] Settings-Änderungen (Blur, Animation, Hotzone) werden sofort sichtbar
  - [ ] „Nur Primärmonitor“ zeigt Bar ausschließlich auf dem Primärmonitor
  - [ ] Mit Multi-Monitor aktiv erscheint die Bar auf dem erwarteten Monitor
- **Performance & Stabilität**
  - [ ] RAM-Verbrauch im Idle-Zustand < 50 MB
  - [ ] CPU-Auslastung im Idle < 0.5 %
  - [ ] Kein Absturz oder Panic in einem 24h-Dauertest

