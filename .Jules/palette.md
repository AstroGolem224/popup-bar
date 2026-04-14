## 2025-02-27 - Dynamic ARIA Labels for Color Pickers
**Learning:** When rendering lists of selection buttons (e.g., color pickers for ItemGroups), using a generic `aria-label` like "Farbe" or "Farbe ändern" for all items prevents screen reader users from distinguishing between the options.
**Action:** Always map generic selections to an array of objects that include descriptive labels (e.g., "Blau", "Grün") and dynamically apply them to `aria-label` and `title` attributes.
