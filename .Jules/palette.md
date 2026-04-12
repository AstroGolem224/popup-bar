## 2024-05-24 - Descriptive Labels for Color Pickers
**Learning:** Screen readers typically struggle with simple `button` elements used as color pickers when they rely solely on background color styling, unless uniquely labeled. Providing a generic `aria-label="Farbe"` causes ambiguity.
**Action:** Always provide specific label descriptions corresponding to the color (e.g., "Blau", "Grün") via `aria-label` or `title` so users can easily differentiate color options in selection grids.
