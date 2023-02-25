// use crate::folder::*;

// mod folder;
use crate::webdriver::*;
mod webdriver;

#[tokio::main]
async fn main(){
    let driver = init_driver();
    println!("{:?}",webtest(driver.await.unwrap(), "https://www.cda.pl/video/748082819").await.unwrap().url);
    // let link = "https://www.cda.pl/MarcinXvZ/folder/32310244";
    // let links = fetch_video_links(link).unwrap();
    // for link2 in links.iter() {
    //     println!("{:?}", link2);
    // }
}