# ADR-003: Event-Driven Architektur für Rust–Frontend-Kommunikation

**Status:** Akzeptiert  
**Datum:** 2026-03-12  
**Entscheidungsträger:** Projektteam  
**Letzte Überprüfung:** 2026-03-12

---

## Kontext

Popup Bar besitzt eine fundamentale architektonische Spannung: Das Rust-Backend produziert asynchrone Ereignisse (Mausbewegung erreicht Hotzone, Datei wird fallengelassen), die das React-Frontend reaktiv darstellen muss. Gleichzeitig löst das Frontend Zustandsänderungen aus (Item hinzufügen, Einstellungen ändern), die im Backend verarbeitet und persistiert werden müssen.

Die Kommunikation zwischen dem Tauri-Rust-Core und dem WebView-Frontend muss drei Anforderungen erfüllen:

1. **Low-Latency:** Hotzone-Events müssen das Frontend in < 16ms erreichen, damit die Slide-Animation frameperfect starten kann
2. **Zuverlässigkeit:** Shelf-Daten-Mutationen müssen atomar und bestätigt sein, bevor die UI aktualisiert wird
3. **Lose Kopplung:** Frontend und Backend sollen unabhängig testbar und austauschbar sein

Evaluierte Alternativen: **Polling (setInterval)**, **Direkte Funktionsaufrufe (Shared State)**, **Event-Driven Architecture (Tauri Commands + Events)**, **WebSocket in-process**.

---

## Entscheidung

Wir verwenden eine **Event-Driven Architecture** mit zwei klar getrennten Kommunikationskanälen:

1. **Tauri Commands (Request-Response):** Das Frontend ruft `invoke("command_name", args)` auf und erhält ein typisiertes `Promise<Result>`. Ausschließlich für **zustandsverändernde Operationen** (CRUD, Settings-Updates, Window-Controls).

2. **Tauri Events (Pub-Sub / Fire-and-Forget):** Das Rust-Backend emittiert Events wenn sich der Systemzustand ändert (Hotzone-Eintritt/-Austritt, Drop-Event). Das Frontend subscribed via `listen()`. Ausschließlich für **asynchrone Systemzustandsänderungen**, die nicht auf einen Frontend-Request zurückgehen.

```
Frontend                          Rust Backend
   │                                    │
   │── invoke("add_shelf_item") ───────>│
   │<─── Ok(ShelfItem) ────────────────│   Commands: Bidirektional, bestätigt
   │                                    │
   │<── emit("hotzone:enter") ─────────│
   │<── emit("shelf:item-added") ──────│   Events: Unidirektional Backend→Frontend
   │                                    │
```

---

## Begründung

### Vergleich der Alternativen

#### Alternative 1: Polling via `setInterval`

```typescript
// ABGELEHNT
useEffect(() => {
  const interval = setInterval(async () => {
    const pos = await invoke<MousePosition>('get_mouse_position');
    if (pos.y < 2) setIsVisible(true);
  }, 16); // 60fps polling
  return () => clearInterval(interval);
}, []);
```

**Probleme:**
- CPU-Dauerlast: `invoke()` alle 16ms bedeutet ~3.600 IPC-Roundtrips pro Minute — auch im Idle-Betrieb
- Latenzvariabilität: Polling-Intervall = Worst-Case-Latenz. Bei 16ms Intervall kann die Bar bis zu 32ms zu spät reagieren
- Ressourcenverschwendung: Das Rust-Backend muss permanent die Mausposition auf Anfrage bereitstellen, auch wenn sich nichts ändert
- Schlechte Skalierbarkeit: Jedes neue Feature (neue Art von System-Event) erhöht die Polling-Frequenz oder erfordert mehr Polling-Funktionen

**Fazit: Inakzeptabel für ein performance-kritisches always-on Tool.**

#### Alternative 2: Direkter Shared State (via Mutex / Arc)

```rust
// ABGELEHNT
static GLOBAL_STATE: Lazy<Arc<Mutex<AppState>>> = Lazy::new(|| {
    Arc::new(Mutex::new(AppState::default()))
});
```

