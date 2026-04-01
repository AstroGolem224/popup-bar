## 2024-05-18 - Drag and Drop Re-renders
**Learning:** Frequent state updates for positioning coordinates during a local drag-and-drop interaction cause high-frequency unoptimized re-renders across the entire shelf grid. `ShelfItem` components previously re-rendered every time any single item was moved.
**Action:** When working with local positioning state (like `useItemReorder`), always enforce strict memoization boundaries via `React.memo` on child nodes (like `ShelfItem`) and stabilize inline functions in their parents using `useCallback` to prevent cascading renders.

## 2025-04-01 - Drag and Drop Re-renders II
**Learning:** React elements receiving inline object structures like `style={{ position: 'absolute', x, y }}` frequently fail `React.memo` bails because object references are recreated on every render cycle. Additionally, closure-heavy event handlers passing through a dynamic list also break memoization.
**Action:** When working with local positioning state (like `useItemReorder`), always pass coordinates as primitive props (`positionX`, `positionY`) to child items and reconstruct complex style objects internally. Additionally, use the "latest-value ref pattern" to decouple rapidly updating dependencies from event handlers.
