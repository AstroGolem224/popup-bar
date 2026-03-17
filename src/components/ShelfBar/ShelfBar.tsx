/**
 * ShelfBar — Main popup bar container.
 *
 * Top-level visual component with glassmorphism background.
 * Contains the ShelfGrid for items and the settings gear.
 */
import { useState } from "react";
import { ShelfGrid } from "../ShelfGrid";
import { SettingsPanel } from "../Settings";
import { useShelfItems } from "../../hooks/useShelfItems";
import { useGlassmorphism } from "../../hooks/useGlassmorphism";
import { useSettingsStore } from "../../stores/settingsStore";
import { exitApp, setSettingsExpanded } from "../../utils/tauri-bridge";
import "./ShelfBar.css";

export interface ShelfBarProps {
  className?: string;
  isVisible: boolean;
  onAnimationComplete?: () => void | Promise<void>;
}

export function ShelfBar({ className, isVisible, onAnimationComplete }: ShelfBarProps) {
  const { items, removeItem } = useShelfItems();
  const glassStyle = useGlassmorphism();
  const animationSpeed = useSettingsStore((s) => s.settings.animationSpeed);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const visibilityClass = isVisible ? "shelf-bar--visible" : "shelf-bar--hidden";

  const barStyle = {
    ...glassStyle,
    ["--shelf-bar-duration-show" as string]: `${220 / animationSpeed}ms`,
    ["--shelf-bar-duration-hide" as string]: `${180 / animationSpeed}ms`,
  };

  return (
    <>
      <div
        className={`shelf-bar ${visibilityClass} ${className ?? ""}`}
        style={barStyle}
        onAnimationEnd={(event) => {
          if (event.currentTarget === event.target) {
            void onAnimationComplete?.();
          }
        }}
      >
        {items.length === 0 ? (
          <div className="shelf-bar__empty">
            <p className="shelf-bar__empty-text">
              Drop files, folders, or apps here
            </p>
          </div>
        ) : (
          /* ⚡ Bolt: Pass `removeItem` as a stable reference.
             Using an inline arrow function like `(id) => void removeItem(id)`
             would break memoization in ShelfGrid and force it to re-render. */
          <ShelfGrid items={items} onDeleteItem={removeItem} />
        )}
        <div className="shelf-bar__right-actions">
          <button
            type="button"
            className="shelf-bar__exit-btn"
            onClick={() => void exitApp()}
            aria-label="App beenden"
            title="Beenden"
          >
            ✕
          </button>
          <button
            type="button"
            className="shelf-bar__settings-btn"
            onClick={() => {
              setSettingsOpen(true);
              void setSettingsExpanded(true);
            }}
            aria-label="Einstellungen öffnen"
            title="Einstellungen"
          >
            ⚙
          </button>
        </div>
      </div>

      {settingsOpen ? (
        <>
          <div
            className="shelf-bar__backdrop"
            role="presentation"
            aria-hidden
            onClick={() => {
              setSettingsOpen(false);
              void setSettingsExpanded(false);
            }}
          />
          <div className="shelf-bar__settings-wrap">
            <SettingsPanel
              className="settings-panel--open"
              onClose={() => {
                setSettingsOpen(false);
                void setSettingsExpanded(false);
              }}
            />
          </div>
        </>
      ) : null}
    </>
  );
}
