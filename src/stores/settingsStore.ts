import { create } from "zustand";
import type { Settings } from "../types/settings";
import { DEFAULT_SETTINGS } from "../types/settings";

interface SettingsState {
  settings: Settings;
  updateSetting: <K extends keyof Settings>(key: K, value: Settings[K]) => void;
  setSettings: (settings: Settings) => void;
  resetSettings: () => void;
}

export const useSettingsStore = create<SettingsState>((set) => ({
  settings: { ...DEFAULT_SETTINGS },

  updateSetting: (key, value) =>
    set((state) => ({
      settings: { ...state.settings, [key]: value },
    })),

  setSettings: (settings) => set({ settings }),

  resetSettings: () => set({ settings: { ...DEFAULT_SETTINGS } }),
}));
