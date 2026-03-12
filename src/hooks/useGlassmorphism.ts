import type { CSSProperties } from "react";
import { useSettingsStore } from "../stores/settingsStore";

/** Returns CSS properties for the glassmorphism effect based on current settings. */
export function useGlassmorphism(): CSSProperties {
  const { blurIntensity, tintColor } = useSettingsStore((s) => s.settings);

  return {
    background: tintColor,
    backdropFilter: `blur(${blurIntensity}px)`,
    WebkitBackdropFilter: `blur(${blurIntensity}px)`,
    border: "1px solid rgba(255, 255, 255, 0.18)",
  };
}
