import { ShelfItem as ShelfItemComponent } from "../ShelfItem";
import type { ShelfItem, ItemGroup as ItemGroupType } from "../../types/shelf";
import React, { useRef, useState } from "react";
import "./ItemGroup.css";

const GROUP_COLORS = [
  { value: "rgba(255,255,255,0.2)", label: "Weiß" },
  { value: "rgba(100,180,255,0.6)", label: "Blau" },
  { value: "rgba(120,255,120,0.5)", label: "Grün" },
  { value: "rgba(255,180,100,0.5)", label: "Orange" },
  { value: "rgba(255,120,200,0.5)", label: "Pink" },
];

export interface ItemGroupProps {
  group: ItemGroupType;
  items: ShelfItem[];
  onUpdateGroup?: (group: ItemGroupType) => void | Promise<void>;
  onDeleteGroup?: (id: string) => void | Promise<void>;
  /** Called when a shelf item is dropped on this group (assign item to group). */
  onDropItemOnGroup?: (groupId: string, itemId: string) => void | Promise<void>;
  /** Called when user clicks delete on an item in this group. */
  onDeleteItem?: (id: string) => void | Promise<void>;
}

export function ItemGroup({
  group,
  items,
  onUpdateGroup,
  onDeleteGroup,
  onDropItemOnGroup,
  onDeleteItem,
}: ItemGroupProps) {
  const [isEditingName, setIsEditingName] = useState(false);
  const [draftName, setDraftName] = useState(group.name);
  const [isDragOver, setIsDragOver] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  const commitName = () => {
    const trimmed = draftName.trim();
    if (trimmed && trimmed !== group.name && onUpdateGroup) {
      void onUpdateGroup({ ...group, name: trimmed });
    }
    setIsEditingName(false);
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = "move";
    if (onDropItemOnGroup) setIsDragOver(true);
  };

  const handleDragLeave = () => setIsDragOver(false);

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragOver(false);
    const itemId = e.dataTransfer.getData("application/x-popup-bar-item-id");
    if (itemId && onDropItemOnGroup) {
      void onDropItemOnGroup(group.id, itemId);
    }
  };

  return (
    <div
      className={`item-group ${isDragOver ? "item-group--drag-over" : ""}`}
      style={{ borderColor: group.color ?? "rgba(255,255,255,0.2)" }}
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
      onDrop={handleDrop}
    >
      <div className="item-group__header">
        {isEditingName ? (
          <input
            ref={inputRef}
            type="text"
            className="item-group__input"
            value={draftName}
            onChange={(e) => setDraftName(e.target.value)}
            onBlur={commitName}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                e.currentTarget.blur();
              }
              if (e.key === "Escape") {
                setDraftName(group.name);
                setIsEditingName(false);
              }
            }}
            autoFocus
          />
        ) : (
          <span
            className="item-group__label item-group__label--editable"
            title="Doppelklick zum Umbenennen"
            onDoubleClick={() => {
              setDraftName(group.name);
              setIsEditingName(true);
            }}
          >
            {group.name}
          </span>
        )}
        <div className="item-group__actions">
          {onUpdateGroup
            ? GROUP_COLORS.map(({ value, label }) => (
                <button
                  key={value}
                  type="button"
                  className="item-group__color"
                  style={{ backgroundColor: value }}
                  title={`Farbe auf ${label} ändern`}
                  aria-label={label}
                  onClick={() =>
                    void onUpdateGroup({ ...group, color: value })
                  }
                />
              ))
            : null}
          {onDeleteGroup ? (
            <button
              type="button"
              className="item-group__delete"
              onClick={() => onDeleteGroup(group.id)}
              aria-label="Gruppe löschen"
              title="Gruppe löschen"
            >
              ×
            </button>
          ) : null}
        </div>
      </div>
      <div className="item-group__items">
        {items.map((item) => (
          <ShelfItemComponent key={item.id} item={item} onDelete={onDeleteItem} />
        ))}
      </div>
    </div>
  );
}
