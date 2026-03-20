import { useEffect, useState, type CSSProperties } from "react";
import { useSettingsStore } from "../stores/settingsStore";
import { getSkinDataUrl } from "../utils/tauri-bridge";
import { getCachedDataUrl } from "../utils/media-cache";

export function useGlassmorphism(): CSSProperties {
  const activeSkin = useSettingsStore((s) => s.settings.activeSkin);
  const [skinUrl, setSkinUrl] = useState<string | null>(null);

  useEffect(() => {
    let isMounted = true;

    if (!activeSkin) {
      setSkinUrl(null);
      return () => {
        isMounted = false;
      };
    }

    getCachedDataUrl(`skin:${activeSkin}`, () => getSkinDataUrl(activeSkin))
      .then((url) => {
        if (isMounted) {
          setSkinUrl(url);
        }
      })
      .catch(() => {
        if (isMounted) {
          setSkinUrl(null);
        }
      });

    return () => {
      isMounted = false;
    };
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
