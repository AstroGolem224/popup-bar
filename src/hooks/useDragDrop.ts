/**
 * Hook: Drag & Drop handlers.
 *
 * Provides event handlers for drag-over, drag-leave, and drop events.
 * Phase 0: Returns no-op handlers.
 * Phase 3: Wired to Tauri file-drop events + DndHandler.
 */
import { type DragEvent } from "react";

interface UseDragDropReturn {
  /** Whether a drag is currently over the drop zone. */
  isDragOver: boolean;
  /** Handle drag enter/over. */
  onDragOver: (e: DragEvent) => void;
  /** Handle drag leave. */
  onDragLeave: (e: DragEvent) => void;
  /** Handle drop. */
  onDrop: (e: DragEvent) => void;
}

export function useDragDrop(): UseDragDropReturn {
  return {
    isDragOver: false,
    onDragOver: (e: DragEvent) => {
      e.preventDefault();
    },
    onDragLeave: (_e: DragEvent) => {
      // Phase 3
    },
    onDrop: (e: DragEvent) => {
      e.preventDefault();
      console.warn("onDrop: not implemented (Phase 3)");
    },
  };
}
