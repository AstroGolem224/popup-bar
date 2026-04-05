## 2024-05-18 - Drag and Drop Re-renders
**Learning:** Frequent state updates for positioning coordinates during a local drag-and-drop interaction cause high-frequency unoptimized re-renders across the entire shelf grid. `ShelfItem` components previously re-rendered every time any single item was moved.
**Action:** When working with local positioning state (like `useItemReorder`), always enforce strict memoization boundaries via `React.memo` on child nodes (like `ShelfItem`) and stabilize inline functions in their parents using `useCallback` to prevent cascading renders.

## 2025-02-20 - Prevent O(n) re-renders during drag-and-drop
**Learning:** React.memo on a list component fails if the parent passes dynamic inline style objects (e.g., `{left: '10px'}`) and unstable callbacks during high-frequency events like drag-and-drop.
**Action:** Always break down position objects into primitive props (e.g., `positionX`, `positionY`) and stabilize callbacks using the "latest-value ref pattern" to ensure the list items don't re-render unecessarily when dragging.
