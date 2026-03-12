import { useState, type DragEventHandler } from "react";

/** Manages drag-and-drop state for a shelf item or drop zone. */
export function useDragDrop(_targetId: string) {
  const [isOver, setIsOver] = useState(false);

  const dragHandlers = {
    draggable: true,
    onDragStart: (() => {
      // TODO: Set drag data
    }) as DragEventHandler,
    onDragEnd: (() => {
      // TODO: Clean up drag state
    }) as DragEventHandler,
  };

  const dropHandlers = {
    onDragOver: ((e: React.DragEvent) => {
      e.preventDefault();
      setIsOver(true);
    }) as DragEventHandler,
    onDragLeave: (() => {
      setIsOver(false);
    }) as DragEventHandler,
    onDrop: ((e: React.DragEvent) => {
      e.preventDefault();
      setIsOver(false);
      // TODO: Process drop
    }) as DragEventHandler,
  };

  return { isOver, dragHandlers, dropHandlers };
}
