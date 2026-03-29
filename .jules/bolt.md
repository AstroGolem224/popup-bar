## 2024-05-18 - Drag and Drop Re-renders
**Learning:** Frequent state updates for positioning coordinates during a local drag-and-drop interaction cause high-frequency unoptimized re-renders across the entire shelf grid. `ShelfItem` components previously re-rendered every time any single item was moved.
**Action:** When working with local positioning state (like `useItemReorder`), always enforce strict memoization boundaries via `React.memo` on child nodes (like `ShelfItem`) and stabilize inline functions in their parents using `useCallback` to prevent cascading renders.
