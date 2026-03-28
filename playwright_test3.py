from playwright.sync_api import sync_playwright
import os

def run_cuj(page):
    page.goto("http://localhost:4173")
    page.wait_for_timeout(2000)

    # Use evaluate_handle to mock before Tauri initialization
    page.add_init_script("""
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
                if (cmd === 'get_platform_info') return { os: 'linux', version: '1.0' };
                return null;
            }
        };
        window.__TAURI_IPC__ = async (cmd) => {
            console.log('IPC', cmd);
        };
    """)
    page.reload()
    page.wait_for_timeout(2000)

    # Hotzone state triggers visibility on mouse enter at the top edge usually, let's mock it
    # We will just evaluate a dispatch event for tauri listener 'hotzone:enter'
    page.evaluate("""
        window.dispatchEvent(new CustomEvent('tauri://event', {
            detail: {
                event: 'hotzone:enter',
                payload: null
            }
        }));
    """)
    page.wait_for_timeout(2000)

    # Also make sure the body is loaded
    page.screenshot(path="/home/jules/verification/screenshots/verification3.png")

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