```typescript
// Frontend würde direkt auf Rust-State zugreifen — nicht möglich
// Kein direkter Shared-Memory-Zugriff zwischen WebView und Rust
```

Tauri's Architektur verbietet direkten Shared-State zwischen WebView (Browser-Kontext) und Rust-Backend grundsätzlich — die WebView ist ein separater Prozess. Dieser Ansatz ist technisch nicht realisierbar ohne Command-basierte Kommunikation.

#### Alternative 3: WebSocket (in-process)

Ein Rust-HTTP-Server (z.B. `axum`) könnte eine WebSocket-Verbindung für Bi-direktionale Kommunikation anbieten. Das Frontend würde sich per `new WebSocket("ws://localhost:PORT")` verbinden.

**Probleme:**
- Unnötige Komplexität: Tauri löst dieses Problem bereits elegant ohne separaten TCP-Server
- Sicherheitsrisiko: Ein offener TCP-Port ist ein potenzieller Angriffspunkt; andere Prozesse könnten sich verbinden
- Port-Konflikte: Port-Wahl ist fehleranfällig (Port bereits belegt)
- Overhead: TCP/IP-Stack-Overhead für lokale IPC — langsamer als Tauri's direktes IPC via Named Pipes / Unix Sockets

**Fazit: Erhöhte Komplexität ohne Mehrwert gegenüber Tauri's eingebautem IPC.**

#### Gewählte Lösung: Tauri Commands + Events

**Vorteile:**

**Lose Kopplung durch explizite Event-Contracts:**
Frontend und Backend kennen sich ausschließlich über die Event-Namen und Command-Signaturen. Diese sind in `src/types/events.ts` und den Command-Signaturen in `commands/*.rs` zentral definiert — der einzige "Vertrag" zwischen beiden Seiten.

```typescript
// src/types/events.ts — der gemeinsame Kontrakt
export const EVENTS = {
  HOTZONE_ENTER: 'hotzone:enter',
  HOTZONE_LEAVE: 'hotzone:leave',
  SHELF_ITEM_ADDED: 'shelf:item-added',
  SHELF_ITEM_REMOVED: 'shelf:item-removed',
  SHELF_ITEM_UPDATED: 'shelf:item-updated',
  SETTINGS_CHANGED: 'settings:changed',
  DND_DRAG_ENTER: 'dnd:drag-enter',
  DND_DRAG_LEAVE: 'dnd:drag-leave',
  DND_DROPPED: 'dnd:dropped',
} as const;
```

**Reaktivität ohne Polling:**
OS-Events treiben den Datenfluss an. Der `HotzoneTracker` in Rust emittiert `hotzone:enter` im selben Callback-Thread des OS-Mouse-Hooks — keine Latenz durch Polling-Intervalle.

```rust
// Hotzone-Event direkt im OS-Hook-Callback
extern "system" fn mouse_hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if let Some(info) = get_mouse_info(lparam) {
        if info.pt.y < HOTZONE_HEIGHT as i32 && !IS_ACTIVE.load(Ordering::Relaxed) {
            APP_HANDLE.emit("hotzone:enter", ()).unwrap();
            IS_ACTIVE.store(true, Ordering::Relaxed);
        }
    }
    unsafe { CallNextHookEx(HOOK, code, wparam, lparam) }
}
```

