from playwright.sync_api import sync_playwright

def run_cuj(page):
    page.goto("http://localhost:4173")
    page.wait_for_timeout(2000)

    # Inject mock for Tauri internally
    page.evaluate("""
        window.__TAURI_INTERNALS__ = {
            metadata: { currentWindow: { label: 'main' } },
            invoke: async (cmd, args) => {
                if (cmd === 'get_settings') return {
                    hotzoneSize: 10, barWidthPx: 480, barHeightPx: 72,
                    animationSpeed: 1, alignment: 'centered', monitorStrategy: 'primary',
                    activeSkin: null, autostart: false, globalShortcut: ''
                };
                if (cmd === 'list_skins') return [];
                if (cmd === 'get_shelf_items') return [
                    { id: '1', itemType: 'app', path: '/foo', displayName: 'App 1', position: { x: 10, y: 10 } },
                    { id: '2', itemType: 'folder', path: '/bar', displayName: 'Folder 2', position: { x: 100, y: 10 } }
                ];
                return null;
            }
        };
    """)
    page.reload()
    page.wait_for_timeout(2000)

    # Trigger hotzone manually since the mouse won't do it naturally in this headless environment without simulating it precisely
    page.evaluate("window.dispatchEvent(new CustomEvent('tauri://event', { detail: { event: 'hotzone:enter' } }));")
    page.wait_for_timeout(1000)

    # Take screenshot at the key moment
    page.screenshot(path="/home/jules/verification/screenshots/verification2.png")
    page.wait_for_timeout(1000)

if __name__ == "__main__":
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        context = browser.new_context(
            record_video_dir="/home/jules/verification/videos"
        )
        page = context.new_page()
        try:
            run_cuj(page)
        finally:
            context.close()
            browser.close()
