/**
 * Hook: Hotzone visibility state.
 *
 * Listens to hotzone:enter / hotzone:leave events from the Rust backend
 * and exposes whether the popup bar should be visible.
 *
 * Phase 0: Always returns visible (true).
 * Phase 1: Wired to actual Tauri events.
 */
import { useState } from "react";

interface HotzoneState {
  /** Whether the popup bar should be visible. */
  isVisible: boolean;
}

export function useHotzoneState(): HotzoneState {
  // Phase 0: Always visible for development.
  // Phase 1: Replace with Tauri event listener.
  const [isVisible] = useState(true);

  return { isVisible };
}
