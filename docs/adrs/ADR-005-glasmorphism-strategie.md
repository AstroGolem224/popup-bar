# ADR-005: Glasmorphism-Rendering-Strategie

**Status:** Akzeptiert  
**Datum:** 2026-03-12  
**Entscheidungsträger:** Projektteam  
**Letzte Überprüfung:** 2026-03-12

---

## Kontext

Das zentrale visuelle Merkmal von Popup Bar ist ein **Glassmorphism-Effekt**: Die Bar soll halbtransparent erscheinen und den Hintergrundinhalt (Desktop, offene Fenster) weichgezeichnet durchscheinen lassen. Dieser Effekt ist der primäre Differenzierungsfaktor des UI-Designs gegenüber opaken Utility-Bars.

Die technische Herausforderung: Der Effekt muss plattformübergreifend (Windows, macOS, Linux) visuell konsistent und performant sein — auch wenn jedes Betriebssystem fundamental unterschiedliche APIs für Fenster-Transparenz und Compositing bereitstellt.

Anforderungen:
- Echter Blur-Effekt hinter der Bar (nicht nur Transparenz)
- Konsistentes Erscheinungsbild auf allen Plattformen (OS-konsistenter Stil)
- 60fps-Animation beim Slide-In/Out ohne sichtbares Ruckeln
- Kein weißer Flash beim ersten Rendern (WebView-Initialisierung)
- Graceful Degradation: Auch ohne Compositor-Unterstützung (z.B. Linux ohne Compositor) akzeptabler Fallback

Evaluierte Alternativen: **Rein CSS `backdrop-filter`**, **Canvas-basiertes Rendering**, **Pre-rendered Background Capture**, **Native OS-APIs (Vibrancy/Acrylic) + CSS-Ergänzung**.

---

## Entscheidung

Wir verwenden eine **kombinierte Strategie**: Native OS-Transparenz-APIs (Acrylic auf Windows, NSVisualEffectView auf macOS) stellen den Kern-Blur-Effekt bereit, ergänzt durch CSS `backdrop-filter` und sorgfältig abgestimmte `rgba`-Hintergründe. Plattform-spezifische Klassen (`platform-windows`, `platform-macos`, `platform-linux`) erlauben differenziertes CSS-Styling.

---

## Begründung

### Alternative 1: Rein CSS `backdrop-filter`

```css
/* Nur CSS — kein native API */
.shelf-bar {
  background: rgba(15, 15, 20, 0.3);
  backdrop-filter: blur(20px) saturate(180%);
  -webkit-backdrop-filter: blur(20px) saturate(180%);
}
```

**Probleme:**

- **Kein echter Blur ohne Fenster-Transparenz:** `backdrop-filter` funktioniert nur, wenn das Fenster selbst transparent ist (d.h. der HTML-Hintergrund muss `transparent` sein und das OS-Fenster muss Transparenz erlauben). Ohne native OS-Transparenz-API ist der WebView-Hintergrund weiß — `backdrop-filter` hat dann keinen Effekt auf das Desktopbild.

- **Performance auf Linux (WebKitGTK):** `backdrop-filter` via `backdrop-filter: blur()` auf WebKitGTK ist bekannt für starke GPU-Auslastung. Ohne Hardware-Compositing degradiert es auf Software-Rendering — sichtbares Ruckeln bei der Slide-Animation.

- **Browser-interne Grenzen:** CSS `backdrop-filter` blurt nur Inhalte *innerhalb* des WebView. Es sieht das Desktopbild oder andere Fenster-Inhalte hinter dem Tauri-Fenster nicht, wenn das Fenster nicht korrekt als transparentes Compositing-Layer konfiguriert ist.

**Fazit:** CSS allein reicht nicht. Fenster-Transparenz via OS-API ist Voraussetzung.

### Alternative 2: Canvas-basiertes Rendering

```typescript
// ABGELEHNT
function BlurredBackground() {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    // Periodisch Screenshot der Fläche hinter dem Fenster aufnehmen
    // Via Tauri Command: capture_screen_region(x, y, w, h) -> PNG
    // Auf Canvas zeichnen + StackBlur-Algorithmus in JS anwenden
    const interval = setInterval(async () => {
      const imageData = await invoke<Uint8Array>('capture_screen_region', {
        x: barX, y: barY, width: barWidth, height: barHeight
      });
      drawBlurred(canvasRef.current!, imageData);
    }, 33); // 30fps

    return () => clearInterval(interval);
  }, []);

  return <canvas ref={canvasRef} style={{ position: 'absolute', zIndex: -1 }} />;
}
```

**Probleme:**
- **Performance-Katastrophe:** Periodisches Screen-Capture (auch 30fps) erfordert `BitBlt`/`CGWindowListCreateImage` — kostspielige Kernel-Operationen. CPU/GPU-Last wäre dauerhaft hoch.
- **Latenz:** Screenshot → IPC → Canvas-Zeichnen → JS-Blur → Render erzeugt 50–100ms Latenz. Bei Scroll-Bewegungen hinter der Bar sieht man das gescroltzte Bild zeitverzögert im Blur — `Ghosting`-Artefakt.
- **Batterielaufzeit:** Dauerhafte Screen-Captures auf Laptops treiben den Akku.
- **Tauri hat keine `capture_screen_region`-API:** Müsste als plattformspezifisches OS-Command implementiert werden — erheblicher Aufwand.

