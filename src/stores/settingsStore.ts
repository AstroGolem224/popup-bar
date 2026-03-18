import { create } from "zustand";
import type { Settings, SkinInfo } from "../types/settings";
import { DEFAULT_SETTINGS } from "../types/settings";

interface SettingsState {
  settings: Settings;
  skins: SkinInfo[];
  updateSetting: <K extends keyof Settings>(key: K, value: Settings[K]) => void;
  setSettings: (settings: Settings) => void;
  resetSettings: () => void;
  setSkins: (skins: SkinInfo[]) => void;
}

export const useSettingsStore = create<SettingsState>((set) => ({
  settings: { ...DEFAULT_SETTINGS },
  skins: [],

  updateSetting: (key, value) =>
    set((state) => ({
      settings: { ...state.settings, [key]: value },
    })),

  setSettings: (settings) => set({ settings }),

  resetSettings: () => set({ settings: { ...DEFAULT_SETTINGS } }),

  setSkins: (skins) => set({ skins }),
}));
