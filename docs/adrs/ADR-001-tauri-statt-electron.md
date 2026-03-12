# ADR-001: Tauri 2 statt Electron

**Status:** Akzeptiert  
**Datum:** 2026-03-12  
**Entscheidungsträger:** Projektteam  
**Letzte Überprüfung:** 2026-03-12

---

## Kontext

Wir entwickeln Popup Bar — eine Desktop-Utility-Anwendung, die permanent im Hintergrund läuft und bei Bedarf eine schwebende Leiste am oberen Bildschirmrand einblendet. Die App ist ein „always-on"-Tool: Sie ist vom Systemstart an aktiv und verbraucht Ressourcen, auch wenn der Nutzer sie gerade nicht aktiv benutzt.

Daraus ergeben sich besondere Anforderungen an das gewählte Framework:

- **Minimaler Ressourcenverbrauch:** Die App darf den Nutzer nicht durch Speicher- oder CPU-Verbrauch stören
- **Native OS-Integration:** Globale Maus-Hooks, Fenster-Transparenz, Icon-Extraktion und Autostart erfordern tiefe OS-API-Zugriffe
- **Fenster-Transparenz:** Glassmorphism-Effekte (backdrop-filter, native Vibrancy / Acrylic) sind ein zentrales UI-Merkmal
- **Cross-Platform:** Windows 10/11, macOS 12+ und Linux (X11/Wayland) müssen aus einer Codebasis bedient werden
- **Bundle-Größe:** Als kleines Utility-Tool ist eine minimale Installationsgröße wünschenswert

Evaluierte Alternativen: **Electron**, **Flutter Desktop**, **Qt (via Rust-Bindings)**, **Tauri 2**.

---

## Entscheidung

Wir verwenden **Tauri 2** mit einem Rust-Backend und einem WebView-Frontend (React + TypeScript).

---

## Begründung

### Vergleichsmatrix: Frameworks im Überblick

| Kriterium | Tauri 2 | Electron | Flutter Desktop | Qt (Rust FFI) |
|-----------|---------|----------|-----------------|---------------|
| **Bundle-Größe** | ~5–8 MB | ~150–200 MB | ~15–25 MB | ~10–20 MB |
| **RAM idle** | ~25–40 MB | ~100–150 MB | ~50–80 MB | ~20–40 MB |
| **CPU idle** | < 0.1% | 0.5–2% | < 0.2% | < 0.1% |
| **Native API-Zugriff** | Rust FFI (exzellent) | Node.js (gut) | Dart FFI (gut) | C++ direkt (exzellent) |
| **Fenster-Transparenz** | Native (Vibrancy / Acrylic) | Eingeschränkt | Eingeschränkt | Gut |
| **WebView-Rendering** | OS-nativ (Edge/WebKit/WebKitGTK) | Chromium gebündelt | Eigene Rendering-Engine | WebEngine optional |
| **Sicherheit** | Rust memory-safe, kein Node.js Attack-Surface | Node.js + Chromium, breitere Attack-Surface | Dart VM | C++ memory-unsafe |
| **Ecosystem-Reife** | Wachsend (v2 seit 2024) | Sehr reif (seit 2013) | Wachsend (Desktop seit 2021) | Sehr reif |
| **Entwickler-Erfahrung (Web)** | Gut (HTML/CSS/JS Frontend) | Sehr gut | Mittel (Dart lernen) | Schlecht |
| **Tauri-Plugin-Ökosystem** | Vorhanden (sql, shell, autostart) | npm-Ökosystem sehr groß | Pub.dev | Qt-Module |
| **Lernkurve** | Mittel (Rust) | Niedrig (Node.js) | Mittel (Dart) | Hoch (C++) |

### Entscheidende Faktoren für Tauri 2

**1. Ressourcenverbrauch ist kritisch:**
Popup Bar ist ein Hintergrundprozess. Ein Tool, das dauerhaft 150 MB RAM belegt (Electron), ist für ein einfaches Utility-Tool nicht akzeptabel. Tauri nutzt die vom OS bereits geladene WebView-Engine (Edge WebView2 auf Windows, WebKit auf macOS), was den RAM-Bedarf drastisch reduziert.

