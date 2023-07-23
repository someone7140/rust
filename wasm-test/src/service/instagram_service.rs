use std::error::Error;
use std::fs;

use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;

pub fn instagram_follow_check() -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    // インスタグラムのログイン画面
    tab.navigate_to("https://www.instagram.com/accounts/login/")?;
    tab.wait_until_navigated()?;

    let jpeg_data =
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)?;
    fs::write("screenshot.jpg", &jpeg_data)?;
    Ok(())
}
