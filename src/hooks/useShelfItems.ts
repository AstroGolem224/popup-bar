import { useShelfStore } from "../stores/shelfStore";

/** Provides access to shelf items and groups with CRUD operations. */
export function useShelfItems() {
  const items = useShelfStore((s) => s.items);
  const groups = useShelfStore((s) => s.groups);
  const addItem = useShelfStore((s) => s.addItem);
  const removeItem = useShelfStore((s) => s.removeItem);
  const updateItem = useShelfStore((s) => s.updateItem);

  return { items, groups, addItem, removeItem, updateItem };
}
