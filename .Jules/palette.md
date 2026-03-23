## 2024-03-23 - Added value indicators for Settings Panel range sliders
**Learning:** Range sliders alone without adjacent specific numerical values create ambiguity for users needing exact adjustments, particularly for pixel or multiplier-based settings.
**Action:** Consistently pair `<input type="range">` elements with a `<span className="settings-panel__hint">` wrapper displaying the numerical state and unit, utilizing safe nullish fallbacks (e.g. `??`) and specific format constraints like `.toFixed()` to prevent render crashes.
