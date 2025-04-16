use anyhow::Result;
use scraper::Html;
use scraper::Selector;
use fantoccini::{Client, Locator};
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

    pub async fn walk(&mut self, client: Arc<Mutex<Client>>) -> Result<PageData> {
        let client = client.lock().await;

        println!("{} : {}", &self.id, &self.url);

        let mut page_data = PageData {
            meta_data: "NULL".to_string(),
            urls: Vec::new(),
            media: Vec::new(),
        };
        if let Err(e) = client.goto(&self.url).await {
            eprintln!("Marionette can't get to url {}", e);
            return Ok(page_data);
        }

        //tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        println!("SUCCESS");

        match client.source().await {
        Ok(page_html) => {
            let document = Html::parse_document(&page_html);

            // Select images
            let img_selector = Selector::parse("img").unwrap();
            for img in document.select(&img_selector) {
                if let Some(src) = img.value().attr("src") { 
                        page_data.media.push(src.to_string());
                }
            }

            // Select links (page URLs)
            let link_selector = Selector::parse("a").unwrap();
            for link in document.select(&link_selector) {
                if let Some(href) = link.value().attr("href") {
                        page_data.urls.push(format!("{}/{}", self.url, href.to_string()));
                }
            }
        }
        Err(e) => eprintln!("Failed to get page source: {}", e),
    }
        Ok(page_data)
    }
}


