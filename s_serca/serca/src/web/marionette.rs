use anyhow::Result;
use scraper::Html;
use scraper::Selector;
use fantoccini::{Client, Locator};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct PageData {
    meta_data: String,
    urls: Vec<String>,
    images: Vec<String>,
}

pub struct Marionette {
    id: i64,
    url: String,
    depth: u8,
}

impl Marionette {
    pub fn new() -> Self {
        Self {
            id: 0,
            url: "https://example.com/".to_string(), 
            depth: 0,
        }
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    pub async fn walk(&mut self, client: Arc<Mutex<Client>>) -> Result<Vec<String>> {
        let client = client.lock().await;

        println!("{}", &self.url);

        if let Err(e) = client.goto(&self.url).await {
            eprintln!("Marionette can't get to url {}", e);
            return Ok(vec![]);
        }


        Ok(vec!["".to_string(), "".to_string()])
    }
}

// Extracts links from the page
fn extract_links(body: &str, base_url: &str) -> Vec<String> {
    let document = Html::parse_document(body);
    let selector = Selector::parse("a").unwrap();
    let base = url::Url::parse(base_url).unwrap();

    document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .filter_map(|href| base.join(href).ok())
        .map(|url| url.to_string())
        .collect()
}


//fn extract_video_links(body: &str, base_url: &str) -> Result<Vec<String>> {
//    let document = Html::parse_document(body);
//
//    // Selectors for video sources
//    let video_selector = Selector::parse("video source")?;
//    let anchor_selector = Selector::parse("a")?;
//
//    let base = Url::parse(base_url)?;
//
//    let mut links = Vec::new();
//
//    // Extract from <video> sources
//    for element in document.select(&video_selector) {
//        if let Some(src) = element.value().attr("src") {
//            if let Ok(url) = base.join(src) {
//                links.push(url.to_string());
//            }
//        }
//    }
//
//    // Extract from <a> tags linking to video files
//    for element in document.select(&anchor_selector) {
//        if let Some(href) = element.value().attr("href") {
//            if href.ends_with(".mp4") || href.ends_with(".webm") || href.ends_with(".m3u8") {
//                if let Ok(url) = base.join(href) {
//                    links.push(url.to_string());
//                }
//            }
//        }
//    }
//
//    Ok(links)
//}
