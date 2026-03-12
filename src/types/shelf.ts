export type ItemType = "file" | "folder" | "app" | "url";

export interface ShelfItem {
  id: string;
  itemType: ItemType;
  path: string;
  displayName: string;
  iconCacheKey: string;
  position: { x: number; y: number };
  groupId?: string;
  createdAt: string;
  lastUsed: string;
}

export interface ItemGroup {
  id: string;
  name: string;
  color?: string;
  position: { x: number; y: number };
}
