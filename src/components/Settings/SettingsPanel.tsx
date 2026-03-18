import { useSettingsStore } from "../../stores/settingsStore";
import { updateSettings, setLaunchAtLogin } from "../../utils/tauri-bridge";
import type { Settings } from "../../types/settings";
import "./SettingsPanel.css";

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
  const { settings, setSettings } = useSettingsStore();

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
          <span className="settings-panel__hint">
            {settings.hotzoneSize} px
          </span>
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
          <span className="settings-panel__hint">
            {Number(settings.animationSpeed || 1).toFixed(1)}x
          </span>
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
