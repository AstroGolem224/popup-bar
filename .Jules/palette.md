## 2025-03-25 - Range Slider UX
**Learning:** Range sliders without explicit value displays cause user uncertainty, especially for decimal settings like animation speed.
**Action:** Always pair `<input type="range">` with a `<span className="settings-panel__hint">` to display the exact current value (using `.toFixed()` for consistency) so users know exactly what they have selected.