**2. Rust ermöglicht nativen OS-Zugriff:**
Die Hotzone-Detection erfordert Low-Level-Maus-Hooks (`SetWindowsHookEx`, `CGEventTap`, `XInput2`). Rust bietet über FFI direkten Zugriff auf diese APIs ohne Performance-Overhead. In Electron müsste man ein natives Node.js-Addon schreiben — zusätzliche Komplexität.

**3. Native Fenster-Transparenz:**
Das Glassmorphism-Design erfordert native `NSVisualEffectView` (macOS) und Acrylic/Mica (Windows). Tauri 2 unterstützt dies über `window-vibrancy`-Crate nativ. Electron hat diese Feature-Parität nicht vollständig erreicht.

**4. Sicherheit durch Rust:**
Rust's Ownership-Modell eliminiert ganze Klassen von Bugs (Use-after-Free, Buffer-Overflows). Da Popup Bar tiefen OS-Zugriff benötigt (Maus-Hooks, Dateisystem), ist memory-safety besonders wichtig.

**5. Bundle-Größe:**
~5 MB vs. ~150 MB ist für eine einfache Utility-App ein überzeugender Unterschied für die User Experience beim Download und der Installation.

### Warum nicht Flutter Desktop?

Flutter hat eine eigene Rendering-Engine (Skia/Impeller), die keine nativen OS-Komponenten nutzt. Fenster-Transparenz mit nativen Effekten (Vibrancy, Acrylic) ist schwer zu implementieren. Außerdem müsste das Team Dart lernen, während Web-Technologien (HTML/CSS/TypeScript) bereits bekannt sind.

### Warum nicht Qt?

Qt hätte exzellente native Integration, aber die Kombination aus C++-Entwicklung und komplexer Qt-Lizenzierung (LGPL vs. kommerzielle Lizenz) macht es unattraktiv. Die Rust-Qt-Bindings sind nicht ausreichend ausgereift für Produktionseinsatz.

---

## Konsequenzen

### Positiv

- **Deutlich kleinerer Footprint:** ~5 MB Bundle, ~30 MB RAM idle — akzeptabel für ein always-on Tool
- **Rust-Performance für OS-nahe Operationen:** Maus-Hooks, Icon-Extraktion und SQLite-Operationen laufen nativ
- **Bessere Sicherheit:** Kein Node.js-Prozess, kein Chromium mit erweiterter Attack-Surface
- **Native Transparenz-Effekte:** `window-vibrancy`-Crate bietet Vibrancy und Acrylic out-of-the-box
- **Web-Technologien für UI:** React + TypeScript sind dem Team bekannt; UI-Entwicklung bleibt produktiv

### Negativ

- **Kleinere Community als Electron:** Weniger StackOverflow-Antworten, weniger fertige Plugins. Mitigation: Tauri-Community wächst schnell; Discord aktiv.
- **WebView-Inkonsistenzen:** Edge WebView2 (Windows), WebKit (macOS) und WebKitGTK (Linux) unterscheiden sich in CSS-Feature-Support (z.B. `backdrop-filter`-Performance auf WebKitGTK). Mitigation: Plattform-spezifische CSS-Fallbacks.
- **Rust-Lernkurve:** Für reine Web-Entwickler ist Rust-Backend eine Hürde. Mitigation: Klare Modul-Grenzen; Frontend-Entwickler müssen nur `commands/`-Signaturen verstehen, nicht Rust-Details.
- **Tauri 2 noch relativ jung:** Potenzielle breaking changes in zukünftigen Versionen. Mitigation: `Cargo.lock`-Pinning; Tauri-Version in CI fixieren (→ ADR-005).

---

## Verwandte Entscheidungen

- ADR-003 beschreibt das Event-Driven-Architekturmuster, das durch Tauris Event-Bus ermöglicht wird
- ADR-004 beschreibt die `PlatformProvider`-Trait-Abstraktion, die von Tauri's Rust-Backend abhängt

---

*ADR-001 — Tauri 2 statt Electron — Akzeptiert 2026-03-12*
