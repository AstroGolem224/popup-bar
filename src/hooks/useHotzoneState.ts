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
  const [windowLabel] = useState(() => getCurrentWindow().label);

  useEffect(() => {
    isVisibleRef.current = isVisible;
  }, [isVisible]);

  const requestShow = useCallback(async () => {
    setIsVisible(true);
    try {
      pendingHideTokenRef.current = null;
      pendingShowTokenRef.current = await showWindow();
    } catch (error) {
      console.warn("show_window failed", error);
    }
  }, []);

  const requestHide = useCallback(async () => {
    setIsVisible(false);
    try {
      pendingShowTokenRef.current = null;
      pendingHideTokenRef.current = await hideWindow();
    } catch (error) {
      console.warn("hide_window failed", error);
    }
  }, []);

  useEffect(() => {
    let unlistenEnter: (() => void) | null = null;
    let unlistenLeave: (() => void) | null = null;
    let unlistenToggle: (() => void) | null = null;

    const setup = async () => {
      const targetEdge = windowLabel === "main" ? "top" : windowLabel;

      unlistenEnter = await listen<{ edge: string }>(EVENTS.HOTZONE_ENTER, async (event) => {
        if (event.payload.edge !== targetEdge) return;
        await requestShow();
      });

      unlistenLeave = await listen<{ edge: string }>(EVENTS.HOTZONE_LEAVE, async (event) => {
        if (event.payload.edge !== targetEdge) return;
        await requestHide();
      });

      if (windowLabel === "main") {
        unlistenToggle = await listen(EVENTS.TOGGLE_VISIBILITY, async () => {
          if (isVisibleRef.current) {
            await requestHide();
          } else {
            await requestShow();
          }
        });
      }
    };

    void setup();

    return () => {
      unlistenEnter?.();
      unlistenLeave?.();
      unlistenToggle?.();
    };
  }, [requestHide, requestShow, windowLabel]);

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
