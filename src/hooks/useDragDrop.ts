/**
 * Hook: Drag & Drop handlers.
 *
 * Uses Tauri's onDragDropEvent for OS-native file/folder/app drops (paths).
 * HTML5 handlers kept for URL paste/drop where supported.
 */
import { useCallback, useEffect, useState, type DragEvent } from "react";
import { addShelfItem, getCurrentWebviewWindow } from "../utils/tauri-bridge";
import { useShelfStore } from "../stores/shelfStore";


interface UseDragDropReturn {
  isDragOver: boolean;
  dragHint: string;
  onDragOver: (e: DragEvent) => void;
  onDragLeave: (e: DragEvent) => void;
  onDrop: (e: DragEvent) => void | Promise<void>;
}

export function useDragDrop(): UseDragDropReturn {
  const [isDragOver, setIsDragOver] = useState(false);
  const [dragHint, setDragHint] = useState("Dateien, Ordner oder Links hier ablegen");
  const addItems = useShelfStore((state) => state.addItems);
  const addItem = useShelfStore((state) => state.addItem);
  const setError = useShelfStore((state) => state.setError);
  const [windowLabel, setWindowLabel] = useState("main");

  useEffect(() => {
    try {
      setWindowLabel(getCurrentWebviewWindow().label);
    } catch (e) {
      console.warn("[drag-drop] Failed to detect window label", e);
    }
  }, []);

  const inferDragHint = (event: DragEvent) => {
    const types = Array.from(event.dataTransfer?.types ?? []);
    if (types.includes("text/uri-list")) return "Link hinzufügen";
    if (types.includes("Files")) return "Hier ablegen";
    return "Dateien, Ordner oder Links hier ablegen";
  };

  const onDragOver = useCallback((e: DragEvent) => {
    // Only handle external file/link drops
    if (!e.dataTransfer.types.includes("Files") && !e.dataTransfer.types.includes("text/uri-list")) {
      return;
    }

    e.preventDefault();
    setDragHint(inferDragHint(e));
    if (!isDragOver) setIsDragOver(true);
  }, [isDragOver]);

  const onDragLeave = useCallback((e: DragEvent) => {
    const related = e.relatedTarget as Node | null;
    if (related && e.currentTarget.contains(related)) return;
    setIsDragOver(false);
    setDragHint("Dateien, Ordner oder Links hier ablegen");
  }, []);

  const onDrop = useCallback(async (e: DragEvent) => {
    e.preventDefault();
    setIsDragOver(false);

    try {
      // Datei-/Ordner-Drops nur über Tauri onDragDropEvent verarbeiten (vermeidet doppelte Ablage).
      // Hier nur URL-Daten aus text/uri-list oder text/plain.
      const uriList = (e.dataTransfer?.getData("text/uri-list") ?? "").trim();
      const plainText = (e.dataTransfer?.getData("text/plain") ?? "").trim();
      const droppedUrl = uriList || plainText;

      if (!droppedUrl) return;
      if (droppedUrl.startsWith("file://")) return;
      if (/^https?:\/\//i.test(droppedUrl)) {
        const createdUrlItem = await addShelfItem(droppedUrl, "url", windowLabel);
        addItem(createdUrlItem);
      }
    } catch (error: unknown) {
      const msg =
        typeof error === "string"
          ? error
          : error instanceof Error
            ? error.message
            : "Ablegen fehlgeschlagen";
      console.warn("drop handling failed", error);
      setError(msg);
    }
  }, [addItem, addItems, setError]);

  // Tauri native drag-drop (files, folders, app shortcuts)
  // MOVED TO RUST (lib.rs) to ensure consistent behavior across all bar windows
  // and handle drops even when windows are hidden/loading.
  useEffect(() => {
    // No-op: file drops handled by setup_drop_handler in src-tauri/src/lib.rs
  }, []);

  return {
    isDragOver,
    dragHint,
    onDragOver,
    onDragLeave,
    onDrop,
  };
}
