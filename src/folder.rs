use std::collections::HashSet;
use reqwest;
use select::document::Document;
use select::predicate::Name;

async fn fetch_website(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .build()?;
    let response = client.get(url).send();
    let body = response.await.unwrap().text().await.unwrap();
    Ok(body)
}

pub async fn fetch_video_links(url: &str) -> Result<HashSet<String>, reqwest::Error> {
    let mut video_links = HashSet::new();
    let body = fetch_website(url).await?;
    let document = Document::from(body.as_str());
    for node in document.find(Name("a")) {
        if let Some(href) = node.attr("href") {
            if href.contains("video/") {
                video_links.insert(format!("https://cda.pl{}", href));
            }
        }
    }
    println!("1");
    Ok(video_links.into_iter().collect())
}
