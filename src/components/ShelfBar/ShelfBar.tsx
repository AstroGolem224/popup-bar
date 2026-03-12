import { ShelfGrid } from "../ShelfGrid";
import { useShelfItems } from "../../hooks/useShelfItems";
import { useGlassmorphism } from "../../hooks/useGlassmorphism";
import "./ShelfBar.css";

export interface ShelfBarProps {
  className?: string;
}

export function ShelfBar({ className }: ShelfBarProps) {
  const { items, groups } = useShelfItems();
  const glassStyle = useGlassmorphism();

  return (
    <div className={`shelf-bar ${className ?? ""}`} style={glassStyle}>
      <ShelfGrid items={items} groups={groups} />
    </div>
  );
}
