import { useCallback, useEffect, useRef, useState } from "react";
import {
  completeHideWindow,
  completeShowWindow,
  hideWindow,
  showWindow,
  listen,
  getCurrentWindow,
} from "../utils/tauri-bridge";
import { EVENTS } from "../types/events";

interface HotzoneState {
  /** Whether the popup bar should be visible. */
  isVisible: boolean;
  /** Call when shelf show/hide animation finishes. */
  onShelfAnimationEnd: () => Promise<void>;
}

export function useHotzoneState(): HotzoneState {
  const [isVisible, setIsVisible] = useState(false);
  const isVisibleRef = useRef(isVisible);
  const pendingShowTokenRef = useRef<number | null>(null);
  const pendingHideTokenRef = useRef<number | null>(null);

  useEffect(() => {
    isVisibleRef.current = isVisible;
  }, [isVisible]);

  useEffect(() => {
    let unlistenEnter: (() => void) | null = null;
    let unlistenLeave: (() => void) | null = null;

    const setup = async () => {
      const label = getCurrentWindow().label;
      const targetEdge = label === "main" ? "top" : label;
      console.log(`[hotzone] window="${label}" listening for edge="${targetEdge}"`);

      unlistenEnter = await listen<{ edge: string }>(EVENTS.HOTZONE_ENTER, async (event) => {
        console.log(`[hotzone] window="${label}" received ENTER edge="${event.payload.edge}" (want="${targetEdge}")`);
        if (event.payload.edge !== targetEdge) return;
        setIsVisible(true);
        try {
          pendingHideTokenRef.current = null;
          const token = await showWindow();
          pendingShowTokenRef.current = token;
        } catch (error) {
          console.warn("show_window failed", error);
        }
      });

      unlistenLeave = await listen<{ edge: string }>(EVENTS.HOTZONE_LEAVE, async (event) => {
        if (event.payload.edge !== targetEdge) return;
        console.log(`[hotzone] leave edge: ${event.payload.edge}`);
        setIsVisible(false);
        try {
          pendingShowTokenRef.current = null;
          pendingHideTokenRef.current = await hideWindow();
        } catch (error) {
          console.warn("hide_window failed", error);
        }
      });
    };

    void setup();

    return () => {
      unlistenEnter?.();
      unlistenLeave?.();
    };
  }, []);

  const onShelfAnimationEnd = useCallback(async () => {
    try {
      if (isVisibleRef.current) {
        const token = pendingShowTokenRef.current;
        if (token == null) {
          return;
        }
        await completeShowWindow(token);
        pendingShowTokenRef.current = null;
      } else {
        const token = pendingHideTokenRef.current;
        if (token == null) {
          return;
        }
        await completeHideWindow(token);
        pendingHideTokenRef.current = null;
      }
    } catch (error) {
      console.warn("window lifecycle completion failed", error);
    }
  }, []);

  return { isVisible, onShelfAnimationEnd };
}
