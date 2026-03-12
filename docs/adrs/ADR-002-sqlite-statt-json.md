# ADR-002: SQLite statt JSON-Datei für Persistenz

**Status:** Akzeptiert  
**Datum:** 2026-03-12  
**Entscheidungsträger:** Projektteam  
**Letzte Überprüfung:** 2026-03-12

---

## Kontext

Popup Bar muss folgende Daten persistent speichern:

- **`shelf_items`:** Alle Items auf der Bar (Dateipfade, URLs, App-Shortcuts) mit Typ, Label, Icon-Pfad und Position
- **`item_groups`:** Optionale Gruppen zur Kategorisierung von Items
- **`settings`:** Alle konfigurierbaren Parameter (`AppSettings`: Hotzone-Höhe, Blur-Radius, Autostart, etc.)
- **`icon_cache`:** Hash-basiertes Mapping von Quelldatei-Pfad zu gecachtem PNG-Icon

Die Daten werden primär beim App-Start gelesen, bei jeder Benutzerinteraktion geschrieben (Item hinzufügen, entfernen, umsortieren) und müssen über App-Neustarts hinweg korrekt erhalten bleiben.

Evaluierte Alternativen: **JSON-Datei(en)**, **SQLite (via sqlx + tauri-plugin-sql)**, **IndexedDB (im WebView)**, **TOML-Datei**, **serde_json + bincode**.

---

## Entscheidung

Wir verwenden **SQLite** über `sqlx 0.8` mit `runtime-tokio`-Feature und `tauri-plugin-sql 2.2`.

Das Datenbankschema umfasst vier Tabellen: `shelf_items`, `item_groups`, `settings` (Key-Value-Store) und `icon_cache`. Migrationen werden über nummerierte `.sql`-Dateien in `src-tauri/migrations/` verwaltet und beim App-Start automatisch via `sqlx::migrate!()` ausgeführt.

---

## Begründung

### Vergleichsmatrix: Persistenz-Alternativen

| Kriterium | SQLite | JSON-Datei | IndexedDB | TOML-Datei |
|-----------|--------|-----------|-----------|-----------|
| **Concurrent Access** | Ja (WAL-Modus) | Nein (Datei-Lock nötig) | Ja (Browser-intern) | Nein |
| **Schema-Migrationen** | Ja (sqlx migrate!) | Manuell (komplex) | Manuell | Manuell |
| **Query-Fähigkeiten** | SQL (ORDER BY, JOIN, WHERE) | Vollständig in RAM laden | Cursor-basiert (umständlich) | Vollständig in RAM laden |
| **Atomare Writes** | Ja (Transaktionen) | Nein (Schreiben = Datei-Truncate + Write) | Ja | Nein |
| **Typsicherheit** | `sqlx::FromRow` + Rust-Typen | `serde_json::from_str` | TypeScript-Interfaces | `serde` |
| **Performance (100 Items)** | < 1ms für alle CRUD-Ops | < 1ms | < 5ms | < 1ms |
| **Performance (10.000 Items)** | < 5ms | ~50ms (JSON-Parse) | ~20ms | ~100ms |
| **Crash-Recovery** | WAL + automatisches Recovery | Datei kann teilweise korrupt sein | Browser-managed | Datei kann teilweise korrupt sein |
| **Frontend-Zugriff** | Nur via Rust Commands | Via `tauri-plugin-fs` möglich | Direkt im WebView | Via `tauri-plugin-fs` möglich |
| **Dependencies** | `sqlx`, `tauri-plugin-sql` | Keine zusätzlich | `@tauri-apps/plugin-sql` | `toml` crate |
| **Lernkurve** | SQL (bekannt) | Sehr niedrig | JavaScript Promises | TOML-Syntax |

### Entscheidende Faktoren für SQLite

**1. Atomare Schreiboperationen:**
Der kritischste Nachteil von JSON-Dateien ist der Schreibprozess: Ein Programm-Absturz während des Schreibens (z.B. `Ctrl+C` im Entwicklungsmodus) kann die gesamte Konfigurationsdatei korrumpieren — alle Items gehen verloren. SQLite mit WAL-Modus (`PRAGMA journal_mode = WAL`) garantiert atomare Transaktionen: Entweder wird der gesamte Write committed, oder er wird zurückgerollt.

```sql
-- Reorder-Operation: Alle Updates in einer Transaktion
BEGIN TRANSACTION;
UPDATE shelf_items SET position = 0, updated_at = datetime('now') WHERE id = ?;
UPDATE shelf_items SET position = 1, updated_at = datetime('now') WHERE id = ?;
UPDATE shelf_items SET position = 2, updated_at = datetime('now') WHERE id = ?;
COMMIT;
```

**2. Schema-Migrationen:**
Wenn Popup Bar ein neues Feature bekommt (z.B. eine `tags`-Spalte für Items), braucht man eine Migrationsstrategie. Mit SQLite ist das trivial:

```sql
-- migrations/0002_add_tags.sql
ALTER TABLE shelf_items ADD COLUMN tags TEXT;
```

