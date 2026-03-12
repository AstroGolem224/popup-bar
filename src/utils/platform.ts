import { platform as getPlatform } from "@tauri-apps/plugin-os";

export type Platform = "windows" | "macos" | "linux" | "unknown";

/** Detect the current operating system. */
export function detectPlatform(): Platform {
  try {
    const os = getPlatform();
    if (os === "windows") return "windows";
    if (os === "macos") return "macos";
    if (os === "linux") return "linux";
    return "unknown";
  } catch {
    return "unknown";
  }
}

/** Check if the current platform supports native vibrancy. */
export function supportsNativeVibrancy(): boolean {
  const p = detectPlatform();
  return p === "windows" || p === "macos";
}
