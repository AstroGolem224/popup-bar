import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import { listen as tauriListen, type UnlistenFn, type EventCallback } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow as tauriGetCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { getCurrentWindow as tauriGetCurrentWindow } from "@tauri-apps/api/window";
import type { ItemGroup, ItemType, ShelfItem } from "../types/shelf";
import type { Settings, SkinInfo } from "../types/settings";

/** Helper to detect if we are running inside the Tauri webview */
const isTauri =
  typeof window !== "undefined" &&
  (!!(window as any).__TAURI_INTERNALS__ || 
   !!(window as any).__TAURI__ ||
   !!(window as any).rpc);

/** Safe invoke wrapper that doesn't crash in non-Tauri environments */
async function invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauri || typeof tauriInvoke !== "function") {
    console.warn(`[tauri-bridge] invoke("${command}") mocked (non-Tauri environment)`);
    return getMockValue<T>(command);
  }

  try {
    return await tauriInvoke<T>(command, args);
  } catch (err) {
    if (err instanceof Error && err.message.includes("__TAURI_INTERNALS__")) {
      console.warn(`[tauri-bridge] invoke("${command}") failed due to missing internals`);
      return getMockValue<T>(command);
    }
    throw err;
  }
}

/** Safe getCurrentWebviewWindow wrapper */
export function getCurrentWebviewWindow() {
  if (!isTauri) {
    return {
      label: "main",
      listen: async (event: string, _callback: EventCallback<any>): Promise<UnlistenFn> => {
        console.warn(`[tauri-bridge] listen("${event}") mocked (non-Tauri)`);
        return () => {};
      },
      emit: async (event: string, _payload?: any) => {
        console.warn(`[tauri-bridge] emit("${event}") mocked (non-Tauri)`);
      },
    } as any;
  }
  try {
    return tauriGetCurrentWebviewWindow();
  } catch (e) {
    console.warn("[tauri-bridge] getCurrentWebviewWindow failed, falling back to mock", e);
    return { label: "main" } as any;
  }
}

/** Safe getCurrentWindow wrapper */
export function getCurrentWindow() {
  if (!isTauri) {
    return {
      label: "main",
      listen: async (event: string, _callback: EventCallback<any>): Promise<UnlistenFn> => {
        console.warn(`[tauri-bridge] window.listen("${event}") mocked (non-Tauri)`);
        return () => {};
      },
    } as any;
  }
  try {
    return tauriGetCurrentWindow();
  } catch (e) {
    console.warn("[tauri-bridge] getCurrentWindow failed, falling back to mock", e);
    return { label: "main" } as any;
  }
}

/** Safe listen wrapper */
export async function listen<T>(
  event: string,
  handler: EventCallback<T>,
): Promise<UnlistenFn> {
  if (!isTauri || typeof tauriListen !== "function") {
    console.warn(`[tauri-bridge] global listen("${event}") mocked (non-Tauri)`);
    return () => {};
  }
  return tauriListen(event, handler);
}

function getMockValue<T>(command: string): T {
  console.log(`[tauri-bridge] returning mock for: ${command}`);
  if (command === "get_shelf_items") return [] as unknown as T;
  if (command === "get_item_groups") return [] as unknown as T;
  if (command === "get_settings") return {
    hotzoneSize: 5,
    animationSpeed: 1.0,
    blurIntensity: 20,
    tintColor: "rgba(255, 255, 255, 0.1)",
    theme: "system",
    autostart: false,
    globalShortcut: "CommandOrControl+Shift+Space",
    multiMonitor: false,
    monitorStrategy: "primary",
    barWidthPx: 480,
    barHeightPx: 72,
    activeSkin: null,
    alignment: "centered"
  } as unknown as T;
  if (command === "list_skins") return [] as unknown as T;
  if (command === "get_platform_info") return { os: "windows", arch: "x86_64", version: "10" } as unknown as T;
  return undefined as unknown as T;
}

/** Typed wrappers around Tauri invoke commands. */

export async function getShelfItems(container?: string): Promise<ShelfItem[]> {
  return invoke<ShelfItem[]>("get_shelf_items", { container: container ?? null });
}

