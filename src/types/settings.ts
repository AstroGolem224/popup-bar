export type Theme = "light" | "dark" | "system";

export interface Settings {
  hotzoneSize: number;
  animationSpeed: number;
  blurIntensity: number;
  tintColor: string;
  theme: Theme;
  autostart: boolean;
  multiMonitor: boolean;
}

export const DEFAULT_SETTINGS: Settings = {
  hotzoneSize: 5,
  animationSpeed: 1.0,
  blurIntensity: 20,
  tintColor: "rgba(255, 255, 255, 0.1)",
  theme: "system",
  autostart: false,
  multiMonitor: false,
};
