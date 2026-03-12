# ADR-004: PlatformProvider-Trait für OS-spezifische Abstraktion

**Status:** Akzeptiert  
**Datum:** 2026-03-12  
**Entscheidungsträger:** Projektteam  
**Letzte Überprüfung:** 2026-03-12

---

## Kontext

Popup Bar führt mehrere Operationen durch, die je nach Betriebssystem grundlegend verschieden implementiert sein müssen:

| Operation | Windows | macOS | Linux |
|-----------|---------|-------|-------|
| Globale Maus-Hooks | `SetWindowsHookEx` (WH_MOUSE_LL) | `CGEventTap` | `XInput2` / Wayland-Fallback |
| Fenster-Transparenz | `DwmExtendFrameIntoClientArea` / Acrylic | `NSVisualEffectView` (Vibrancy) | Compositor (KWin/Mutter) |
| Icon-Extraktion | `SHGetFileInfo` + HICON | `NSWorkspace.icon(forFile:)` | Freedesktop.org + GTK |
| Autostart | Registry `HKCU\Run` | Launch Agent plist | `.config/autostart/` |
| Datei öffnen | `ShellExecuteW` | `NSWorkspace.openURL` | `xdg-open` |
| Accessibility-Rechte | Nicht nötig | `AXIsProcessTrustedWithOptions` | Nicht nötig |
| Monitor-Erkennung | `EnumDisplayMonitors` | `NSScreen.screens` | `XRandR` / Wayland |

Diese OS-spezifischen Code-Pfade sind ein zentrales Merkmal der Anwendung. Die Entscheidung, wie sie strukturiert werden, hat weitreichende Auswirkungen auf Wartbarkeit, Testbarkeit und Erweiterbarkeit.

Evaluierte Alternativen: **`if cfg!(target_os)` überall im Code**, **Separate Crates/Feature-Flags**, **`PlatformProvider`-Trait mit Compile-Time-Dispatch**.

---

## Entscheidung

Wir definieren ein **`PlatformProvider`-Trait** in `src-tauri/src/modules/platform/mod.rs`. Die OS-spezifischen Implementierungen befinden sich in separaten Dateien (`windows.rs`, `macos.rs`, `linux.rs`) und werden via `#[cfg(target_os = ...)]`-Guards zur Compile-Zeit eingebunden.

```
src-tauri/src/modules/platform/
├── mod.rs        ← PlatformProvider trait + pub fn create_platform_provider()
├── windows.rs    ← WindowsPlatform: impl PlatformProvider
├── macos.rs      ← MacOSPlatform:   impl PlatformProvider
└── linux.rs      ← LinuxPlatform:   impl PlatformProvider
```

Die übrigen Module (`hotzone.rs`, `icon_resolver.rs`, etc.) erhalten den `PlatformProvider` als generischen Parameter oder als `Box<dyn PlatformProvider>` und rufen ausschließlich Trait-Methoden auf — kein direktes `cfg!` in der Business-Logik.

---

## Begründung

### Alternative 1: `if cfg!(target_os)` direkt in der Business-Logik

```rust
// ABGELEHNT — Beispiel, wie es NICHT gemacht werden soll
pub fn setup_mouse_hook(app: AppHandle) {
    if cfg!(target_os = "windows") {
        unsafe {
            let hook = SetWindowsHookExW(WH_MOUSE_LL, Some(low_level_mouse_proc), None, 0);
            // ... 40 Zeilen Win32-Code ...
        }
    } else if cfg!(target_os = "macos") {
        // ... 30 Zeilen CoreGraphics-Code ...
    } else {
        // ... 50 Zeilen X11-Code ...
    }
}
```

**Probleme:**
- **Unleserliche Business-Logik:** Die Kern-Logik (`HotzoneTracker`) ist mit OS-Details verschmutzt
- **Schwer testbar:** Um `setup_mouse_hook` zu testen, braucht man den kompletten OS-Kontext; kein Mocking möglich
- **Schlechte Erweiterbarkeit:** Jedes neue OS-Feature erfordert das Durchsuchen aller Funktionen nach `cfg!`-Blöcken
- **Compile-Fehler auf falschen Plattformen:** `#[cfg]`-Blöcke schützen nur den Inhalt, nicht die Funktion — unaufmerksame Fehler können zu Code führen, der auf einer Plattform nicht kompiliert

### Alternative 2: Separate Crates mit Feature-Flags

```toml
# Cargo.toml — ABGELEHNT
[features]
platform-windows = ["dep:windows"]
platform-macos   = ["dep:objc2", "dep:core-graphics"]
platform-linux   = ["dep:x11rb", "dep:gtk"]
```

```rust
// lib.rs
#[cfg(feature = "platform-windows")]
mod platform_windows;
```

**Probleme:**
- **Komplexere Build-Konfiguration:** Feature-Flags müssen für jede CI-Plattform korrekt gesetzt werden; Fehler sind schwer zu debuggen
- **Kein einheitliches Interface erzwungen:** Feature-Flags garantieren nicht, dass alle Plattformen dieselben Funktionen implementieren — ein vergessenes `impl` führt zu einem Linker-Fehler statt einem klaren Compiler-Fehler
- **Kein Mocking möglich:** Für Tests braucht man trotzdem eine Abstraktion

