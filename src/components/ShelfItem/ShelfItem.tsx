/**
 * ShelfItem — A single item on the shelf (file, folder, app, or URL).
 *
 * Displays the item icon and label. Supports double-click to launch
 * and drag to reorder (Phase 3).
 * Icons loaded via get_icon_data (base64) to avoid asset protocol scope.
 */
import type { ShelfItem as ShelfItemType } from "../../types/shelf";
import { useEffect, useRef, useState, memo } from "react";
import {
  getIconDataUrl,
  openShelfItemViaLauncher,
} from "../../utils/tauri-bridge";
import "./ShelfItem.css";

const ACTIVATE_DEBOUNCE_MS = 400;

export interface ShelfItemProps {
  /** The shelf item data to render. */
  item: ShelfItemType;
  /** Callback when item is double-clicked (launch). */
  onDoubleClick?: (item: ShelfItemType) => void;
  /** Callback when dragging starts. */
  onDragStartItem?: (id: string) => void;
  /** Callback when another item is dropped onto this item. */
  onDropOnItem?: (id: string) => void | Promise<void>;
  /** Callback when delete (X) is clicked. Preferred over drag-to-trash on Windows (HTML5 drag broken with Tauri dragDropEnabled). */
  onDelete?: (id: string) => void | Promise<void>;
}

export const ShelfItem = memo(function ShelfItem({
  item,
  onDoubleClick,
  onDragStartItem,
  onDropOnItem,
  onDelete,
}: ShelfItemProps) {
  const [iconLoadFailed, setIconLoadFailed] = useState(false);
  const [iconDataUrl, setIconDataUrl] = useState<string | null>(null);
  const lastActivateRef = useRef(0);

  useEffect(() => {
    setIconLoadFailed(false);
    setIconDataUrl(null);
    if (!item.iconCacheKey) return;
    getIconDataUrl(item.iconCacheKey)
      .then(setIconDataUrl)
      .catch(() => setIconLoadFailed(true));
  }, [item.iconCacheKey]);

  const fallbackIcon = item.itemType === "folder"
    ? "📁"
    : item.itemType === "app"
      ? "🧩"
      : item.itemType === "url"
        ? "🌐"
        : "📄";

  const handleActivate = async () => {
    if (onDoubleClick) {
      onDoubleClick(item);
      return;
    }
    const now = Date.now();
    if (now - lastActivateRef.current < ACTIVATE_DEBOUNCE_MS) return;
    lastActivateRef.current = now;

    try {
      await openShelfItemViaLauncher(item.itemType, item.path);
    } catch (error) {
      console.warn("failed to open shelf item", error);
    }
  };

  return (
    <div
      className={`shelf-item shelf-item--${item.itemType}`}
      title={item.displayName}
      tabIndex={0}
      draggable
      onClick={() => {
        void handleActivate();
      }}
      onKeyDown={(event) => {
        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          void handleActivate();
        }
      }}
      onDragStart={(e) => {
        onDragStartItem?.(item.id);
        e.dataTransfer.setData("application/x-popup-bar-item-id", item.id);
        e.dataTransfer.setData("text/plain", item.id);
        e.dataTransfer.effectAllowed = "move";
      }}
      onDragOver={(event) => event.preventDefault()}
      onDrop={() => {
        void onDropOnItem?.(item.id);
      }}
    >
      {onDelete ? (
        <button
          type="button"
          className="shelf-item__delete"
          onClick={(e) => {
            e.stopPropagation();
            e.preventDefault();
            void onDelete(item.id);
          }}
          aria-label="Item entfernen"
          title="Entfernen"
        >
          ×
        </button>
      ) : null}
      <div className="shelf-item__icon" title={item.displayName}>
        {iconDataUrl && !iconLoadFailed ? (
          <img
            className="shelf-item__icon-img"
            src={iconDataUrl}
            alt=""
            draggable={false}
            onError={() => setIconLoadFailed(true)}
          />
        ) : (
          <span className="shelf-item__icon-fallback">{fallbackIcon}</span>
        )}
      </div>
    </div>
  );
});
