/**
 * Hook: persisted item positioning.
 *
 * Dragging updates a local preview immediately and commits the final
 * item position once the mouse is released.
 */
import { useCallback, useEffect, useRef, useState } from "react";
import type { ShelfItem } from "../types/shelf";

export interface ItemPosition {
  x: number;
  y: number;
}

interface UseItemReorderOptions {
  items: ShelfItem[];
  resolvedPositions: Record<string, ItemPosition>;
  containerSize: { width: number; height: number };
  alignment: "centered" | "start" | "grid";
  onCommitPosition: (itemId: string, position: ItemPosition) => Promise<void>;
}

export interface UseItemReorderReturn {
  onReorderMouseDown: (itemId: string, event: React.MouseEvent) => void;
  draggingId: string | null;
  dragPositions: Record<string, ItemPosition>;
  activationBlockedId: string | null;
}

const DRAG_THRESHOLD = 5;
const ITEM_SIZE = 52;
const GRID_STEP = 64;
const MIN_MANUAL_Y = 1;

interface ActiveDragState {
  itemId: string;
  mouseStart: { x: number; y: number };
  itemStart: ItemPosition;
  moved: boolean;
}

function clamp(value: number, min: number, max: number) {
  return Math.min(Math.max(value, min), max);
}

function roundPosition(value: number) {
  return Math.round(value * 10) / 10;
}

function normalizePosition(
  position: ItemPosition,
  containerSize: { width: number; height: number },
  alignment: "centered" | "start" | "grid",
): ItemPosition {
  const maxX = Math.max(0, containerSize.width - ITEM_SIZE);
  const maxY = Math.max(MIN_MANUAL_Y, containerSize.height - ITEM_SIZE);

  let nextX = clamp(position.x, 0, maxX);
  let nextY = clamp(position.y, MIN_MANUAL_Y, maxY);

  if (alignment === "grid") {
    nextX = clamp(Math.round(nextX / GRID_STEP) * GRID_STEP, 0, maxX);
    nextY = clamp(Math.round(nextY / GRID_STEP) * GRID_STEP, MIN_MANUAL_Y, maxY);
  }

  return {
    x: roundPosition(nextX),
    y: roundPosition(nextY),
  };
}

export function useItemReorder({
  items,
  resolvedPositions,
  containerSize,
  alignment,
  onCommitPosition,
}: UseItemReorderOptions): UseItemReorderReturn {
  const activeDragRef = useRef<ActiveDragState | null>(null);
  const [draggingId, setDraggingId] = useState<string | null>(null);
  const [dragPositions, setDragPositions] = useState<Record<string, ItemPosition>>({});
  const [activationBlockedId, setActivationBlockedId] = useState<string | null>(null);
  const blockResetTimerRef = useRef<number | null>(null);
  const mouseMoveHandlerRef = useRef<(event: MouseEvent) => void>(() => {});
  const mouseUpHandlerRef = useRef<(event: MouseEvent) => void>(() => {});

  const latestStateRef = useRef({ items, dragPositions, resolvedPositions });
  useEffect(() => {
    latestStateRef.current = { items, dragPositions, resolvedPositions };
  }, [items, dragPositions, resolvedPositions]);

  const clearInteractionStyles = useCallback(() => {
    document.body.style.cursor = "";
    document.body.style.userSelect = "";
  }, []);

  const releaseActivationBlock = useCallback(() => {
    if (blockResetTimerRef.current != null) {
      window.clearTimeout(blockResetTimerRef.current);
      blockResetTimerRef.current = null;
    }
  }, []);

  mouseMoveHandlerRef.current = (event: MouseEvent) => {
    const activeDrag = activeDragRef.current;
    if (!activeDrag) {
      return;
    }

    const deltaX = event.clientX - activeDrag.mouseStart.x;
    const deltaY = event.clientY - activeDrag.mouseStart.y;

    if (!activeDrag.moved && Math.hypot(deltaX, deltaY) <= DRAG_THRESHOLD) {
      return;
    }

    if (!activeDrag.moved) {
      activeDrag.moved = true;
      setDraggingId(activeDrag.itemId);
      document.body.style.cursor = "grabbing";
      document.body.style.userSelect = "none";
    }

    const nextPosition = normalizePosition(
      {
        x: activeDrag.itemStart.x + deltaX,
        y: activeDrag.itemStart.y + deltaY,
      },
      containerSize,
      alignment,
    );

    setDragPositions((previous) => ({
      ...previous,
      [activeDrag.itemId]: nextPosition,
    }));
  };

  mouseUpHandlerRef.current = () => {
    document.removeEventListener("mousemove", mouseMoveWrapper);
    document.removeEventListener("mouseup", mouseUpWrapper);
    clearInteractionStyles();

    const activeDrag = activeDragRef.current;
    activeDragRef.current = null;

    if (!activeDrag) {
      setDraggingId(null);
      return;
    }

    const nextPosition = normalizePosition(
      dragPositions[activeDrag.itemId] ?? activeDrag.itemStart,
      containerSize,
      alignment,
    );

    if (activeDrag.moved) {
      setActivationBlockedId(activeDrag.itemId);
      releaseActivationBlock();
      blockResetTimerRef.current = window.setTimeout(() => {
        setActivationBlockedId((current) => (current === activeDrag.itemId ? null : current));
        blockResetTimerRef.current = null;
      }, 180);

      void onCommitPosition(activeDrag.itemId, nextPosition).finally(() => {
        setDragPositions((previous) => {
          const { [activeDrag.itemId]: _removed, ...rest } = previous;
          return rest;
        });
      });
    }

    setDraggingId(null);
  };

  const mouseMoveWrapper = useCallback((event: Event) => {
    mouseMoveHandlerRef.current(event as MouseEvent);
  }, []);

  const mouseUpWrapper = useCallback((event: Event) => {
    mouseUpHandlerRef.current(event as MouseEvent);
  }, []);

  const onReorderMouseDown = useCallback(
    (itemId: string, event: React.MouseEvent) => {
      if (event.button !== 0) {
        return;
      }

      const { items: currentItems, dragPositions: currentDragPositions, resolvedPositions: currentResolvedPositions } = latestStateRef.current;
      const item = currentItems.find((entry) => entry.id === itemId);
      if (!item) {
        return;
      }

      event.preventDefault();
      const itemStart = currentDragPositions[itemId] ?? currentResolvedPositions[itemId] ?? {
        x: item.position.x,
        y: Math.max(item.position.y, MIN_MANUAL_Y),
      };

      activeDragRef.current = {
        itemId,
        mouseStart: { x: event.clientX, y: event.clientY },
        itemStart,
        moved: false,
      };

      document.addEventListener("mousemove", mouseMoveWrapper);
      document.addEventListener("mouseup", mouseUpWrapper);
    },
    [mouseMoveWrapper, mouseUpWrapper],
  );

  useEffect(() => {
    return () => {
      document.removeEventListener("mousemove", mouseMoveWrapper);
      document.removeEventListener("mouseup", mouseUpWrapper);
      clearInteractionStyles();
      releaseActivationBlock();
    };
  }, [clearInteractionStyles, mouseMoveWrapper, mouseUpWrapper, releaseActivationBlock]);

  return {
    onReorderMouseDown,
    draggingId,
    dragPositions,
    activationBlockedId,
  };
}
