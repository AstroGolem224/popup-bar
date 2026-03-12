import type { ShelfItem, ItemGroup } from "./shelf";
import type { Settings } from "./settings";

/** Tauri event name constants */
export const EVENTS = {
  HOTZONE_ENTER: "hotzone:enter",
  HOTZONE_LEAVE: "hotzone:leave",
  SHELF_ITEM_ADDED: "shelf:item-added",
  SHELF_ITEM_REMOVED: "shelf:item-removed",
  SHELF_ITEM_UPDATED: "shelf:item-updated",
  SHELF_GROUP_CREATED: "shelf:group-created",
  SHELF_GROUP_DELETED: "shelf:group-deleted",
  SETTINGS_CHANGED: "settings:changed",
  WINDOW_SHOW: "window:show",
  WINDOW_HIDE: "window:hide",
  DND_DROP: "dnd:drop",
} as const;

export type EventName = (typeof EVENTS)[keyof typeof EVENTS];

/** Event payload types */
export interface EventPayloads {
  [EVENTS.HOTZONE_ENTER]: { x: number; y: number };
  [EVENTS.HOTZONE_LEAVE]: undefined;
  [EVENTS.SHELF_ITEM_ADDED]: ShelfItem;
  [EVENTS.SHELF_ITEM_REMOVED]: { id: string };
  [EVENTS.SHELF_ITEM_UPDATED]: ShelfItem;
  [EVENTS.SHELF_GROUP_CREATED]: ItemGroup;
  [EVENTS.SHELF_GROUP_DELETED]: { id: string };
  [EVENTS.SETTINGS_CHANGED]: Settings;
  [EVENTS.WINDOW_SHOW]: undefined;
  [EVENTS.WINDOW_HIDE]: undefined;
  [EVENTS.DND_DROP]: { paths: string[]; x: number; y: number };
}
