use anyhow::Result;

pub struct Marionette {
    base_url: String,
    curr_url: String,
    depth: u8,
}

impl Marionette {
    pub fn new() -> Self {
        Self {
            base_url: "https://example.com/".to_string(),
            curr_url: "https://example.com/".to_string(), 
            depth: 0,
        }
    }

    pub async fn pull_page(mut self) -> Result<String> {
        let client = reqwest::Client::new();

        let response = client
            .get(self.curr_url)  // Replace with actual Squarespace site
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .send()
            .await?;
        Ok(response.text().await?)
    }

    pub fn parse_page(mut self) {

    }
}
