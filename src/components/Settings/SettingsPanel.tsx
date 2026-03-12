import { useSettingsStore } from "../../stores/settingsStore";
import "./SettingsPanel.css";

export interface SettingsPanelProps {
  className?: string;
}

export function SettingsPanel({ className }: SettingsPanelProps) {
  const { settings, updateSetting } = useSettingsStore();

  return (
    <div className={`settings-panel ${className ?? ""}`}>
      <h2 className="settings-panel__title">Settings</h2>

      <label className="settings-panel__field">
        <span>Hotzone Size</span>
        <input
          type="range"
          min={1}
          max={20}
          value={settings.hotzoneSize}
          onChange={(e) => updateSetting("hotzoneSize", Number(e.target.value))}
        />
      </label>

      <label className="settings-panel__field">
        <span>Blur Intensity</span>
        <input
          type="range"
          min={0}
          max={50}
          value={settings.blurIntensity}
          onChange={(e) =>
            updateSetting("blurIntensity", Number(e.target.value))
          }
        />
      </label>

      <label className="settings-panel__field">
        <span>Animation Speed</span>
        <input
          type="range"
          min={0.1}
          max={3}
          step={0.1}
          value={settings.animationSpeed}
          onChange={(e) =>
            updateSetting("animationSpeed", Number(e.target.value))
          }
        />
      </label>
    </div>
  );
}
