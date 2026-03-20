import { ShelfBar } from "./components/ShelfBar";
import { DropZone } from "./components/DropZone";
import { useHotzoneState } from "./hooks/useHotzoneState";
import { useShelfStore } from "./stores/shelfStore";
import { useSettingsStore } from "./stores/settingsStore";
import { DEFAULT_SETTINGS } from "./types/settings";
import { getPlatformInfo, getSettings } from "./utils/tauri-bridge";
import { useEffect } from "react";
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

  useEffect(() => {
    getSettings()
      .then((saved) => setSettings({ ...DEFAULT_SETTINGS, ...saved }))
      .catch(() => {});
  }, [setSettings]);

  useEffect(() => {
    getPlatformInfo()
      .then((info) => {
        document.body.classList.add(`platform-${info.os}`);
      })
      .catch(() => {});
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

  return (
    <div className="app">
      <DropZone>
        <ShelfBar isVisible={isVisible} onAnimationComplete={onShelfAnimationEnd} />
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
