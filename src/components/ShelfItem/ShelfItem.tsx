/**
 * ShelfItem — A single item on the shelf (file, folder, app, or URL).
 *
 * Displays the item icon and label. Supports double-click to launch
 * and drag to reorder (Phase 3).
 */
import type { ShelfItem as ShelfItemType } from "../../types/shelf";
import "./ShelfItem.css";

export interface ShelfItemProps {
  /** The shelf item data to render. */
  item: ShelfItemType;
  /** Callback when item is double-clicked (launch). */
  onDoubleClick?: (item: ShelfItemType) => void;
}

export function ShelfItem({ item, onDoubleClick }: ShelfItemProps) {
  return (
    <div
      className={`shelf-item shelf-item--${item.itemType}`}
      title={item.displayName}
      onDoubleClick={() => onDoubleClick?.(item)}
    >
      <div className="shelf-item__icon">
        {/* Phase 4: Icon rendered from icon_cache_key */}
      </div>
      <span className="shelf-item__label">{item.displayName}</span>
    </div>
  );
}
