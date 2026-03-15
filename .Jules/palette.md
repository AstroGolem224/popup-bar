## 2024-03-15 - Interactive Custom Elements Require ARIA Roles
**Learning:** Custom interactive elements (like drag-and-drop items) that handle keyboard events (`Enter`, `Space`) and clicks must have a proper ARIA role (e.g., `role="button"`) to be announced correctly by screen readers.
**Action:** Always add `role="button"` to `<div>` or `<span>` elements that are meant to be interacted with like buttons.
