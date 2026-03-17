/**
 * DropZone — Wraps children with drag & drop event handling.
 *
 * Accepts external drops (files, folders, URLs) and forwards
 * them to the DndHandler. Phase 3: Full implementation.
 */
import { type ReactNode } from "react";
import { useDragDrop } from "../../hooks/useDragDrop";
import "./DropZone.css";

export interface DropZoneProps {
  children: ReactNode;
}

export function DropZone({ children }: DropZoneProps) {
  const { isDragOver, dragHint, onDragOver, onDragLeave, onDrop } = useDragDrop();

  return (
    <div
      className={`drop-zone ${isDragOver ? "drop-zone--active" : ""}`}
      onDragOver={onDragOver}
      onDragLeave={onDragLeave}
      onDrop={onDrop}
      role="region"
      aria-label="Drop-Zone für Dateien und Ordner"
    >
      {isDragOver ? <div className="drop-zone__overlay" aria-live="polite">{dragHint}</div> : null}
      {children}
    </div>
  );
}
