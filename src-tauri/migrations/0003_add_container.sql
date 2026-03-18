-- Add container column to shelf_items to identify which bar (main/left/right) an item belongs to.
ALTER TABLE shelf_items ADD COLUMN container TEXT NOT NULL DEFAULT 'main';
CREATE INDEX IF NOT EXISTS idx_shelf_items_container ON shelf_items(container);
