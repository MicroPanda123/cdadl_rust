use thirtyfour::prelude::*;

pub struct VideoLink {
    pub name: String,
    pub url: String
}

impl VideoLink {
    fn new() -> VideoLink {
        VideoLink {
            name: "".to_string(),
            url: "".to_string()
        }
    }
}

pub async fn init_driver() -> Result<WebDriver, WebDriverError> {
    let mut caps = DesiredCapabilities::firefox();
    caps.set_headless().unwrap();
    let driver = WebDriver::new("http://localhost:4444", caps);
    driver.await
}

pub async fn webtest(driver: WebDriver, url: &str) -> WebDriverResult<VideoLink> {
    let mut res = VideoLink::new();

    // Navigate to https://wikipedia.org.
    driver.goto(url).await?;

    let videoplayer = driver.find(By::ClassName("pb-video-player")).await?;
    let name = driver.find(By::Id("naglowek")).await?;
    let url = videoplayer.attr("src").await?;

    res.name = name.text().await?;
    res.url = url.unwrap();

    driver.quit().await?;

    return Ok(res);
}