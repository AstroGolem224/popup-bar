import { useCallback, useEffect, useRef, useState } from "react";
import { useSettingsStore } from "../../stores/settingsStore";
import {
  updateSettings,
  setLaunchAtLogin,
  listSkins,
  importSkinBytes,
  setActiveSkin,
  deleteSkin,
  getSkinDataUrl,
} from "../../utils/tauri-bridge";
import {
  evictCachedDataUrl,
  getCachedDataUrl,
  pruneCachedDataUrlsByPrefix,
} from "../../utils/media-cache";
import type { Settings } from "../../types/settings";
import "./SettingsPanel.css";

function SkinGrid({
  activeSkin,
  onSelect,
  onDelete,
  onImport,
}: {
  activeSkin: string | null | undefined;
  onSelect: (filename: string | null) => void;
  onDelete: (filename: string) => void;
  onImport: () => void;
}) {
  const skins = useSettingsStore((state) => state.skins);
  const [previews, setPreviews] = useState<Record<string, string>>({});

  useEffect(() => {
    const activePreviewKeys = skins.map((skin) => `skin:${skin.filename}`);
    pruneCachedDataUrlsByPrefix("skin:", activePreviewKeys);

    skins.forEach((skin) => {
      if (!previews[skin.filename]) {
        getCachedDataUrl(`skin:${skin.filename}`, () => getSkinDataUrl(skin.filename)).then((url) => {
          if (url) {
            setPreviews((previous) => ({ ...previous, [skin.filename]: url }));
          }
        });
      }
    });
  }, [previews, skins]);

  return (
    <div className="skin-grid">
      <div className="skin-grid__items">
        <button
          type="button"
          className={`skin-tile skin-tile--none${!activeSkin ? " skin-tile--active" : ""}`}
          onClick={() => onSelect(null)}
          title="Standard (Glassmorphism)"
        >
          <span className="skin-tile__label">Standard</span>
        </button>
        {skins.map((skin) => (
          <button
            key={skin.filename}
            type="button"
            className={`skin-tile${activeSkin === skin.filename ? " skin-tile--active" : ""}`}
            onClick={() => onSelect(skin.filename)}
            title={skin.name}
          >
            {previews[skin.filename] ? (
              <img
                className="skin-tile__preview"
                src={previews[skin.filename]}
                alt={skin.name}
              />
            ) : null}
            <span
              className="skin-tile__delete"
              role="button"
              aria-label={`${skin.name} loeschen`}
              onClick={(event) => {
                event.stopPropagation();
                onDelete(skin.filename);
              }}
            >
              ✕
            </span>
          </button>
        ))}
      </div>
      <button type="button" className="skin-grid__import-btn" onClick={onImport}>
        + Skin importieren
      </button>
    </div>
  );
}

export interface SettingsPanelProps {
  className?: string;
  onClose?: () => void;
}

function SettingsGroup({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <div className="settings-panel__group">
      <h3 className="settings-panel__group-title">{title}</h3>
      {children}
    </div>
  );
}