export async function addShelfItem(
  path: string,
  itemType: ItemType,
  container?: string,
): Promise<ShelfItem> {
  return invoke<ShelfItem>("add_shelf_item", { path, itemType, container: container ?? null });
}

export async function removeShelfItem(id: string): Promise<void> {
  return invoke<void>("remove_shelf_item", { id });
}

export async function updateShelfItem(item: ShelfItem): Promise<ShelfItem> {
  return invoke<ShelfItem>("update_shelf_item", { item });
}

export async function addDroppedPaths(paths: string[], container?: string): Promise<ShelfItem[]> {
  return invoke<ShelfItem[]>("add_dropped_paths", { paths, container: container ?? null });
}

export async function reorderShelfItems(orderedIds: string[]): Promise<void> {
  return invoke<void>("reorder_shelf_items", { orderedIds });
}

export async function getItemGroups(): Promise<ItemGroup[]> {
  return invoke<ItemGroup[]>("get_item_groups");
}

export async function createItemGroup(
  name: string,
  color?: string,
): Promise<ItemGroup> {
  return invoke<ItemGroup>("create_item_group", {
    name,
    color: color ?? null,
  });
}

export async function updateItemGroup(group: ItemGroup): Promise<ItemGroup> {
  return invoke<ItemGroup>("update_item_group", { group });
}

export async function deleteItemGroup(id: string): Promise<void> {
  return invoke<void>("delete_item_group", { id });
}

export async function getSettings(): Promise<Settings> {
  return invoke<Settings>("get_settings");
}

export async function updateSettings(settings: Settings): Promise<Settings> {
  return invoke<Settings>("update_settings", { settings });
}

export async function setLaunchAtLogin(enabled: boolean): Promise<void> {
  return invoke<void>("set_launch_at_login", { enabled });
}

export async function showWindow(): Promise<number | null> {
  return invoke<number | null>("show_window");
}

export async function completeShowWindow(token: number): Promise<boolean> {
  return invoke<boolean>("complete_show_window", { token });
}

export async function hideWindow(): Promise<number | null> {
  return invoke<number | null>("hide_window");
}

export async function completeHideWindow(token: number): Promise<boolean> {
  return invoke<boolean>("complete_hide_window", { token });
}

export async function setSettingsExpanded(expanded: boolean): Promise<void> {
  return invoke<void>("set_settings_expanded", { expanded });
}

export async function openShelfItemViaLauncher(
  itemType: ItemType,
  path: string,
): Promise<void> {
  await invoke<void>("open_shelf_item", { itemType, path });
}

export async function exitApp(): Promise<void> {
  await invoke<void>("exit_app");
}

/** Returns data URL for a cached icon (avoids asset protocol scope). */
export async function getIconDataUrl(iconPath: string): Promise<string | null> {
  const result = await invoke<[string, string]>("get_icon_data", {
    iconPath,
  }).catch(() => null);
  if (!result || !Array.isArray(result) || result.length < 2) return null;
  const [base64, mime] = result;
  return base64 && mime ? `data:${mime};base64,${base64}` : null;
}

export interface PlatformInfo {
  os: string;
  arch: string;
  version: string;
}

export async function getPlatformInfo(): Promise<PlatformInfo> {
  return invoke<PlatformInfo>("get_platform_info");
}

export async function listSkins(): Promise<SkinInfo[]> {
  return invoke<SkinInfo[]>("list_skins");
}

export async function importSkin(sourcePath: string): Promise<SkinInfo> {
  return invoke<SkinInfo>("import_skin", { sourcePath });
}

export async function importSkinBytes(
  filenameStem: string,
  ext: string,
  bytes: number[],
): Promise<SkinInfo> {
  return invoke<SkinInfo>("import_skin_bytes", { filenameStem, ext, bytes });
}

export async function setActiveSkin(filename: string | null): Promise<Settings> {
  return invoke<Settings>("set_active_skin", { filename });
}

export async function deleteSkin(filename: string): Promise<Settings> {
  return invoke<Settings>("delete_skin", { filename });
}

/** Returns a base64 data URL for a skin image file. */
export async function getSkinDataUrl(filename: string): Promise<string | null> {
  return invoke<string>("get_skin_data", { filename }).catch((err) => {
    console.error("[skin] get_skin_data failed for", filename, err);
    return null;
  });
}
