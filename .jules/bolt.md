
## 2024-03-28 - Optimizing O(n) Re-renders in Drag/Drop Lists
**Learning:** In a list component tracking high-frequency states like dragging and active positions (e.g., `dragPositions`), passing state-dependent inline objects or non-memoized callbacks directly to child items triggers O(n) re-renders, breaking memoization.
**Action:** Always extract dynamic inline styles into primitive component properties and use the "latest-value ref pattern" (`useRef` + `useEffect` update) to stabilize callbacks like `onDelete` or `onReorderMouseDown` inside parent components and custom hooks, allowing child component memoization to work as intended.