### Gewählte Lösung: `PlatformProvider`-Trait

```rust
// src-tauri/src/modules/platform/mod.rs

/// Plattform-Abstraktion: Alle OS-spezifischen Operationen gehen über dieses Trait.
/// Der Rust-Compiler erzwingt, dass alle drei Implementierungen alle Methoden bereitstellen.
pub trait PlatformProvider: Send + Sync {
    // --- Maus-Hooks ---
    fn setup_mouse_hook(
        &self,
        callback: Arc<dyn Fn(HotzoneEvent) + Send + Sync>,
    ) -> Result<HookHandle, PlatformError>;

    fn teardown_mouse_hook(&self, handle: HookHandle) -> Result<(), PlatformError>;

    // --- Fenster-Transparenz ---
    fn apply_window_transparency(
        &self,
        window: &tauri::WebviewWindow,
    ) -> Result<(), PlatformError>;

    // --- Icon-Extraktion ---
    fn extract_icon(
        &self,
        path: &std::path::Path,
    ) -> Result<Vec<u8>, PlatformError>; // PNG-Bytes

    // --- System-Integration ---
    fn open_path_with_default_app(
        &self,
        path: &std::path::Path,
    ) -> Result<(), PlatformError>;

    fn request_accessibility_permission(&self) -> AccessibilityStatus;

    fn get_monitors(&self) -> Result<Vec<MonitorInfo>, PlatformError>;
}

/// Factory-Funktion: Gibt zur Compile-Zeit die korrekte Implementierung zurück.
pub fn create_platform_provider() -> Box<dyn PlatformProvider> {
    #[cfg(target_os = "windows")]
    return Box::new(windows::WindowsPlatform::new());

    #[cfg(target_os = "macos")]
    return Box::new(macos::MacOSPlatform::new());

    #[cfg(target_os = "linux")]
    return Box::new(linux::LinuxPlatform::detect());
}

// Hilfs-Typen
pub struct HookHandle(pub u64);

pub struct MonitorInfo {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    pub is_primary: bool,
    pub scale_factor: f64,
}

pub enum AccessibilityStatus {
    Granted,
    Denied,
    NotRequired, // Windows, Linux
}

#[derive(Debug, thiserror::Error)]
pub enum PlatformError {
    #[error("OS-Hook konnte nicht installiert werden: {0}")]
    HookInstallFailed(String),
    #[error("Icon-Extraktion fehlgeschlagen für {path}: {reason}")]
    IconExtractionFailed { path: String, reason: String },
    #[error("Accessibility-Berechtigung nicht erteilt")]
    AccessibilityDenied,
    #[error("Plattform-Funktion nicht unterstützt: {0}")]
    Unsupported(String),
}
```

**Vorteile dieses Musters:**

**Compile-Time-Vollständigkeitsprüfung:**
Wenn `LinuxPlatform` eine Methode des Traits nicht implementiert, gibt der Rust-Compiler einen klaren Fehler:
```
error[E0277]: the trait bound `LinuxPlatform: PlatformProvider` is not satisfied
   --> src/modules/platform/linux.rs:42:6
    |
42  | impl PlatformProvider for LinuxPlatform {
    |      ^^^^^^^^^^^^^^^^ missing `extract_icon` in implementation
```

**Testbarkeit via Mock-Provider:**
Business-Logik-Tests können einen `MockPlatformProvider` injizieren:

```rust
// tests/hotzone_tests.rs
struct MockPlatform {
    emitted_events: Arc<Mutex<Vec<HotzoneEvent>>>,
}

impl PlatformProvider for MockPlatform {
    fn setup_mouse_hook(
        &self,
        callback: Arc<dyn Fn(HotzoneEvent) + Send + Sync>,
    ) -> Result<HookHandle, PlatformError> {
        // Simuliert einen Maus-Event nach 10ms
        let cb = callback.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(10));
            cb(HotzoneEvent::Enter { cursor_x: 500, monitor_id: 0 });
        });
        Ok(HookHandle(42))
    }

    fn extract_icon(&self, _path: &Path) -> Result<Vec<u8>, PlatformError> {
        // Gibt immer ein 1x1 transparentes PNG zurück
        Ok(include_bytes!("../fixtures/test-icon.png").to_vec())
    }

    // ... alle anderen Methoden mit sinnvollen Test-Defaults ...
}

#[tokio::test]
async fn test_hotzone_tracker_emits_enter_event() {
    let platform = Arc::new(MockPlatform::new());
    let tracker = HotzoneTracker::new(platform, HotzoneConfig::default());
    let events = tracker.subscribe();

    tracker.start().await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    assert_eq!(events.try_recv().unwrap(), HotzoneEvent::Enter { cursor_x: 500, monitor_id: 0 });
}
```

