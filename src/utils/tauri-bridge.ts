import { invoke } from "@tauri-apps/api/core";
import type { ItemGroup, ItemType, ShelfItem } from "../types/shelf";
import type { Settings } from "../types/settings";

/** Typed wrappers around Tauri invoke commands. */

export async function getShelfItems(): Promise<ShelfItem[]> {
  return invoke<ShelfItem[]>("get_shelf_items");
}

export async function addShelfItem(
  path: string,
  itemType: ItemType,
): Promise<ShelfItem> {
  return invoke<ShelfItem>("add_shelf_item", { path, itemType });
}

export async function removeShelfItem(id: string): Promise<void> {
  return invoke<void>("remove_shelf_item", { id });
}

export async function updateShelfItem(item: ShelfItem): Promise<ShelfItem> {
  return invoke<ShelfItem>("update_shelf_item", { item });
}

export async function addDroppedPaths(paths: string[]): Promise<ShelfItem[]> {
  return invoke<ShelfItem[]>("add_dropped_paths", { paths });
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
