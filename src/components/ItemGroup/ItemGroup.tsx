import { ShelfItem as ShelfItemComponent } from "../ShelfItem";
import type { ShelfItem, ItemGroup as ItemGroupType } from "../../types/shelf";
import "./ItemGroup.css";

export interface ItemGroupProps {
  group: ItemGroupType;
  items: ShelfItem[];
}

export function ItemGroup({ group, items }: ItemGroupProps) {
  return (
    <div
      className="item-group"
      style={{ borderColor: group.color ?? "rgba(255,255,255,0.2)" }}
    >
      <span className="item-group__label">{group.name}</span>
      <div className="item-group__items">
        {items.map((item) => (
          <ShelfItemComponent key={item.id} item={item} />
        ))}
      </div>
    </div>
  );
}