**Fazit: Inakzeptabel aus Performance-Gründen.**

### Alternative 3: Pre-rendered Background Capture (Einmaliges Snapshot)

```typescript
// ABGELEHNT — Variation von Alternative 2
// Nur einmal beim Öffnen der Bar ein Screenshot aufnehmen
```

Dasselbe Grundproblem wie Alternative 2, nur weniger schlimm. Aber: Der gecachte Background veraltet sofort, wenn sich dahinter liegende Fenster bewegen. Das Ergebnis ist ein eingefrorenes Hintergrundbild, das nicht zum tatsächlichen Inhalt passt — schlechter als Transparenz ohne Blur.

**Fazit: Artefakt-beladene UX, kein echter Glassmorphism.**

### Gewählte Lösung: Native OS-APIs + CSS

Der korrekte Ansatz ist, das OS-eigene Compositing zu nutzen:

- **Windows:** `DwmExtendFrameIntoClientArea` + Acrylic (Windows 10/11) via `window-vibrancy`-Crate. Das DWM (Desktop Window Manager) übernimmt den Blur im Compositor — null CPU-Overhead im App-Prozess.

- **macOS:** `NSVisualEffectView` mit `material: .hudWindow` oder `.sidebar`. macOS' Core Animation Compositor rendert den Blur nativ auf der GPU. Kein CSS nötig.

- **Linux:** Hängt vom Compositor ab. Unter KWin/Mutter mit aktivierten Compositing-Effekten: CSS `backdrop-filter` funktioniert über WebKitGTK mit GPU-Beschleunigung. Ohne Compositor: Fallback auf opake `rgba`-Farbe.

```rust
// modules/platform/mod.rs
pub fn apply_window_transparency(window: &WebviewWindow) -> Result<(), PlatformError> {
    #[cfg(target_os = "windows")]
    {
        use window_vibrancy::apply_acrylic;
        // Dunkler Acrylic-Hintergrund: RGBA (18, 18, 18, 125) — leicht getönt
        apply_acrylic(window, Some((18, 18, 18, 125)))
            .map_err(|e| PlatformError::WindowTransparencyFailed(e.to_string()))?;
    }

    #[cfg(target_os = "macos")]
    {
        use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
        apply_vibrancy(
            window,
            NSVisualEffectMaterial::HudWindow,
            None,   // appearance: auto (system theme)
            Some(16.0), // corner_radius
        ).map_err(|e| PlatformError::WindowTransparencyFailed(e.to_string()))?;
    }

    // Linux: Transparenz wird über tauri.conf.json "transparent: true" gesetzt
    // Compositor übernimmt Blur falls verfügbar; CSS backdrop-filter als Ergänzung

    Ok(())
}
```

### CSS-Schichtung: OS-Blur + CSS-Verfeinerung

Der native Blur ist die Grundlage. CSS fügt visuelle Verfeinerung hinzu:

```css
/* src/styles/glassmorphism.css */

/* ═══════════════════════════════════════════════════
   Basis: Gilt für alle Plattformen als Fallback
   ═══════════════════════════════════════════════════ */
.shelf-bar {
  /* Basis-Transparenz — OS-Blur macht den eigentlichen Effekt */
  background: rgba(12, 12, 18, 0.70);

  /* Subtile Glasrand-Imitation */
  border: 1px solid rgba(255, 255, 255, 0.10);
  border-top: none; /* Bar kommt von oben — kein Border oben */
  border-radius: 0 0 16px 16px;

  /* Tiefenwirkung */
  box-shadow:
    0 8px 32px rgba(0, 0, 0, 0.45),
    0 2px 8px rgba(0, 0, 0, 0.25),
    inset 0 1px 0 rgba(255, 255, 255, 0.07); /* Obere Lichtkante */

  /* CSS-Blur als Ergänzung (wo unterstützt) */
  @supports (backdrop-filter: blur(1px)) {
    background: rgba(12, 12, 18, 0.50);
    backdrop-filter: blur(12px) saturate(160%) brightness(0.9);
    -webkit-backdrop-filter: blur(12px) saturate(160%) brightness(0.9);
  }
}

/* ═══════════════════════════════════════════════════
   Windows: Acrylic ist aktiv — CSS reduzieren
   ═══════════════════════════════════════════════════ */
.platform-windows .shelf-bar {
  /* Acrylic bringt seinen eigenen Blur — CSS nur für Tönungsfarbe */
  background: rgba(18, 18, 24, 0.35);
  backdrop-filter: brightness(0.9); /* Kein blur — Acrylic macht das */
}

/* ═══════════════════════════════════════════════════
   macOS: Vibrancy aktiv — fast transparenter Hintergrund
   ═══════════════════════════════════════════════════ */
.platform-macos .shelf-bar {
  /* NSVisualEffectView rendert alles — nur eine Hauch Tint nötig */
  background: rgba(0, 0, 0, 0.15);
  backdrop-filter: brightness(0.95);
  border-color: rgba(255, 255, 255, 0.08);
}

/* ═══════════════════════════════════════════════════
   Linux mit Compositor: CSS-Blur aktiv
   ═══════════════════════════════════════════════════ */
.platform-linux-compositor .shelf-bar {
  background: rgba(12, 12, 18, 0.55);
  backdrop-filter: blur(16px) saturate(150%);
}

/* ═══════════════════════════════════════════════════
   Linux ohne Compositor: Opaker Fallback
   ═══════════════════════════════════════════════════ */
.platform-linux-no-compositor .shelf-bar {
  background: rgba(18, 18, 24, 0.97);
  backdrop-filter: none;
  border-color: rgba(255, 255, 255, 0.15);
}

/* ═══════════════════════════════════════════════════
   Accessibility: Reduced Motion + Reduced Transparency
   ═══════════════════════════════════════════════════ */
@media (prefers-reduced-transparency: reduce) {
  .shelf-bar {
    background: rgba(18, 18, 24, 0.97) !important;
    backdrop-filter: none !important;
  }
}
```

