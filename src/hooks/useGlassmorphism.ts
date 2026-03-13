/**
 * Hook: Glassmorphism CSS properties for the shelf bar.
 * Uses CSS variables from glassmorphism.css (platform-tuned).
 */
import type { CSSProperties } from "react";

export function useGlassmorphism(): CSSProperties {
  return {
    background: "var(--shelf-bar-bg)",
    backdropFilter: "var(--shelf-bar-blur)",
    WebkitBackdropFilter: "var(--shelf-bar-blur)",
    border: "var(--shelf-bar-border)",
    boxShadow: "var(--shelf-bar-shadow)",
  };
}
