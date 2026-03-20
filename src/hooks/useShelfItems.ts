/**
 * Hook: Shelf items and groups.
 *
 * Provides the current list of shelf items and item groups
 * from the Zustand store, with methods to modify them.
 *
 * Uses Zustand as local cache and synchronizes with Tauri commands.
 */
import { startTransition, useCallback, useEffect, useState } from "react";
import type { ShelfItem, ItemGroup } from "../types/shelf";
import {
  addShelfItem,
  createItemGroup,
  deleteItemGroup,
  getItemGroups,
  getShelfItems,
  removeShelfItem,
  reorderShelfItems,
  updateItemGroup,
  updateShelfItem,
  listen,
  getCurrentWebviewWindow,
} from "../utils/tauri-bridge";
import { useShelfStore } from "../stores/shelfStore";
import type { ItemType } from "../types/shelf";

interface UseShelfItemsReturn {
  /** All shelf items. */
  items: ShelfItem[];
  /** All item groups. */
  groups: ItemGroup[];
  /** Current window label (container). */
  container: string;
  /** Add an item. */
  addItem: (path: string, itemType: string) => Promise<void>;
  /** Remove an item by ID. */
  removeItem: (id: string) => Promise<void>;
  /** Update an item (e.g. set groupId). */
  updateItem: (item: ShelfItem) => Promise<void>;
  /** Create a new item group. */
  addGroup: (name: string, color?: string) => Promise<void>;
  /** Update an item group (name, color). */
  updateGroup: (group: ItemGroup) => Promise<void>;
  /** Delete an item group by ID. */
  removeGroup: (id: string) => Promise<void>;
  /** Move an item to a new index. */
  moveItem: (id: string, newIndex: number) => Promise<void>;
}

export function useShelfItems(): UseShelfItemsReturn {
  const items = useShelfStore((state) => state.items);
  const groups = useShelfStore((state) => state.groups);
  const storeAddItem = useShelfStore((state) => state.addItem);
  const storeRemoveItem = useShelfStore((state) => state.removeItem);
  const storeUpdateItem = useShelfStore((state) => state.updateItem);
  const setItems = useShelfStore((state) => state.setItems);
  const setGroups = useShelfStore((state) => state.setGroups);
  const setError = useShelfStore((state) => state.setError);

  // Determine the current window label for container filtering
  const [container] = useState(() => {
    try {
      return getCurrentWebviewWindow().label;
    } catch {
      return "main";
    }
  });

  const loadShelfData = useCallback(
    async (includeGroups: boolean) => {
      try {
        const [allItems, allGroups] = await Promise.all([
          getShelfItems(container),
          includeGroups ? getItemGroups() : Promise.resolve(null),
        ]);

        startTransition(() => {
          setItems(allItems);
          if (allGroups) {
            setGroups(allGroups);
          }
        });
      } catch (error) {
        throw error;
      }
    },
    [container, setGroups, setItems],
  );

  useEffect(() => {
    let isMounted = true;
    void (async () => {
      try {
        const [allItems, allGroups] = await Promise.all([
          getShelfItems(container),
          getItemGroups(),
        ]);
        if (isMounted) {
          startTransition(() => {
            setItems(allItems);
            setGroups(allGroups);
          });
        }
      } catch (error) {
        console.warn("initial shelf load failed", error);
        setError("items und gruppen konnten nicht geladen werden");
      }
    })();

    return () => {
      isMounted = false;
    };
  }, [container, setError, setGroups, setItems]);

  // Listen for shelf_items_changed events emitted by Rust (e.g. sidebar drag-drop)
  // and reload items from backend to keep all windows in sync.
  useEffect(() => {
    let unlisten: (() => void) | undefined;
    listen("shelf_items_changed", async () => {
      try {
        await loadShelfData(false);
      } catch (error) {
        console.warn("shelf reload after change failed", error);
      }
    }).then((fn) => {
      unlisten = fn;
    });
    return () => {
      unlisten?.();
    };
  }, [loadShelfData]);

  return {
    items,
    groups,
    container,
    addItem: async (path: string, itemType: string) => {
      try {
        const item = await addShelfItem(path, itemType as ItemType, container);
        storeAddItem(item);
      } catch (error) {
        console.warn("add_shelf_item failed", error);
        setError("item konnte nicht angelegt werden");
      }
    },
    removeItem: async (id: string) => {
      try {
        await removeShelfItem(id);
        storeRemoveItem(id);
      } catch (error) {
        console.warn("remove_shelf_item failed", error);
        setError("item konnte nicht geloescht werden");
      }
    },
    updateItem: async (item: ShelfItem) => {
      try {
        const updated = await updateShelfItem(item);
        storeUpdateItem(updated);
      } catch (error) {
        console.warn("update_shelf_item failed", error);
        setError("item konnte nicht gespeichert werden");
      }
    },
    addGroup: async (name: string, color?: string) => {
      try {
        const group = await createItemGroup(name, color);
        useShelfStore.setState((state) => ({
          groups: [...state.groups, group],
        }));
      } catch (error) {
        console.warn("create_item_group failed", error);
        setError("gruppe konnte nicht angelegt werden");
      }
    },
    updateGroup: async (group: ItemGroup) => {
      try {
        const updated = await updateItemGroup(group);
        useShelfStore.setState((state) => ({
          groups: state.groups.map((g) => (g.id === updated.id ? updated : g)),
        }));
      } catch (error) {
        console.warn("update_item_group failed", error);
        setError("gruppe konnte nicht gespeichert werden");
      }
    },
    removeGroup: async (id: string) => {
      try {
        await deleteItemGroup(id);
        useShelfStore.setState((state) => ({
          groups: state.groups.filter((g) => g.id !== id),
          items: state.items.map((item) =>
            item.groupId === id ? { ...item, groupId: undefined } : item,
          ),
        }));
      } catch (error) {
        console.warn("delete_item_group failed", error);
        setError("gruppe konnte nicht geloescht werden");
      }
    },
    moveItem: async (id: string, newIndex: number) => {
      const currentItems = useShelfStore.getState().items;
      const oldIndex = currentItems.findIndex((i) => i.id === id);
      if (oldIndex === -1 || oldIndex === newIndex) return;

      const newItems = [...currentItems];
      const [movedItem] = newItems.splice(oldIndex, 1);
      if (!movedItem) return;
      newItems.splice(newIndex, 0, movedItem);

      useShelfStore.getState().setItems(newItems);
      try {
        await reorderShelfItems(newItems.map((i) => i.id));
      } catch (error) {
        console.warn("move_item persist failed", error);
        setError("reihenfolge konnte nicht gespeichert werden");
        useShelfStore.getState().setItems(currentItems);
      }
    },
  };
}