**Testbarkeit:**
Jeder Command ist eine reine Funktion mit `tauri::State`-Injektion — testbar ohne Tauri-App-Kontext:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_shelf_item_returns_item_with_id() {
        let pool = test_pool().await;
        let store = ShelfStore::new(pool);
        // Kein Tauri-App-Kontext nötig — direkt testbar
        let result = store.add_item(test_create_item()).await;
        assert!(result.is_ok());
        assert!(!result.unwrap().id.is_empty());
    }
}
```

Frontend-Hooks können mit Mock-Event-Emittern getestet werden:

```typescript
// useHotzoneState.test.ts
it('setzt isVisible=true bei hotzone:enter Event', async () => {
  const { result } = renderHook(() => useHotzoneState(), {
    wrapper: MockTauriProvider, // Injects mock listen()
  });
  await act(() => mockEmit(EVENTS.HOTZONE_ENTER));
  expect(result.current.isVisible).toBe(true);
});
```

**Extensibilität:**
Neue Features erfordern nur neue Event-Namen oder Command-Signaturen — keine Änderungen an bestehenden Kommunikationspfaden.

### Klare Trennung: Wann Command, wann Event?

| Situation | Kanal | Beispiel |
|-----------|-------|---------|
| Frontend initiiert Zustandsänderung | Command (invoke) | `add_shelf_item`, `update_settings` |
| Frontend fragt Daten ab | Command (invoke) | `list_shelf_items`, `get_settings` |
| Backend meldet Systemzustandsänderung | Event (emit) | `hotzone:enter`, `dnd:dropped` |
| Backend bestätigt Backend-initiierte Änderung | Event (emit) | `shelf:item-added` nach externem Drop |

---

## Implementierungsdetails

### Event-Typing auf Rust-Seite

```rust
// src-tauri/src/types.rs
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct HotzoneEnterPayload {
    pub monitor_id: u32,
    pub cursor_x: i32,
}

#[derive(Serialize, Clone)]
pub struct ShelfItemAddedPayload {
    pub item: ShelfItem,
    pub source: AddSource, // "drop" | "command" | "import"
}

#[derive(Serialize, Clone)]
pub enum AddSource {
    Drop,
    Command,
    Import,
}
```

### Registrierung in `lib.rs`

```rust
// src-tauri/src/lib.rs
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Module initialisieren
            let store = ShelfStore::new(create_pool(&app.path()).await?);
            app.manage(store);

            let hotzone_tracker = HotzoneTracker::setup(app.handle().clone(), HotzoneConfig::default());
            app.manage(hotzone_tracker);

            DndHandler::setup(app.handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // shelf_commands
            list_shelf_items,
            add_shelf_item,
            remove_shelf_item,
            reorder_shelf_items,
            // settings_commands
            get_settings,
            update_setting,
            // system_commands
            open_shelf_item,
            show_item_in_finder,
        ])
        .run(tauri::generate_context!())
        .expect("Popup Bar konnte nicht gestartet werden");
}
```

---

## Konsequenzen

### Positiv

- **Maximale Reaktivität:** OS-Events landen ohne Polling-Overhead im Frontend
- **Vollständige Testbarkeit:** Commands und Hooks sind isoliert testbar
- **Klare Architektur:** Jeder Entwickler weiß sofort, welchen Kanal er nutzen soll
- **Kein Sicherheitsrisiko** durch offene Netzwerk-Ports

### Negativ

- **Event-Ordering ist nicht garantiert:** Bei sehr schnellen Event-Sequenzen (z.B. Enter/Leave in < 10ms) könnten Events im Frontend in falscher Reihenfolge ankommen. Mitigation: Debounce-Logik im `HotzoneTracker` (300ms Cooldown).
- **Fire-and-Forget-Events:** Das Backend weiß nicht, ob das Frontend ein Event empfangen hat. Bei kritischen Zustandsänderungen (z.B. externem Drop) wird daher zusätzlich ein Command angeboten, um den aktuellen State abzurufen.
- **Boilerplate:** Jedes neue Feature erfordert Event-Namen in `events.ts`, Payload-Typen in Rust und TypeScript und `listen()`-Registrierung. Bei sehr vielen Features könnte dies unübersichtlich werden. Mitigation: Konventionen durch `EVENTS`-Konstanten-Objekt und Code-Generator (langfristig).

---

## Verwandte Entscheidungen

- ADR-001: Tauri 2 wurde auch wegen des ausgereiften Command+Event-IPC-Systems gewählt
- ADR-002: SQLite-Writes emittieren nach Abschluss Events an das Frontend (`shelf:item-added`)
- ADR-004: Der `PlatformProvider`-Trait abstrahiert OS-Hooks, die Events in dieses System einspeisen

---

*ADR-003 — Event-Driven Architektur — Akzeptiert 2026-03-12*
