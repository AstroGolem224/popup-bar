import { ShelfItem as ShelfItemComponent } from "../ShelfItem";
import type { ShelfItem } from "../../types/shelf";
import React from "react";
import { useItemReorder } from "../../hooks/useItemReorder";
import "./ShelfGrid.css";

export interface ShelfGridProps {
  items: ShelfItem[];
  alignment?: "centered" | "start" | "grid";
  orientation?: "horizontal" | "vertical";
  /** When user clicks delete (X) on an item. */
  onDeleteItem?: (id: string) => void | Promise<void>;
}

export const ShelfGrid = React.memo(function ShelfGrid({
  items, 
  alignment = "centered", 
  orientation = "horizontal",
  onDeleteItem 
}: ShelfGridProps) {
  const { onReorderMouseDown, draggingId, dragOverId } = useItemReorder();

  return (
    <div className={`shelf-grid shelf-grid--${alignment} shelf-grid--${orientation}`}>
      {items.map((item) => (
        <ShelfItemComponent
          key={item.id}
          item={item}
          isDragging={draggingId === item.id}
          isDragOver={dragOverId === item.id}
          onReorderMouseDown={onReorderMouseDown}
          onDelete={onDeleteItem}
        />
      ))}
    </div>
  );
});
