import { ShelfBar } from "./components/ShelfBar";
import { DropZone } from "./components/DropZone";
import { SettingsPanel } from "./components/Settings";
import { useHotzoneState } from "./hooks/useHotzoneState";
import { useShelfStore } from "./stores/shelfStore";
import { useSettingsStore } from "./stores/settingsStore";
import { DEFAULT_SETTINGS } from "./types/settings";
import { getPlatformInfo, getSettings, listSkins, listen, getCurrentWindow } from "./utils/tauri-bridge";
import type { Settings } from "./types/settings";
import { useEffect, useState } from "react";
import "./App.css";
import "./styles/animations.css";
import "./styles/glassmorphism.css";

/**
 * Root application component.
 *
 * Wraps the ShelfBar in a DropZone and controls visibility
 * based on hotzone enter/leave events.
 */
function App() {
  const { isVisible, onShelfAnimationEnd } = useHotzoneState();
  const errorMessage = useShelfStore((state) => state.errorMessage);
  const clearError = useShelfStore((state) => state.clearError);
  const setSettings = useSettingsStore((state) => state.setSettings);
  const setSkins = useSettingsStore((state) => state.setSkins);

  useEffect(() => {
    Promise.all([getSettings(), listSkins()])
      .then(([settings, skins]) => {
        setSettings({ ...DEFAULT_SETTINGS, ...settings });
        setSkins(skins);
      })
      .catch(() => {});
  }, [setSettings, setSkins]);

  useEffect(() => {
    const unlisten = listen<Settings>("settings_changed", (event) => {
      setSettings({ ...DEFAULT_SETTINGS, ...event.payload });
      listSkins().then(setSkins).catch(() => {});
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, [setSettings, setSkins]);

  useEffect(() => {
    getPlatformInfo()
      .then((info) => {
        document.body.classList.add(`platform-${info.os}`);
      })
      .catch(() => {});
  }, []);

  const [windowLabel, setWindowLabel] = useState<string>("main");

  useEffect(() => {
    const label = getCurrentWindow().label;
    setWindowLabel(label);
  }, []);

  useEffect(() => {
    if (!errorMessage) {
      return;
    }
    const timeout = window.setTimeout(() => {
      clearError();
    }, 3000);
    return () => {
      window.clearTimeout(timeout);
    };
  }, [clearError, errorMessage]);

  if (windowLabel === "settings") {
    return (
      <div className="app settings-window">
        <SettingsPanel className="settings-panel--open settings-panel--standalone" />
      </div>
    );
  }

  return (
    <div className={`app bar-${windowLabel === "main" ? "top" : windowLabel}`}>
      <DropZone>
        <ShelfBar 
          isVisible={isVisible} 
          onAnimationComplete={onShelfAnimationEnd}
          orientation={windowLabel === "main" ? "horizontal" : "vertical"}
        />
      </DropZone>
      {errorMessage ? (
        <div className="app-toast" role="status" aria-live="polite" onClick={clearError}>
          {errorMessage}
        </div>
      ) : null}
    </div>
  );
}

export default App;
