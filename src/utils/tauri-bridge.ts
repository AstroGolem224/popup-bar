import { invoke } from "@tauri-apps/api/core";
import type { ShelfItem } from "../types/shelf";
import type { Settings } from "../types/settings";

/** Typed wrappers around Tauri invoke commands. */

export async function getShelfItems(): Promise<ShelfItem[]> {
  return invoke<ShelfItem[]>("get_shelf_items");
}

export async function addShelfItem(
  path: string,
  itemType: string,
): Promise<ShelfItem> {
  return invoke<ShelfItem>("add_shelf_item", { path, itemType });
}

export async function removeShelfItem(id: string): Promise<void> {
  return invoke<void>("remove_shelf_item", { id });
}

export async function updateShelfItem(item: ShelfItem): Promise<ShelfItem> {
  return invoke<ShelfItem>("update_shelf_item", { item });
}

export async function getSettings(): Promise<Settings> {
  return invoke<Settings>("get_settings");
}

export async function updateSettings(settings: Settings): Promise<Settings> {
  return invoke<Settings>("update_settings", { settings });
}

export async function showWindow(): Promise<void> {
  return invoke<void>("show_window");
}

export async function hideWindow(): Promise<void> {
  return invoke<void>("hide_window");
}
