use crate::webdriver::*;
use crate::folder::*;

mod webdriver;
mod folder;

#[tokio::main]
async fn main(){
    let f_link = "{link}";
    let links = fetch_video_links(f_link).await.unwrap();
    let linki = webtest(links).await.unwrap();
    for link in linki {
        print!("{:?} --- ", link.name);
        println!("{:?}", link.url);
    }
}