import { ShelfItem as ShelfItemComponent } from "../ShelfItem";
import { ItemGroup as ItemGroupComponent } from "../ItemGroup";
import type { ShelfItem, ItemGroup } from "../../types/shelf";
import "./ShelfGrid.css";

export interface ShelfGridProps {
  items: ShelfItem[];
  groups: ItemGroup[];
}

export function ShelfGrid({ items, groups }: ShelfGridProps) {
  return (
    <div className="shelf-grid">
      {groups.map((group) => (
        <ItemGroupComponent
          key={group.id}
          group={group}
          items={items.filter((item) => item.groupId === group.id)}
        />
      ))}
      {items
        .filter((item) => !item.groupId)
        .map((item) => (
          <ShelfItemComponent key={item.id} item={item} />
        ))}
    </div>
  );
}