**Klare Trennung von Business-Logik und OS-Code:**
`hotzone.rs` kennt kein Win32, kein AppKit, kein GTK — nur `PlatformProvider`-Methoden. Das macht den Code erheblich lesbarer:

```rust
// modules/hotzone.rs — sauber, kein OS-Code
pub struct HotzoneTracker {
    platform: Arc<dyn PlatformProvider>,
    config: HotzoneConfig,
    state: Arc<Mutex<HotzoneState>>,
    app: AppHandle,
}

impl HotzoneTracker {
    pub fn start(&self) -> Result<(), PlatformError> {
        let app = self.app.clone();
        let config = self.config.clone();
        let state = self.state.clone();

        let callback = Arc::new(move |event: HotzoneEvent| {
            match event {
                HotzoneEvent::Enter { .. } => {
                    app.emit("hotzone:enter", ()).unwrap();
                }
                HotzoneEvent::Leave => {
                    // Debounce via tokio::time::sleep
                    app.emit("hotzone:leave", ()).unwrap();
                }
            }
        });

        self.platform.setup_mouse_hook(callback)?;
        Ok(())
    }
}
```

---

## Implementierungsdetails

### Windows-Implementierung (Auszug)

```rust
// modules/platform/windows.rs
#[cfg(target_os = "windows")]
pub struct WindowsPlatform;

#[cfg(target_os = "windows")]
impl PlatformProvider for WindowsPlatform {
    fn setup_mouse_hook(
        &self,
        callback: Arc<dyn Fn(HotzoneEvent) + Send + Sync>,
    ) -> Result<HookHandle, PlatformError> {
        use windows::Win32::UI::WindowsAndMessaging::{
            SetWindowsHookExW, WH_MOUSE_LL, CallNextHookEx, UnhookWindowsHookEx,
        };
        // CALLBACK_FN in Thread-Local speichern, Hook installieren
        todo!("Win32 Low-Level Mouse Hook")
    }

    fn apply_window_transparency(
        &self,
        window: &tauri::WebviewWindow,
    ) -> Result<(), PlatformError> {
        window_vibrancy::apply_acrylic(window, Some((18, 18, 18, 125)))
            .map_err(|e| PlatformError::HookInstallFailed(e.to_string()))
    }

    fn request_accessibility_permission(&self) -> AccessibilityStatus {
        AccessibilityStatus::NotRequired // Windows braucht keine explizite Permission
    }
}
```

### macOS-Implementierung (Auszug)

```rust
// modules/platform/macos.rs
#[cfg(target_os = "macos")]
pub struct MacOSPlatform;

#[cfg(target_os = "macos")]
impl PlatformProvider for MacOSPlatform {
    fn request_accessibility_permission(&self) -> AccessibilityStatus {
        // AXIsProcessTrustedWithOptions(nil) aufrufen
        todo!("objc2 AX API")
    }

    fn apply_window_transparency(
        &self,
        window: &tauri::WebviewWindow,
    ) -> Result<(), PlatformError> {
        window_vibrancy::apply_vibrancy(
            window,
            window_vibrancy::NSVisualEffectMaterial::HudWindow,
            None,
            None,
        ).map_err(|e| PlatformError::HookInstallFailed(e.to_string()))
    }
}
```

---

## Konsequenzen

### Positiv

- **Compile-Zeit-Sicherheit:** Der Rust-Compiler erzwingt vollständige Trait-Implementierungen
- **Vollständige Testbarkeit:** Business-Logik ist via `MockPlatformProvider` ohne echte OS-APIs testbar
- **Klare Modul-Grenzen:** OS-Code ist in `platform/`-Unterordner isoliert; alle anderen Module sind plattform-agnostisch
- **Einfache Erweiterbarkeit:** Neue Plattform (z.B. FreeBSD) = neue Datei + `impl PlatformProvider`
- **Lesbarer Business-Code:** `hotzone.rs`, `icon_resolver.rs` etc. sind frei von OS-Spezifika

### Negativ

- **Trait-Objekt-Overhead:** `Box<dyn PlatformProvider>` hat minimal höheren Laufzeit-Overhead als direkter Aufruf (vtable dispatch). In der Praxis irrelevant — Maus-Hook-Callbacks haben ms-Granularität.
- **Trait-Design ist schwierig:** Das Trait muss den Lowest Common Denominator aller Plattformen abbilden. Plattform-spezifische Features (z.B. macOS-spezifische Vibrancy-Materialien) müssen als optionale `Option<...>`-Rückgaben modelliert werden.
- **Boilerplate:** Drei Implementierungs-Dateien für jede neue Methode. Mitigation: Wenn eine Plattform eine Funktion nicht unterstützt, gibt `Err(PlatformError::Unsupported(...))` zurück.

---

## Verwandte Entscheidungen

- ADR-001: Rust-Backend macht dieses Trait-Pattern erst möglich; in Electron würde Node.js-Addon-Struktur genutzt
- ADR-003: Der `PlatformProvider` ist der Eintrittspunkt für OS-Events in das Event-Driven-System

---

*ADR-004 — PlatformProvider-Trait — Akzeptiert 2026-03-12*
