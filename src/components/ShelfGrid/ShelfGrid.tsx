import React from "react";
import { ShelfItem as ShelfItemComponent } from "../ShelfItem";
import type { ShelfItem } from "../../types/shelf";
import { useItemReorder } from "../../hooks/useItemReorder";
import "./ShelfGrid.css";

export interface ShelfGridProps {
  items: ShelfItem[];
  /** When user clicks delete (X) on an item. */
  onDeleteItem?: (id: string) => void | Promise<void>;
}

// ⚡ Bolt: Wrapped ShelfGrid in React.memo. Together with stabilized callbacks
// passed from useItemReorder and useShelfItems, this prevents the entire grid
// and its children from re-rendering whenever the parent ShelfBar updates.
export const ShelfGrid = React.memo(function ShelfGrid({ items, onDeleteItem }: ShelfGridProps) {
  const { onDragStart, onDropOnItem } = useItemReorder();

  return (
    <div className="shelf-grid">
      {items.map((item) => (
        <ShelfItemComponent
          key={item.id}
          item={item}
          onDragStartItem={onDragStart}
          onDropOnItem={onDropOnItem}
          onDelete={onDeleteItem}
        />
      ))}
    </div>
  );
});