Mit einer JSON-Datei müsste man eine eigene Migrations-Logik implementieren (`"schema_version": 2` im JSON prüfen, transformieren). Das ist fehleranfällig und schwer testbar.

**3. Query-Fähigkeiten:**
Für die Sortierung nach `position`, Filterung nach `item_type`, und JOIN-Abfragen für Items mit ihren Gruppen ist SQL erheblich ausdrucksstärker als manuelles JSON-Filtern in Rust:

```sql
-- Items mit Gruppen-Namen, sortiert nach Position
SELECT
    si.*,
    ig.name AS group_name
FROM shelf_items si
LEFT JOIN item_groups ig ON si.group_id = ig.id
ORDER BY si.position ASC;
```

Dasselbe in JSON würde einen vollständigen Deserialisierungs- und Sortier-Schritt in Rust erfordern.

**4. `tauri-plugin-sql`-Unterstützung:**
Das offizielle Tauri-Ökosystem bietet `tauri-plugin-sql 2.2` mit SQLite-Backend. Dies ist gut gepflegt, well-tested und integriert sich nativ in den Tauri-App-Builder. Die Alternative, IndexedDB direkt im WebView zu nutzen, würde bedeuten, dass der Datenbankzugriff im unsicheren WebView-Kontext stattfindet — ein Security-Anti-Pattern in Tauri.

**5. Concurrent Access (zukunftssicher):**
Obwohl Popup Bar aktuell kein Multi-Window-Szenario hat, könnte in Zukunft ein separates Settings-Fenster oder eine Quick-Actions-Overlay-Komponente als eigenes Tauri-Fenster existieren. SQLite (WAL-Modus) erlaubt mehrere gleichzeitige Lesezugriffe und einen Schreiber — JSON würde hier sofort scheitern.

### Warum nicht IndexedDB?

IndexedDB läuft im WebView-Kontext. Tauri 2 empfiehlt explizit, sensitive Operationen und Datenpersistenz im Rust-Backend zu handhaben. IndexedDB-Daten sind unter `AppData/...` gespeichert, aber nicht in einem kontrollierbaren Pfad. Außerdem würde IndexedDB dazu führen, dass die Datenbanklogik in TypeScript statt Rust implementiert wird — was die Typsicherheit und Testbarkeit verschlechtert.

### Warum nicht TOML/JSON?

Für die `settings`-Tabelle wäre TOML tatsächlich eine valide Alternative (einfach, menschenlesbar). Wir verwenden jedoch bewusst SQLite auch für Settings, um eine einzige Persistenzschicht zu haben. Die Settings werden als Key-Value-Paare in der `settings`-Tabelle gespeichert — einfach und flexibel.

---

## Implementierungsdetails

```rust
// modules/shelf_store.rs
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub async fn create_pool(app_dir: &std::path::Path) -> Result<SqlitePool, sqlx::Error> {
    let db_path = app_dir.join("popup-bar.db");
    let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // WAL-Modus für bessere Performance und Crash-Recovery
    sqlx::query("PRAGMA journal_mode = WAL").execute(&pool).await?;
    sqlx::query("PRAGMA foreign_keys = ON").execute(&pool).await?;

    // Migrationen automatisch ausführen
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
```

---

## Konsequenzen

### Positiv

- **Atomare Schreiboperationen** via Transaktionen — kein Datenverlust bei Absturz
- **Schema-Migrationen** mit `sqlx migrate!()` — einfaches Schema-Evolution
- **Typsichere Abfragen** via `sqlx::FromRow` — Kompilier-Zeit-Fehler bei Schema-Mismatch
- **WAL-Modus** ermöglicht gleichzeitige Lese-Operationen ohne Write-Blockierung
- **Einzige Persistenz-Schicht** für alle Daten — kein Mix aus DB und Konfig-Dateien

### Negativ

- **Dependency-Overhead:** `sqlx` + `tauri-plugin-sql` fügen Build-Zeit und Binary-Größe hinzu (~500 KB). Akzeptabel.
- **SQL-Schema-Pflege:** Änderungen am Schema erfordern eine neue Migrations-Datei. Für ein kleines Team mit klaren Conventions kein Problem.
- **Nicht menschenlesbar:** Die Datenbank ist nicht direkt editierbar wie eine JSON-Datei. Mitigation: Import/Export-Funktion in Settings-UI geplant (Phase 5).
- **Keine direkte Frontend-Abfragen:** Alle Datenbankoperationen laufen über Rust-Commands. Das ist ein Feature (Sicherheit), nicht ein Bug — erhöht aber die Anzahl notwendiger `invoke()`-Aufrufe.

---

## Verwandte Entscheidungen

- ADR-003: Das Event-Driven-Muster ist Voraussetzung für die Benachrichtigung des Frontends nach SQLite-Schreiboperationen (`shelf:item-added`-Event nach erfolgreichem INSERT)

---

*ADR-002 — SQLite statt JSON-Datei — Akzeptiert 2026-03-12*
