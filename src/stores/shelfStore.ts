import { create } from "zustand";
import type { ShelfItem, ItemGroup } from "../types/shelf";

interface ShelfState {
  items: ShelfItem[];
  groups: ItemGroup[];
  errorMessage: string | null;
  addItem: (item: ShelfItem) => void;
  addItems: (items: ShelfItem[]) => void;
  removeItem: (id: string) => void;
  updateItem: (item: ShelfItem) => void;
  reorderItems: (orderedIds: string[]) => void;
  setItems: (items: ShelfItem[]) => void;
  addGroup: (group: ItemGroup) => void;
  removeGroup: (id: string) => void;
  setGroups: (groups: ItemGroup[]) => void;
  setError: (message: string) => void;
  clearError: () => void;
}

export const useShelfStore = create<ShelfState>((set) => ({
  items: [],
  groups: [],
  errorMessage: null,

  addItem: (item) =>
    set((state) => ({ items: [...state.items, item] })),

  addItems: (items) =>
    set((state) => ({ items: [...state.items, ...items] })),

  removeItem: (id) =>
    set((state) => ({ items: state.items.filter((i) => i.id !== id) })),

  updateItem: (item) =>
    set((state) => ({
      items: state.items.map((i) => (i.id === item.id ? item : i)),
    })),

  reorderItems: (orderedIds) =>
    set((state) => {
      const itemMap = new Map(state.items.map((item) => [item.id, item]));
      const reordered = orderedIds
        .map((id) => itemMap.get(id))
        .filter((item): item is ShelfItem => Boolean(item));
      const remaining = state.items.filter((item) => !orderedIds.includes(item.id));
      return { items: [...reordered, ...remaining] };
    }),

  setItems: (items) => set({ items }),

  addGroup: (group) =>
    set((state) => ({ groups: [...state.groups, group] })),

  removeGroup: (id) =>
    set((state) => ({
      groups: state.groups.filter((g) => g.id !== id),
      items: state.items.map((i) =>
        i.groupId === id ? { ...i, groupId: undefined } : i,
      ),
    })),

  setGroups: (groups) => set({ groups }),

  setError: (message) => set({ errorMessage: message }),
  clearError: () => set({ errorMessage: null }),
}));
