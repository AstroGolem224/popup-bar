/**
 * ShelfBar — Main popup bar container.
 *
 * Top-level visual component with glassmorphism background.
 * Contains the ShelfGrid for rendering items.
 */
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
      {items.length === 0 ? (
        <div className="shelf-bar__empty">
          <p className="shelf-bar__empty-text">
            Drop files, folders, or apps here
          </p>
        </div>
      ) : (
        <ShelfGrid items={items} groups={groups} />
      )}
    </div>
  );
}