export function SettingsPanel({ className, onClose }: SettingsPanelProps) {
  const settings = useSettingsStore((state) => state.settings);
  const setSettings = useSettingsStore((state) => state.setSettings);
  const setSkins = useSettingsStore((state) => state.setSkins);
  const updateSetting = useSettingsStore((state) => state.updateSetting);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [shortcutDraft, setShortcutDraft] = useState(settings.globalShortcut);

  const refreshSkins = useCallback(async () => {
    try {
      const list = await listSkins();
      setSkins(list);
    } catch (_) {}
  }, [setSkins]);

  useEffect(() => {
    void refreshSkins();
  }, [refreshSkins]);

  useEffect(() => {
    setShortcutDraft(settings.globalShortcut);
  }, [settings.globalShortcut]);

  async function applyPatch(patch: Partial<Settings>) {
    const previous = settings;
    const next = { ...previous, ...patch };

    for (const [key, value] of Object.entries(patch) as [keyof Settings, Settings[keyof Settings]][]) {
      updateSetting(key, value);
    }

    try {
      const saved = await updateSettings(next);
      setSettings(saved);
      if ("autostart" in patch) {
        await setLaunchAtLogin(!!saved.autostart);
      }
    } catch (_) {
      setSettings(previous);
    }
  }

  async function apply<K extends keyof Settings>(key: K, value: Settings[K]) {
    await applyPatch({ [key]: value } as Pick<Settings, K>);
  }

  async function applyMonitorStrategy(monitorStrategy: Settings["monitorStrategy"]) {
    await applyPatch({
      monitorStrategy,
      multiMonitor: monitorStrategy !== "primary",
    });
  }

  async function commitShortcutDraft() {
    if (shortcutDraft === settings.globalShortcut) {
      return;
    }
    await apply("globalShortcut", shortcutDraft);
  }

  async function handleSkinSelect(filename: string | null) {
    try {
      const saved = await setActiveSkin(filename);
      setSettings(saved);
    } catch (_) {}
  }

  async function handleSkinDelete(filename: string) {
    try {
      const saved = await deleteSkin(filename);
      evictCachedDataUrl(`skin:${filename}`);
      setSettings(saved);
      await refreshSkins();
    } catch (_) {}
  }

  async function handleFileSelected(event: React.ChangeEvent<HTMLInputElement>) {
    const file = event.target.files?.[0];
    if (!file) {
      return;
    }

    const ext = file.name.split(".").pop()?.toLowerCase() ?? "";
    const stem = file.name.replace(/\.[^/.]+$/, "");
    const buffer = await file.arrayBuffer();
    const bytes = Array.from(new Uint8Array(buffer));

    try {
      await importSkinBytes(stem, ext, bytes);
      await refreshSkins();
    } catch (_) {}

    if (fileInputRef.current) {
      fileInputRef.current.value = "";
    }
  }

  return (
    <div className={`settings-panel ${className ?? ""}`} role="dialog" aria-label="Einstellungen">
      <div className="settings-panel__head">
        <h2 className="settings-panel__title">Einstellungen</h2>
        {onClose ? (
          <button
            type="button"
            className="settings-panel__close"
            onClick={onClose}
            aria-label="Schliessen"
          >
            ✕
          </button>
        ) : null}
      </div>

      <SettingsGroup title="Hotzone">
        <label className="settings-panel__field" title="Hoehe des Ausloesebereichs am oberen Bildschirmrand in Pixeln">
          <span>Ausloese-Hoehe (px)</span>
          <input
            type="range"
            min={1}
            max={20}
            value={settings.hotzoneSize}
            onChange={(event) => void apply("hotzoneSize", Number(event.target.value))}
          />
        </label>
      </SettingsGroup>

      <SettingsGroup title="Darstellung">
        <label className="settings-panel__field" title="Breite der Leiste in Pixeln (Icons umbrechen in neue Zeile)">
          <span>Leistenbreite (px)</span>
          <input
            type="range"
            min={280}
            max={900}
            step={20}
            value={typeof settings.barWidthPx === "number" ? settings.barWidthPx : 480}
            onChange={(event) => void apply("barWidthPx", Number(event.target.value))}
          />
          <span className="settings-panel__hint">
            {typeof settings.barWidthPx === "number" ? settings.barWidthPx : 480} px
          </span>
        </label>

        <label className="settings-panel__field" title="Hoehe der Leiste in Pixeln">
          <span>Leistenhoehe (px)</span>
          <input
            type="range"
            min={56}
            max={180}
            step={4}
            value={typeof settings.barHeightPx === "number" ? settings.barHeightPx : 72}
            onChange={(event) => void apply("barHeightPx", Number(event.target.value))}
          />
          <span className="settings-panel__hint">
            {typeof settings.barHeightPx === "number" ? settings.barHeightPx : 72} px
          </span>
        </label>

        <label className="settings-panel__field" title="Geschwindigkeit der Ein-/Ausblend-Animation">
          <span>Animation</span>
          <input
            type="range"
            min={0.1}
            max={3}
            step={0.1}
            value={settings.animationSpeed}
            onChange={(event) => void apply("animationSpeed", Number(event.target.value))}
          />
        </label>

        <label className="settings-panel__field" title="Legt fest, ob Icons frei oder am Raster positioniert werden">
          <span>Icon-Anordnung</span>
          <select
            value={settings.alignment}
            onChange={(event) => void apply("alignment", event.target.value as Settings["alignment"])}
          >
            <option value="centered">Frei, Startlayout zentriert</option>
            <option value="start">Frei, Startlayout links</option>
            <option value="grid">Am Raster ausrichten</option>
          </select>
        </label>

        <label className="settings-panel__field" title="Legt fest, welcher Monitor fuer die Bar verwendet wird">
          <span>Monitor-Strategie</span>
          <select
            value={settings.monitorStrategy}
            onChange={(event) => void applyMonitorStrategy(event.target.value as Settings["monitorStrategy"])}
          >
            <option value="primary">Primaermonitor</option>
            <option value="cursor">Monitor unter Mauszeiger</option>
            <option value="last-active">Zuletzt aktiver Monitor</option>
          </select>
        </label>
        <span className="settings-panel__hint">
          `last-active` faellt bei leerem Verlauf auf cursor und dann auf primary zurueck.
        </span>
      </SettingsGroup>

      <SettingsGroup title="Skin">
        <SkinGrid
          activeSkin={settings.activeSkin}
          onSelect={(filename) => void handleSkinSelect(filename)}
          onDelete={(filename) => void handleSkinDelete(filename)}
          onImport={() => fileInputRef.current?.click()}
        />
        <input
          ref={fileInputRef}
          type="file"
          accept="image/png,image/jpeg"
          style={{ display: "none" }}
          onChange={(event) => void handleFileSelected(event)}
        />
      </SettingsGroup>

      <SettingsGroup title="System">
        <label className="settings-panel__field settings-panel__field--row" title="App beim Systemstart starten">
          <span>Beim Systemstart oeffnen</span>
          <input
            type="checkbox"
            checked={settings.autostart}
            onChange={(event) => void apply("autostart", event.target.checked)}
          />
        </label>

        <label className="settings-panel__field" title="Leer lassen, um den globalen Shortcut zu deaktivieren">
          <span>Global Shortcut</span>
          <input
            type="text"
            value={shortcutDraft}
            placeholder="CommandOrControl+Shift+Space"
            onChange={(event) => setShortcutDraft(event.target.value)}
            onBlur={() => void commitShortcutDraft()}
            onKeyDown={(event) => {
              if (event.key === "Enter") {
                event.currentTarget.blur();
              }
            }}
          />
          <span className="settings-panel__hint">leer = deaktiviert, standard: CommandOrControl+Shift+Space</span>
        </label>

        <div className="settings-panel__field">
          <span>Tray</span>
          <span className="settings-panel__hint">
            linksklick toggelt die leiste, das menue oeffnet settings oder beendet die app.
          </span>
        </div>
      </SettingsGroup>
    </div>
  );
}
