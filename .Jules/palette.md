## 2024-04-13 - Improve color picker accessibility in ItemGroup
**Learning:** When rendering lists of selection buttons (e.g., color pickers), avoid using a generic `aria-label` for all items (like 'Farbe'). Screen reader users cannot distinguish between the options.
**Action:** Dynamically set the `aria-label` and `title` to the specific item name (e.g., 'Farbe: Blau', 'Farbe: Grün') so they are uniquely identifiable.
