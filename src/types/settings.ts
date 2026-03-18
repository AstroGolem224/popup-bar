export type Theme = "light" | "dark" | "system";

export interface SkinInfo {
  name: string;
  filename: string;
  absolutePath: string;
}

export interface Settings {
  hotzoneSize: number;
  animationSpeed: number;
  /** Nicht mehr in der UI; wird weiterhin für Backend-Kompatibilität mitgesendet. */
  blurIntensity?: number;
  tintColor: string;
  theme: Theme;
  autostart: boolean;
  multiMonitor: boolean;
  barWidthPx: number;
  barHeightPx: number;
  activeSkin?: string | null;
  alignment: "centered" | "start" | "grid";
}

export const DEFAULT_SETTINGS: Settings = {
  hotzoneSize: 5,
  animationSpeed: 1.0,
  blurIntensity: 20,
  tintColor: "rgba(255, 255, 255, 0.1)",
  theme: "system",
  autostart: false,
  multiMonitor: false,
  barWidthPx: 480,
  barHeightPx: 72,
  activeSkin: null,
  alignment: "centered",
};
