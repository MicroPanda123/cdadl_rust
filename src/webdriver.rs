use std::collections::HashSet;
use thirtyfour::prelude::*;

pub struct VideoLink {
    pub name: String,
    pub url: String
}

pub async fn webtest(url: HashSet<String>) -> WebDriverResult<Vec<VideoLink>> {
    let mut res: Vec<VideoLink> = vec![];

    let mut caps = DesiredCapabilities::firefox();
    caps.set_headless().unwrap();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    for link in url {
        driver.goto(link).await?;

        let videoplayer = driver.find(By::ClassName("pb-video-player")).await?;
        let name = driver.find(By::Id("naglowek")).await?;
        let url = videoplayer.attr("src").await?;

        res.push(VideoLink { name: name.text().await?, url: url.unwrap() });
    }
    driver.quit().await?;

    return Ok(res);
}