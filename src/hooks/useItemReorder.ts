/**
 * Hook: Mouse-event-based item reordering.
 *
 * Uses mousedown/mousemove/mouseup instead of HTML5 DnD to avoid
 * conflicts with Tauri's native dragDropEnabled on Windows/Webview2.
 */
import { useCallback, useRef, useEffect, useState } from "react";
import { useShelfStore } from "../stores/shelfStore";
import { reorderShelfItems } from "../utils/tauri-bridge";

export interface UseItemReorderReturn {
  /** Attach to the item's onMouseDown */
  onReorderMouseDown: (itemId: string, event: React.MouseEvent) => void;
  /** The ID of the item currently being dragged (for visual feedback). */
  draggingId: string | null;
  /** The ID of the item currently being hovered over as a drop target. */
  dragOverId: string | null;
}

export function useItemReorder(): UseItemReorderReturn {
  const draggedIdRef = useRef<string | null>(null);
  const startPosRef = useRef<{ x: number; y: number } | null>(null);
  const isDraggingRef = useRef(false);
  const items = useShelfStore((state) => state.items);
  const reorderItems = useShelfStore((state) => state.reorderItems);
  const setError = useShelfStore((state) => state.setError);

  const [draggingId, setDraggingId] = useState<string | null>(null);
  const [dragOverId, setDragOverId] = useState<string | null>(null);

  const DRAG_THRESHOLD = 5; // pixels before drag starts

  const findItemElementAt = useCallback((x: number, y: number): string | null => {
    // 1. Find the element at the point
    let el = document.elementFromPoint(x, y);
    
    // 2. Search upwards to find a shelf item container
    while (el instanceof HTMLElement) {
      if (el.dataset.shelfItemId) {
        return el.dataset.shelfItemId;
      }
      el = el.parentElement;
    }
    return null;
  }, []);

  const handleMouseMoveRef = useRef<(e: MouseEvent) => void>(() => {});
  const handleMouseUpRef = useRef<(e: MouseEvent) => void>(() => {});

  handleMouseMoveRef.current = (e: MouseEvent) => {
    if (!draggedIdRef.current || !startPosRef.current) return;

    const dx = Math.abs(e.clientX - startPosRef.current.x);
    const dy = Math.abs(e.clientY - startPosRef.current.y);

    // Start dragging after threshold
    if (!isDraggingRef.current && (dx > DRAG_THRESHOLD || dy > DRAG_THRESHOLD)) {
      console.log(`[reorder] Drag STARTED for item: ${draggedIdRef.current}`);
      isDraggingRef.current = true;
      setDraggingId(draggedIdRef.current);
      document.body.style.cursor = "grabbing";
      document.body.style.userSelect = "none";
    }

    if (isDraggingRef.current) {
      const targetId = findItemElementAt(e.clientX, e.clientY);
      if (targetId && targetId !== draggedIdRef.current) {
        if (dragOverId !== targetId) {
          console.log(`[reorder] Hovering OVER target: ${targetId}`);
          setDragOverId(targetId);
        }
      } else if (dragOverId) {
        setDragOverId(null);
      }
    }
  };

  handleMouseUpRef.current = async (e: MouseEvent) => {
    document.removeEventListener("mousemove", mouseMoveWrapper);
    document.removeEventListener("mouseup", mouseUpWrapper);
    document.body.style.cursor = "";
    document.body.style.userSelect = "";

    if (isDraggingRef.current && draggedIdRef.current) {
      const targetId = findItemElementAt(e.clientX, e.clientY);
      if (targetId && targetId !== draggedIdRef.current) {
        const currentIds = items.map((item) => item.id);
        const fromIndex = currentIds.indexOf(draggedIdRef.current);
        const targetIndex = currentIds.indexOf(targetId);

        if (fromIndex >= 0 && targetIndex >= 0) {
          const reorderedIds = [...currentIds];
          reorderedIds.splice(fromIndex, 1);
          reorderedIds.splice(targetIndex, 0, draggedIdRef.current);

          reorderItems(reorderedIds);
          try {
            await reorderShelfItems(reorderedIds);
          } catch (error) {
            console.warn("reorder_shelf_items failed", error);
            reorderItems(currentIds);
            setError("Reihenfolge konnte nicht gespeichert werden");
          }
        }
      }
    }

    draggedIdRef.current = null;
    startPosRef.current = null;
    isDraggingRef.current = false;
    setDraggingId(null);
    setDragOverId(null);
  };

  // Stable wrappers for addEventListener/removeEventListener
  const mouseMoveWrapper = useCallback((e: Event) => {
    handleMouseMoveRef.current(e as MouseEvent);
  }, []);

  const mouseUpWrapper = useCallback((e: Event) => {
    void handleMouseUpRef.current(e as MouseEvent);
  }, []);

  const onReorderMouseDown = useCallback((itemId: string, event: React.MouseEvent) => {
    // Only primary mouse button
    if (event.button !== 0) return;
    event.preventDefault();

    draggedIdRef.current = itemId;
    startPosRef.current = { x: event.clientX, y: event.clientY };
    isDraggingRef.current = false;

    document.addEventListener("mousemove", mouseMoveWrapper);
    document.addEventListener("mouseup", mouseUpWrapper);
  }, [mouseMoveWrapper, mouseUpWrapper]);

  // Cleanup listener on unmount
  useEffect(() => {
    return () => {
      document.removeEventListener("mousemove", mouseMoveWrapper);
      document.removeEventListener("mouseup", mouseUpWrapper);
      document.body.style.cursor = "";
      document.body.style.userSelect = "";
    };
  }, [mouseMoveWrapper, mouseUpWrapper]);

  return {
    onReorderMouseDown,
    draggingId,
    dragOverId,
  };
}
