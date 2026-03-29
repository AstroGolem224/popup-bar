import { useCallback } from "react";
import { ShelfGrid } from "../ShelfGrid";
import { useShelfItems } from "../../hooks/useShelfItems";
import type { ShelfItem } from "../../types/shelf";
import { useGlassmorphism } from "../../hooks/useGlassmorphism";
import { useSettingsStore } from "../../stores/settingsStore";
import { exitApp, setSettingsExpanded } from "../../utils/tauri-bridge";
import "./ShelfBar.css";

export interface ShelfBarProps {
  className?: string;
  isVisible: boolean;
  onAnimationComplete?: () => void | Promise<void>;
  orientation?: "horizontal" | "vertical";
}

export function ShelfBar({ 
  className, 
  isVisible, 
  onAnimationComplete,
  orientation = "horizontal"
}: ShelfBarProps) {
  const { items, removeItem, updateItem } = useShelfItems();
  const handleDeleteItem = useCallback((id: string) => { void removeItem(id); }, [removeItem]);
  const handleUpdateItem = useCallback((item: ShelfItem) => updateItem(item), [updateItem]);
  // Optimize: stabilize callbacks passed to ShelfGrid to allow React.memo on items
  const glassStyle = useGlassmorphism();
  const animationSpeed = useSettingsStore((s) => s.settings.animationSpeed);
  const visibilityClass = isVisible ? "shelf-bar--visible" : "shelf-bar--hidden";

  const barStyle = {
    ...glassStyle,
    ["--shelf-bar-duration-show" as string]: `${220 / animationSpeed}ms`,
    ["--shelf-bar-duration-hide" as string]: `${180 / animationSpeed}ms`,
  };

  const alignment = useSettingsStore((s) => s.settings.alignment);

  return (
    <>
      <div
        className={`shelf-bar shelf-bar--${orientation} ${visibilityClass} ${className ?? ""}`}
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
          <ShelfGrid
            items={items}
            alignment={alignment}
            orientation={orientation}
            onDeleteItem={handleDeleteItem}
            onUpdateItem={handleUpdateItem}
          />
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
              void setSettingsExpanded(true);
            }}
            aria-label="Einstellungen öffnen"
            title="Einstellungen"
          >
            ⚙
          </button>
        </div>
      </div>
    </>
  );
}
