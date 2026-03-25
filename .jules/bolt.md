## 2024-03-25 - React.memo for ShelfItem component
**Learning:** React list components should be wrapped in `React.memo` to prevent unnecessary re-renders. `ShelfItem` is a list item inside `ShelfGrid`, and it re-renders whenever `ShelfGrid` state changes (like during drag-and-drop actions that frequently update `dragPositions`).
**Action:** Wrap `ShelfItem` with `React.memo` and ensure the passed props are optimized for caching (primitives vs objects).
