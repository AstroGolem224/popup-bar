import { create } from "zustand";
import type { ShelfItem, ItemGroup } from "../types/shelf";

interface ShelfState {
  items: ShelfItem[];
  groups: ItemGroup[];
  addItem: (item: ShelfItem) => void;
  removeItem: (id: string) => void;
  updateItem: (item: ShelfItem) => void;
  setItems: (items: ShelfItem[]) => void;
  addGroup: (group: ItemGroup) => void;
  removeGroup: (id: string) => void;
  setGroups: (groups: ItemGroup[]) => void;
}

export const useShelfStore = create<ShelfState>((set) => ({
  items: [],
  groups: [],

  addItem: (item) =>
    set((state) => ({ items: [...state.items, item] })),

  removeItem: (id) =>
    set((state) => ({ items: state.items.filter((i) => i.id !== id) })),

  updateItem: (item) =>
    set((state) => ({
      items: state.items.map((i) => (i.id === item.id ? item : i)),
    })),

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
}));
