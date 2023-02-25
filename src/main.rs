use reqwest;
use select::document::Document;
use select::predicate::Name;

fn fetch_website(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .build()?;
    let response = client.get(url).send()?;
    let body = response.text()?;
    Ok(body)
}

fn fetch_video_links(url: &str) -> Result<Vec<String>, reqwest::Error> {
    let mut video_links = vec![];
    let body = fetch_website(url)?;
    let document = Document::from(body.as_str());
    for node in document.find(Name("a")) {
        if let Some(href) = node.attr("href") {
            if href.contains("video") {
                video_links.push(format!("https://cda.pl{}", href));
            }
        }
    }
    Ok(video_links)
}

fn main() {
    let link = "https://www.cda.pl/MarcinXvZ/folder/32310244";
    let links = fetch_video_links(link).unwrap();
    println!("{:?}", links);
}