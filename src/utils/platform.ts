/**
 * Platform detection utilities.
 *
 * Uses navigator.userAgent as a lightweight fallback
 * that works without additional Tauri plugins.
 */

export type Platform = "windows" | "macos" | "linux" | "unknown";

/** Detect the current operating system from the user agent. */
export function detectPlatform(): Platform {
  const ua = navigator.userAgent.toLowerCase();
  if (ua.includes("win")) return "windows";
  if (ua.includes("mac")) return "macos";
  if (ua.includes("linux")) return "linux";
  return "unknown";
}

/** Check if the current platform supports native vibrancy. */
export function supportsNativeVibrancy(): boolean {
  const p = detectPlatform();
  return p === "windows" || p === "macos";
}
