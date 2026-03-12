import type { ShelfItem as ShelfItemType } from "../../types/shelf";
import { useDragDrop } from "../../hooks/useDragDrop";
import "./ShelfItem.css";

export interface ShelfItemProps {
  item: ShelfItemType;
  onDoubleClick?: (item: ShelfItemType) => void;
}

export function ShelfItem({ item, onDoubleClick }: ShelfItemProps) {
  const { dragHandlers } = useDragDrop(item.id);

  return (
    <div
      className={`shelf-item shelf-item--${item.itemType}`}
      title={item.displayName}
      onDoubleClick={() => onDoubleClick?.(item)}
      {...dragHandlers}
    >
      <div className="shelf-item__icon">
        {/* Icon rendered from cache key */}
      </div>
      <span className="shelf-item__label">{item.displayName}</span>
    </div>
  );
}