### Plattform-Klasse beim Start setzen

```rust
// commands/system_commands.rs
#[tauri::command]
pub fn get_platform_class() -> String {
    #[cfg(target_os = "windows")]
    return "platform-windows".to_string();

    #[cfg(target_os = "macos")]
    return "platform-macos".to_string();

    #[cfg(target_os = "linux")]
    {
        // Compositor via D-Bus abfragen
        let has_compositor = detect_linux_compositor();
        if has_compositor {
            return "platform-linux-compositor".to_string();
        } else {
            return "platform-linux-no-compositor".to_string();
        }
    }
}
```

```typescript
// src/App.tsx
useEffect(() => {
  invoke<string>('get_platform_class').then(cls => {
    document.body.classList.add(cls);
  });
}, []);
```

---

## Performance-Analyse

| Approach | CPU idle | GPU idle | Animation FPS | Blur-Qualität |
|----------|----------|----------|---------------|---------------|
| Native Vibrancy (macOS) | < 0.1% | Compositor (0%) | 120fps | Exzellent (OS-native) |
| Native Acrylic (Windows 11) | < 0.1% | DWM (0%) | 60fps | Sehr gut |
| CSS backdrop-filter (Linux/KWin) | 0.2–0.5% | GPU-accelerated | 60fps | Gut |
| CSS backdrop-filter (WebKitGTK, kein GPU) | 2–5% | Software | 30fps | Mittel |
| Canvas-Screenshot-Blur | 15–25% | Hoch | 30fps (ruckelnd) | Schlecht (Artefakte) |
| Opaker Fallback | 0% | 0% | 60fps | Kein Blur |

---

## Konsequenzen

### Positiv

- **Beste visuelle Qualität:** Nativer OS-Blur ist dem CSS-Blur überlegen — er nutzt das OS-Compositor-System und reagiert in Echtzeit auf Hintergrundänderungen
- **OS-konsistenter Look:** Vibrancy (macOS) und Acrylic (Windows) passen sich dem System-Theme (Hell/Dunkel) automatisch an
- **Minimaler CPU/GPU-Overhead:** OS-Compositor macht die Arbeit — kein App-seitiger Performance-Aufwand
- **Graceful Degradation:** Opaker Fallback ist auf allen Plattformen definiert und aktiviert sich automatisch
- **`prefers-reduced-transparency`-Support:** Accessibility-Nutzer erhalten einen opaken Hintergrund

### Negativ

- **`window-vibrancy`-Dependency:** Externe Crate, die Windows und macOS-APIs wrappt. Wenn Tauri eine eigene API dafür einführt, ist eine Migration nötig. Mitigation: Wrapper in `platform/`-Modul isoliert (ADR-004).
- **Inkonsistenz zwischen Plattformen nicht vollständig vermeidbar:** macOS Vibrancy sieht fundamental anders aus als Windows Acrylic. Das ist jedoch gewollt — OS-konsistentes Design ist die Intention.
- **Linux-Variabilität:** Je nach Compositor (KWin, Mutter, ohne) sieht die Bar unterschiedlich aus. Mitigation: Klare Fallback-Kette und automatische Compositor-Erkennung.
- **Kein Blur auf Linux ohne Compositor:** Betroffen sind minimale Linux-Setups (i3, sway ohne Blur-Plugin). Mitigation: Dokumentation; Nutzer können Blur via Settings deaktivieren.

---

## Verwandte Entscheidungen

- ADR-001: Tauri 2 wurde u.a. gewählt, weil es native Fenster-Transparenz für Glassmorphism erlaubt
- ADR-004: `apply_window_transparency()` ist eine Methode des `PlatformProvider`-Traits

---

*ADR-005 — Glasmorphism-Strategie — Akzeptiert 2026-03-12*
