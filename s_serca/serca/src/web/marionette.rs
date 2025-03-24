use anyhow::Result;
use reqwest::Client;
use scraper::Html;
use scraper::Selector;

pub struct Marionette {
    url: String,
    depth: u8,
}

impl Marionette {
    pub fn new() -> Self {
        Self {
            url: "https://example.com/".to_string(), 
            depth: 0,
        }
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    pub async fn walk(&mut self) -> Result<String> {
        let client = reqwest::Client::new();

        let response = client
            .get(self.url.clone())  // Replace with actual Squarespace site
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .send()
            .await?;
        Ok(response.text().await?)
    }
}

async fn fetch_page(client: &Client, url: &str) -> Result<String, reqwest::Error> {
    let resp = client.get(url).send().await?.text().await?;
    Ok(resp)
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

