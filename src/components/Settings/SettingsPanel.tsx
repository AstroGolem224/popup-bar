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
  const skins = useSettingsStore((s) => s.skins);
  const [previews, setPreviews] = useState<Record<string, string>>({});

  useEffect(() => {
    skins.forEach((skin) => {
      if (!previews[skin.filename]) {
        getSkinDataUrl(skin.filename).then((url) => {
          if (url) {
            setPreviews((prev) => ({ ...prev, [skin.filename]: url }));
          }
        });
      }
    });
  }, [skins]);

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
              onClick={(e) => {
                e.stopPropagation();
                onDelete(skin.filename);
              }}
            >
              ×
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
  const { settings, setSettings, setSkins } = useSettingsStore();
  const fileInputRef = useRef<HTMLInputElement>(null);

  const refreshSkins = useCallback(async () => {
    try {
      const list = await listSkins();
      setSkins(list);
    } catch (_) {}
  }, [setSkins]);

  useEffect(() => {
    void refreshSkins();
  }, [refreshSkins]);

  async function apply<K extends keyof Settings>(key: K, value: Settings[K]) {
    const next = { ...settings, [key]: value };
    try {
      const saved = await updateSettings(next);
      setSettings(saved);
      if (key === "autostart") {
        await setLaunchAtLogin(!!saved.autostart);
      }
    } catch (_) {
      /* keep local state on error */
    }
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
      setSettings(saved);
      await refreshSkins();
    } catch (_) {}
  }

  async function handleFileSelected(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    if (!file) return;
    const ext = file.name.split(".").pop()?.toLowerCase() ?? "";
    const stem = file.name.replace(/\.[^/.]+$/, "");
    const buffer = await file.arrayBuffer();
    const bytes = Array.from(new Uint8Array(buffer));
    try {
      await importSkinBytes(stem, ext, bytes);
      await refreshSkins();
    } catch (_) {}
    if (fileInputRef.current) fileInputRef.current.value = "";
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
            aria-label="Schließen"
          >
            ×
          </button>
        ) : null}
      </div>

      <SettingsGroup title="Hotzone">
        <label className="settings-panel__field" title="Höhe des Auslösebereichs am oberen Bildschirmrand in Pixeln">
          <span>Auslöse-Höhe (px)</span>
          <input
            type="range"
            min={1}
            max={20}
            value={settings.hotzoneSize}
            onChange={(e) => apply("hotzoneSize", Number(e.target.value))}
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
            onChange={(e) => apply("barWidthPx", Number(e.target.value))}
          />
          <span className="settings-panel__hint">
            {typeof settings.barWidthPx === "number" ? settings.barWidthPx : 480} px
          </span>
        </label>
        <label className="settings-panel__field" title="Höhe der Leiste in Pixeln">
          <span>Leistenhöhe (px)</span>
          <input
            type="range"
            min={56}
            max={180}
            step={4}
            value={typeof settings.barHeightPx === "number" ? settings.barHeightPx : 72}
            onChange={(e) => apply("barHeightPx", Number(e.target.value))}
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
            onChange={(e) => apply("animationSpeed", Number(e.target.value))}
          />
        </label>
        <label className="settings-panel__field" title="Ausrichtung der Icons in der Leiste">
          <span>Ausrichtung</span>
          <select
            value={settings.alignment}
            onChange={(e) => apply("alignment", e.target.value as any)}
          >
            <option value="centered">Zentriert</option>
            <option value="start">Linksbündig</option>
            <option value="grid">Am Raster ausrichten</option>
          </select>
        </label>
        <label className="settings-panel__field settings-panel__field--row" title="Bar nur auf dem Hauptmonitor anzeigen">
          <span>Nur Primärmonitor</span>
          <input
            type="checkbox"
            checked={!settings.multiMonitor}
            onChange={(e) => apply("multiMonitor", !e.target.checked)}
          />
        </label>
      </SettingsGroup>

      <SettingsGroup title="Skin">
        <SkinGrid
          activeSkin={settings.activeSkin}
          onSelect={(f) => void handleSkinSelect(f)}
          onDelete={(f) => void handleSkinDelete(f)}
          onImport={() => fileInputRef.current?.click()}
        />
        <input
          ref={fileInputRef}
          type="file"
          accept="image/png,image/jpeg"
          style={{ display: "none" }}
          onChange={(e) => void handleFileSelected(e)}
        />
      </SettingsGroup>

      <SettingsGroup title="System">
        <label className="settings-panel__field settings-panel__field--row" title="App beim Systemstart starten">
          <span>Beim Systemstart öffnen</span>
          <input
            type="checkbox"
            checked={settings.autostart}
            onChange={(e) => apply("autostart", e.target.checked)}
          />
        </label>
      </SettingsGroup>
    </div>
  );
}
