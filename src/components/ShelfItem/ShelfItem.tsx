import React from 'react';
/**
 * ShelfItem — A single item on the shelf (file, folder, app, or URL).
 *
 * Displays the item icon and label. Supports click to launch
 * and mouse-based reordering (avoids HTML5 DnD / Tauri conflicts).
 * Icons loaded via get_icon_data (base64) to avoid asset protocol scope.
 */
import type { ShelfItem as ShelfItemType } from "../../types/shelf";
import type { CSSProperties } from "react";
import { useEffect, useRef, useState } from "react";
import {
  getIconDataUrl,
  openShelfItemViaLauncher,
} from "../../utils/tauri-bridge";
import { getCachedDataUrl } from "../../utils/media-cache";
import "./ShelfItem.css";

const ACTIVATE_DEBOUNCE_MS = 400;

export interface ShelfItemProps {
  /** The shelf item data to render. */
  item: ShelfItemType;
  /** Whether this item is currently being dragged. */
  isDragging?: boolean;
  /** Whether another item is being dragged over this one. */
  isDragOver?: boolean;
  /** Callback when mouse starts reorder drag. */
  onReorderMouseDown?: (id: string, event: React.MouseEvent) => void;
  /** Callback when delete (X) is clicked. */
  onDelete?: (id: string) => void | Promise<void>;
  /** Inline positioning style from the layout system. */
  style?: CSSProperties;
  /** Explicit horizontal position to avoid breaking memoization with inline style objects. */
  positionX?: number;
  /** Explicit vertical position to avoid breaking memoization with inline style objects. */
  positionY?: number;
  /** Suppresses accidental open directly after dragging. */
  activationBlocked?: boolean;
}

// Optimize: wrap ShelfItem in React.memo to prevent unnecessary re-renders
// when other items are dragged in the ShelfGrid
export const ShelfItem = React.memo(function ShelfItem({
  item,
  isDragging = false,
  isDragOver = false,
  onReorderMouseDown,
  onDelete,
  style,
  positionX,
  positionY,
  activationBlocked = false,
}: ShelfItemProps) {
  const [iconLoadFailed, setIconLoadFailed] = useState(false);
  const [iconDataUrl, setIconDataUrl] = useState<string | null>(null);
  const lastActivateRef = useRef(0);

  useEffect(() => {
    let isMounted = true;

    setIconLoadFailed(false);
    setIconDataUrl(null);
    if (!item.iconCacheKey) {
      return () => {
        isMounted = false;
      };
    }

    getCachedDataUrl(`icon:${item.iconCacheKey}`, () => getIconDataUrl(item.iconCacheKey))
      .then((url) => {
        if (isMounted) {
          setIconDataUrl(url);
        }
      })
      .catch(() => {
        if (isMounted) {
          setIconLoadFailed(true);
        }
      });

    return () => {
      isMounted = false;
    };
  }, [item.iconCacheKey]);

  const fallbackIcon = item.itemType === "folder"
    ? "📁"
    : item.itemType === "app"
      ? "🧩"
      : item.itemType === "url"
        ? "🌐"
        : "📄";

  const handleActivate = async () => {
    if (activationBlocked) return;
    const now = Date.now();
    if (now - lastActivateRef.current < ACTIVATE_DEBOUNCE_MS) return;
    lastActivateRef.current = now;

    try {
      await openShelfItemViaLauncher(item.itemType, item.path);
    } catch (error) {
      console.warn("failed to open shelf item", error);
    }
  };

  const classNames = [
    "shelf-item",
    `shelf-item--${item.itemType}`,
    isDragging ? "shelf-item--dragging" : "",
    isDragOver ? "shelf-item--drag-over" : "",
  ].filter(Boolean).join(" ");

  const computedStyle: CSSProperties = { ...style };
  if (positionX !== undefined) {
    computedStyle.left = `${positionX}px`;
    computedStyle.position = computedStyle.position || "absolute";
  }
  if (positionY !== undefined) {
    computedStyle.top = `${positionY}px`;
    computedStyle.position = computedStyle.position || "absolute";
  }

  return (
    <div
      className={classNames}
      style={Object.keys(computedStyle).length > 0 ? computedStyle : undefined}
      data-shelf-item-id={item.id}
      title={item.displayName}
      tabIndex={0}
      onClick={() => {
        void handleActivate();
      }}
      onKeyDown={(event) => {
        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          void handleActivate();
        }
      }}
      onMouseDown={(e) => {
        onReorderMouseDown?.(item.id, e);
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
          onMouseDown={(e) => e.stopPropagation()}
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
}
);
