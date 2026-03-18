/**
 * Hook: Shelf items and groups.
 *
 * Provides the current list of shelf items and item groups
 * from the Zustand store, with methods to modify them.
 *
 * Uses Zustand as local cache and synchronizes with Tauri commands.
 */
import { useEffect, useCallback } from "react";
import type { ShelfItem, ItemGroup } from "../types/shelf";
import {
  addShelfItem,
  createItemGroup,
  deleteItemGroup,
  getItemGroups,
  getShelfItems,
  removeShelfItem,
  updateItemGroup,
  updateShelfItem,
} from "../utils/tauri-bridge";
import { useShelfStore } from "../stores/shelfStore";
import type { ItemType } from "../types/shelf";

interface UseShelfItemsReturn {
  /** All shelf items. */
  items: ShelfItem[];
  /** All item groups. */
  groups: ItemGroup[];
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

  useEffect(() => {
    let isMounted = true;
    void (async () => {
      try {
        const [allItems, allGroups] = await Promise.all([
          getShelfItems(),
          getItemGroups(),
        ]);
        if (isMounted) {
          setItems(allItems);
          setGroups(allGroups);
        }
      } catch (error) {
        console.warn("initial shelf load failed", error);
        setError("items und gruppen konnten nicht geladen werden");
      }
    })();

    return () => {
      isMounted = false;
    };
  }, [setError, setItems]);

  return {
    items,
    groups,
    addItem: useCallback(async (path: string, itemType: string) => {
      try {
        const item = await addShelfItem(path, itemType as ItemType);
        storeAddItem(item);
      } catch (error) {
        console.warn("add_shelf_item failed", error);
        setError("item konnte nicht angelegt werden");
      }
    }, [storeAddItem, setError]),
    removeItem: useCallback(async (id: string) => {
      try {
        await removeShelfItem(id);
        storeRemoveItem(id);
      } catch (error) {
        console.warn("remove_shelf_item failed", error);
        setError("item konnte nicht geloescht werden");
      }
    }, [storeRemoveItem, setError]),
    updateItem: useCallback(async (item: ShelfItem) => {
      try {
        const updated = await updateShelfItem(item);
        storeUpdateItem(updated);
      } catch (error) {
        console.warn("update_shelf_item failed", error);
        setError("item konnte nicht gespeichert werden");
      }
    }, [storeUpdateItem, setError]),
    addGroup: useCallback(async (name: string, color?: string) => {
      try {
        const group = await createItemGroup(name, color);
        useShelfStore.setState((state) => ({
          groups: [...state.groups, group],
        }));
      } catch (error) {
        console.warn("create_item_group failed", error);
        setError("gruppe konnte nicht angelegt werden");
      }
    }, [setError]),
    updateGroup: useCallback(async (group: ItemGroup) => {
      try {
        const updated = await updateItemGroup(group);
        useShelfStore.setState((state) => ({
          groups: state.groups.map((g) => (g.id === updated.id ? updated : g)),
        }));
      } catch (error) {
        console.warn("update_item_group failed", error);
        setError("gruppe konnte nicht gespeichert werden");
      }
    }, [setError]),
    removeGroup: useCallback(async (id: string) => {
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
    }, [setError]),
  };
}
