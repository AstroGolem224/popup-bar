import { useEffect, useState, type CSSProperties } from "react";
import { useSettingsStore } from "../stores/settingsStore";
import { getSkinDataUrl } from "../utils/tauri-bridge";

export function useGlassmorphism(): CSSProperties {
  const activeSkin = useSettingsStore((s) => s.settings.activeSkin);
  const [skinUrl, setSkinUrl] = useState<string | null>(null);

  useEffect(() => {
    if (!activeSkin) {
      setSkinUrl(null);
      return;
    }
    getSkinDataUrl(activeSkin).then(setSkinUrl);
  }, [activeSkin]);

  const base: CSSProperties = {
    background: "var(--shelf-bar-bg)",
    backdropFilter: "var(--shelf-bar-blur)",
    WebkitBackdropFilter: "var(--shelf-bar-blur)",
    border: "var(--shelf-bar-border)",
    boxShadow: "var(--shelf-bar-shadow)",
  };

  if (skinUrl) {
    return {
      ...base,
      backgroundImage: `url(${skinUrl})`,
      backgroundSize: "cover",
      backgroundPosition: "center",
    };
  }
  return base;
}
