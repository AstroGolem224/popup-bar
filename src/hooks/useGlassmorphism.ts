/**
 * Hook: Glassmorphism CSS properties.
 *
 * Returns a CSSProperties object for inline glassmorphism styling.
 * Reads blur intensity and tint color from settings store.
 *
 * Phase 0: Returns hardcoded defaults.
 * Phase 5: Reads from settings store dynamically.
 */
import type { CSSProperties } from "react";

export function useGlassmorphism(): CSSProperties {
  return {
    background: "rgba(255, 255, 255, 0.08)",
    backdropFilter: "blur(20px)",
    WebkitBackdropFilter: "blur(20px)",
    border: "1px solid rgba(255, 255, 255, 0.15)",
    boxShadow: "0 8px 32px rgba(0, 0, 0, 0.3)",
  };
}
