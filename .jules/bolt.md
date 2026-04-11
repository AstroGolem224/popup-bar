## 2024-05-18 - Drag and Drop Re-renders
**Learning:** Frequent state updates for positioning coordinates during a local drag-and-drop interaction cause high-frequency unoptimized re-renders across the entire shelf grid. `ShelfItem` components previously re-rendered every time any single item was moved.
**Action:** When working with local positioning state (like `useItemReorder`), always enforce strict memoization boundaries via `React.memo` on child nodes (like `ShelfItem`) and stabilize inline functions in their parents using `useCallback` to prevent cascading renders.

## 2024-10-24 - Database Optimization with QueryBuilder
**Learning:** Issuing `N` individual `UPDATE` statements inside a loop (even within a transaction) for operations like batch-reordering causes an N+1 query pattern that is unnecessarily slow and hogs the connection pool.
**Action:** Use `sqlx::QueryBuilder` to generate a single `UPDATE` query utilizing a `CASE` statement (e.g. `UPDATE shelf_items SET position_x = CASE id WHEN ? THEN ? ... ELSE position_x END ... WHERE id IN (?, ...)`). This consolidates the updates into one execution roundtrip and dramatically improves throughput.
