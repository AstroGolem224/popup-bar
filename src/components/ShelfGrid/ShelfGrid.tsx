import { useEffect, useRef, useState, useCallback } from "react";
import { ShelfItem as ShelfItemComponent } from "../ShelfItem";
import type { ShelfItem } from "../../types/shelf";
import { useItemReorder, type ItemPosition } from "../../hooks/useItemReorder";
import "./ShelfGrid.css";

const ITEM_SIZE = 52;
const ITEM_GAP = 12;
const GRID_STEP = 64;
const GRID_PADDING = 8;
const ACTIONS_GUTTER = 72;

export interface ShelfGridProps {
  items: ShelfItem[];
  alignment?: "centered" | "start" | "grid";
  orientation?: "horizontal" | "vertical";
  onDeleteItem?: (id: string) => void | Promise<void>;
  onUpdateItem: (item: ShelfItem) => Promise<void>;
}

function clamp(value: number, min: number, max: number) {
  return Math.min(Math.max(value, min), max);
}

function isLegacyPosition(item: ShelfItem, totalItems: number) {
  return item.position.y <= 0 && item.position.x <= totalItems + 2 && Number.isInteger(item.position.x);
}

function buildAutoLayoutPosition(
  index: number,
  totalItems: number,
  alignment: "centered" | "start" | "grid",
  orientation: "horizontal" | "vertical",
  containerSize: { width: number; height: number },
): ItemPosition {
  const step = alignment === "grid" ? GRID_STEP : ITEM_SIZE + ITEM_GAP;
  const usableWidth = Math.max(step, containerSize.width - ACTIONS_GUTTER);
  const usableHeight = Math.max(step, containerSize.height);

  if (orientation === "vertical") {
    const rows = Math.max(1, Math.floor((usableHeight - GRID_PADDING) / step));
    const column = Math.floor(index / rows);
    const row = index % rows;
    const x = GRID_PADDING + column * step;
    const y = GRID_PADDING + row * step;
    return { x, y };
  }

  const columns = Math.max(1, Math.floor((usableWidth - GRID_PADDING) / step));
  const row = Math.floor(index / columns);
  const column = index % columns;
  const itemsInRow = Math.min(columns, totalItems - row * columns);
  const rowWidth = (itemsInRow - 1) * step + ITEM_SIZE;
  const startX = alignment === "centered"
    ? Math.max(GRID_PADDING, Math.floor((usableWidth - rowWidth) / 2))
    : GRID_PADDING;

  return {
    x: startX + column * step,
    y: GRID_PADDING + row * step,
  };
}

function normalizeManualPosition(
  position: ItemPosition,
  containerSize: { width: number; height: number },
) {
  return {
    x: clamp(position.x, 0, Math.max(0, containerSize.width - ITEM_SIZE - ACTIONS_GUTTER)),
    y: clamp(position.y, 1, Math.max(1, containerSize.height - ITEM_SIZE)),
  };
}

export function ShelfGrid({
  items,
  alignment = "centered",
  orientation = "horizontal",
  onDeleteItem,
  onUpdateItem,
}: ShelfGridProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const [containerSize, setContainerSize] = useState({ width: 480, height: 72 });

  useEffect(() => {
    const element = containerRef.current;
    if (!element) {
      return;
    }

    const updateSize = () => {
      setContainerSize({
        width: element.clientWidth || 480,
        height: element.clientHeight || 72,
      });
    };

    updateSize();
    const observer = new ResizeObserver(() => updateSize());
    observer.observe(element);

    return () => {
      observer.disconnect();
    };
  }, []);

  const basePositions = items.reduce<Record<string, ItemPosition>>((positions, item, index) => {
    positions[item.id] = isLegacyPosition(item, items.length)
      ? buildAutoLayoutPosition(index, items.length, alignment, orientation, containerSize)
      : normalizeManualPosition(item.position, containerSize);
    return positions;
  }, {});

  const onUpdateItemRef = useRef(onUpdateItem);
  onUpdateItemRef.current = onUpdateItem;

  const { onReorderMouseDown, draggingId, dragPositions, activationBlockedId } = useItemReorder({
    items,
    resolvedPositions: basePositions,
    containerSize,
    alignment,
    onCommitPosition: async (itemId, position) => {
      const item = items.find((entry) => entry.id === itemId);
      if (!item) {
        return;
      }

      await onUpdateItemRef.current({
        ...item,
        position,
      });
    },
  });

  const resolvedPositions = { ...basePositions, ...dragPositions };
  const orderedItems = [...items].sort((left, right) => {
    const leftPosition = resolvedPositions[left.id] ?? { x: 0, y: 0 };
    const rightPosition = resolvedPositions[right.id] ?? { x: 0, y: 0 };
    if (leftPosition.y !== rightPosition.y) {
      return leftPosition.y - rightPosition.y;
    }
    return leftPosition.x - rightPosition.x;
  });

  const onDeleteItemRef = useRef(onDeleteItem);
  onDeleteItemRef.current = onDeleteItem;

  const handleDeleteItem = useCallback((id: string) => {
    if (onDeleteItemRef.current) {
      void onDeleteItemRef.current(id);
    }
  }, []);

  return (
    <div
      ref={containerRef}
      className={`shelf-grid shelf-grid--${alignment} shelf-grid--${orientation}`}
    >
      {orderedItems.map((item) => {
        const position = resolvedPositions[item.id] ?? { x: 0, y: 1 };

        return (
          <ShelfItemComponent
            key={item.id}
            item={item}
            positionX={position.x}
            positionY={position.y}
            isDragging={draggingId === item.id}
            onReorderMouseDown={onReorderMouseDown}
            onDelete={onDeleteItem ? handleDeleteItem : undefined}
            activationBlocked={activationBlockedId === item.id}
          />
        );
      })}
    </div>
  );
}
