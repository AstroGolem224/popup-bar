## 2024-05-18 - Drag and Drop Re-renders
**Learning:** Frequent state updates for positioning coordinates during a local drag-and-drop interaction cause high-frequency unoptimized re-renders across the entire shelf grid. `ShelfItem` components previously re-rendered every time any single item was moved.
**Action:** When working with local positioning state (like `useItemReorder`), always enforce strict memoization boundaries via `React.memo` on child nodes (like `ShelfItem`) and stabilize inline functions in their parents using `useCallback` to prevent cascading renders.
## 2024-05-18 - Drag and Drop Re-renders (Updated)
**Learning:** React elements passed an inline style object as a prop (e.g., `style={{ position: "absolute", left: x }}`) will inherently fail `React.memo`'s shallow equality check, causing them to re-render on every cycle, even if the underlying layout values haven't changed.
**Action:** When working with local positioning state, always pass primitive variables (like `positionX={x}` and `positionY={y}`) to memoized child components, and construct the inline style object *inside* the child component's render function.
