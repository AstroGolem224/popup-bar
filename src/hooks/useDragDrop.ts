/**
 * Hook: Drag & Drop handlers.
 *
 * Uses Tauri's onDragDropEvent for OS-native file/folder/app drops (paths).
 * HTML5 handlers kept for URL paste/drop where supported.
 */
import { useCallback, useEffect, useRef, useState, type DragEvent } from "react";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { addDroppedPaths, addShelfItem } from "../utils/tauri-bridge";
import { useShelfStore } from "../stores/shelfStore";

/** Convert file:// URL or path string to a normal filesystem path for the backend. */
function toFilesystemPath(s: string): string {
  const t = s.trim();
  if (t.startsWith("file:///")) {
    try {
      return decodeURIComponent(t.slice(7).replace(/^\/+/, ""));
    } catch {
      return t.slice(7).replace(/^\/+/, "");
    }
  }
  if (t.startsWith("file://")) {
    try {
      return decodeURIComponent(t.slice(6));
    } catch {
      return t.slice(6);
    }
  }
  return t;
}

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

  const inferDragHint = (event: DragEvent) => {
    const types = Array.from(event.dataTransfer?.types ?? []);
    if (types.includes("text/uri-list")) return "Link hinzufügen";
    if (types.includes("Files")) return "Hier ablegen";
    return "Dateien, Ordner oder Links hier ablegen";
  };

  const onDragOver = useCallback((e: DragEvent) => {
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
        const createdUrlItem = await addShelfItem(droppedUrl, "url");
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

  // Tauri native drag-drop (files, folders, app shortcuts) — primary path when dragDropEnabled
  const lastDropRef = useRef<{ paths: string[]; at: number }>({ paths: [], at: 0 });
  const DROP_DEBOUNCE_MS = 800;

  useEffect(() => {
    let unlisten: (() => void) | undefined;

    getCurrentWebviewWindow()
      .onDragDropEvent((event) => {
        const p = event.payload;
        if (p.type === "enter" || p.type === "over") {
          setIsDragOver(true);
          setDragHint("Hier ablegen");
        } else if (p.type === "leave") {
          setIsDragOver(false);
          setDragHint("Dateien, Ordner oder Links hier ablegen");
        } else if (p.type === "drop" && p.paths?.length) {
          setIsDragOver(false);
          const paths = p.paths.map(toFilesystemPath);
          const now = Date.now();
          const last = lastDropRef.current;
          if (
            last.paths.length === paths.length &&
            last.paths.every((pp, i) => pp === paths[i]) &&
            now - last.at < DROP_DEBOUNCE_MS
          ) {
            return;
          }
          lastDropRef.current = { paths, at: now };
          addDroppedPaths(paths)
            .then((created) => {
              addItems(created);
            })
            .catch((err: unknown) => {
              const msg =
                typeof err === "string"
                  ? err
                  : err instanceof Error
                    ? err.message
                    : "Ablegen fehlgeschlagen";
              console.warn("add_dropped_paths failed", err);
              setError(msg);
            });
        }
      })
      .then((fn) => {
        unlisten = fn;
      })
      .catch((err) => {
        console.warn("onDragDropEvent register failed", err);
      });

    return () => {
      unlisten?.();
    };
  }, [addItems, setError]);

  return {
    isDragOver,
    dragHint,
    onDragOver,
    onDragLeave,
    onDrop,
  };
}
