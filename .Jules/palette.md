## 2024-05-19 - Inconsistent Range Slider Feedback
**Learning:** Found that custom `type="range"` sliders in settings panels often lacked explicit visual value feedback (some had `.settings-panel__hint`, others didn't). This degrades UX because users cannot precisely tell what value they are setting, especially for non-standard scales like animation speed multipliers.
**Action:** When adding or reviewing range inputs in this application's settings UI, always pair them with a `<span className="settings-panel__hint">` to display the exact current value to the user.
