use anyhow::Result;
use scraper::Html;
use scraper::Selector;
use fantoccini::{ Client, Locator };
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct PageData {
    pub meta_data: String,
    pub urls: Vec<String>,
    pub media: Vec<String>,
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

    pub fn id(mut self, id: i64) -> Self {
        self.id = id;
        self
    }

    pub async fn walk(&mut self) -> Option<PageData> {
        //let client = client.lock().await;

        //println!("{} : {}", &self.id, &self.url);

        if self.url.contains(".mp4") || self.url.contains(".mp3") || self.url.contains(".jpg") || self.url.contains(".png") {
            return None;
        }
        let mut page_data = PageData {
            meta_data: "NULL".to_string(),
            urls: Vec::new(),
            media: Vec::new(),
        };

        let html = reqwest::get(&self.url)
            .await.unwrap()
            .text()
            .await.unwrap();

        //let fragment = Html::parse_fragment(&body);

        let document = Html::parse_document(&html);

        let link_selector = Selector::parse("a").unwrap();
        for element in document.select(&link_selector) {
            if let Some(href) = element.value().attr("href") { 
                page_data.urls.push(format!("{}/{}", self.url, href.replace("/", "")));
            }
        }

        // Extract <img src="...">
        let img_selector = Selector::parse("img").unwrap();
        for element in document.select(&img_selector) {
            if let Some(src) = element.value().attr("src") {
                //println!("Image: {}", src);
            }
        }
        Some(page_data)
    }
}


