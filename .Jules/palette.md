## 2025-04-11 - Descriptive Labels for Selection Buttons
**Learning:** When rendering lists of selection buttons (e.g., color pickers), a generic `aria-label` (like "Farbe") on all items prevents screen reader users from distinguishing between the options.
**Action:** Always dynamically set `aria-label` and `title` to the specific item name (e.g., "Blau", "Grün") to ensure unique identification for accessibility.
