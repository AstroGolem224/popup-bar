/**
 * Hook: Shelf items and groups.
 *
 * Provides the current list of shelf items and item groups
 * from the Zustand store, with methods to modify them.
 *
 * Phase 0: Returns empty arrays.
 * Phase 2: Wired to Tauri commands + Zustand store.
 */
import type { ShelfItem, ItemGroup } from "../types/shelf";

interface UseShelfItemsReturn {
  /** All shelf items. */
  items: ShelfItem[];
  /** All item groups. */
  groups: ItemGroup[];
  /** Add an item (Phase 2). */
  addItem: (path: string, itemType: string) => Promise<void>;
  /** Remove an item by ID (Phase 2). */
  removeItem: (id: string) => Promise<void>;
}

export function useShelfItems(): UseShelfItemsReturn {
  return {
    items: [],
    groups: [],
    addItem: async (_path: string, _itemType: string) => {
      console.warn("addItem: not implemented (Phase 2)");
    },
    removeItem: async (_id: string) => {
      console.warn("removeItem: not implemented (Phase 2)");
    },
  };
}
