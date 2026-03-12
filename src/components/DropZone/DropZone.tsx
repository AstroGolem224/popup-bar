import { type ReactNode } from "react";
import { useDragDrop } from "../../hooks/useDragDrop";
import "./DropZone.css";

export interface DropZoneProps {
  children: ReactNode;
}

export function DropZone({ children }: DropZoneProps) {
  const { isOver, dropHandlers } = useDragDrop("dropzone");

  return (
    <div
      className={`drop-zone ${isOver ? "drop-zone--active" : ""}`}
      {...dropHandlers}
    >
      {children}
    </div>
  );
}
