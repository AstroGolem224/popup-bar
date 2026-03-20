import { useCallback, useRef } from "react";
import { useShelfStore } from "../stores/shelfStore";
import { reorderShelfItems } from "../utils/tauri-bridge";

export function useItemReorder() {
  const draggedIdRef = useRef<string | null>(null);
  const items = useShelfStore((state) => state.items);
  const reorderItems = useShelfStore((state) => state.reorderItems);
  const setError = useShelfStore((state) => state.setError);

  const onDragStart = useCallback((itemId: string) => {
    draggedIdRef.current = itemId;
  }, []);

  const onDropOnItem = useCallback(async (targetId: string) => {
    const draggedId = draggedIdRef.current;
    if (!draggedId || draggedId === targetId) {
      return;
    }

    const currentIds = items.map((item) => item.id);
    const fromIndex = currentIds.indexOf(draggedId);
    const targetIndex = currentIds.indexOf(targetId);
    if (fromIndex < 0 || targetIndex < 0) {
      return;
    }

    const reorderedIds = [...currentIds];
    reorderedIds.splice(fromIndex, 1);
    reorderedIds.splice(targetIndex, 0, draggedId);

    reorderItems(reorderedIds);
    draggedIdRef.current = null;
    try {
      await reorderShelfItems(reorderedIds);
    } catch (error) {
      console.warn("reorder_shelf_items failed", error);
      reorderItems(currentIds);
      setError("reihenfolge konnte nicht gespeichert werden");
    }
  }, [items, reorderItems, setError]);

  return {
    onDragStart,
    onDropOnItem,
  };
}
