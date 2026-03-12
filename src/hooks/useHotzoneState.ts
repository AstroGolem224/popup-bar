import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { EVENTS } from "../types/events";

/** Tracks whether the popup bar should be visible based on hotzone events. */
export function useHotzoneState() {
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    const unlisten = Promise.all([
      listen(EVENTS.HOTZONE_ENTER, () => setIsVisible(true)),
      listen(EVENTS.HOTZONE_LEAVE, () => setIsVisible(false)),
    ]);

    return () => {
      unlisten.then((fns) => fns.forEach((fn) => fn()));
    };
  }, []);

  return { isVisible, setIsVisible };
}
