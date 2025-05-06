use anyhow::Result;
use scraper::Html;
use scraper::Selector;
use fantoccini::{ Client, Locator };
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct PageData {
    pub spawn_url: String,
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
            url: "https://example.com".to_string(), 
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

    pub async fn walk(&mut self) -> Result<PageData> {
        //let client = client.lock().await;

        //println!("{} : {}", &self.id, &self.url);

        let mut page_data = PageData {
            spawn_url: self.url.clone(),
            meta_data: "NULL".to_string(),
            urls: Vec::new(),
            media: Vec::new(),
        };
      
        let html = reqwest::get(&self.url)
            .await?
            .text()
            .await?; 

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
                page_data.urls.push(format!("{}/{}", self.url, src.replace("/", "")));
            }   
        }
        Ok(page_data)
    }

    fn pop_first(s: &str) {
        let mut chars = s.chars();

        if chars.next() == Some('/') {
            println!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA {:?}", chars);
        }
        else {
            //println!("hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh {:?}", chars);
        }
    }
}

