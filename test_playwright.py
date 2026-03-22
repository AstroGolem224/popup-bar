import subprocess
from playwright.sync_api import sync_playwright

def verify_feature():
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        context = browser.new_context(record_video_dir="/home/jules/verification/video")
        page = context.new_page()

        try:
            page.goto("http://localhost:4173")
            page.wait_for_timeout(1000)

            # Check if __TAURI_INTERNALS__ mock issue occurs
            # This is a known issue documented in memory:
            # "When attempting Playwright visual verification for the Vite frontend, injecting a basic `window.__TAURI_INTERNALS__` mock often causes app initialization crashes... It is acceptable to skip the screenshot and proceed with code review if this technical blocker occurs."

            page.screenshot(path="/home/jules/verification/verification.png")
            print("Screenshot captured successfully.")

        except Exception as e:
            print(f"Error occurred: {e}")
        finally:
            context.close()
            browser.close()

if __name__ == "__main__":
    import os
    os.makedirs("/home/jules/verification/video", exist_ok=True)
    verify_feature()
