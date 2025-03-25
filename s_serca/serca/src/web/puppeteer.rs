use anyhow::Result;
use crate::web::marionette;
use crate::web::marionette::Marionette;
use std::thread;
use std::fs::read_to_string;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashSet;
use fantoccini::{Client, Locator};

pub struct Puppeteer {
    client: Arc<Mutex<Client>>,
    url_db: Vec<String>,
    max: i64,
    marionettes: Vec<Marionette>,
    cached_urls: Arc<Mutex<HashSet<String>>>,
}

impl Puppeteer {
    pub async fn new() -> Self {
        let contents = read_to_string("urls.txt").expect("Damnit");
        let lines: Vec<String> = contents
            .lines()
            .map(|line| line.trim().to_string())
            .collect();

        Puppeteer {
            client: Arc::new(Mutex::new(Client::new("http://localhost:4444").await.expect("10 bucks says geckodriver isn't running and/or isn't installed"))),
            url_db: lines,
            max: 5,
            marionettes: Vec::new(),
            cached_urls: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn url_db(mut self, url_db: Vec<String>) -> Self{
        self.url_db = url_db;
        self
    }

    pub async fn control(mut self) {
        let mut marionettes = vec![];

        for url in self.url_db {
            let client_clone = Arc::clone(&self.client);
            let marionette = tokio::spawn(async move {
                let mut marionette = Marionette::new()
                    .url(url);
                let html = marionette.walk(client_clone).await; 
            });
            marionettes.push(marionette);
        }

        // TODO need to handle this correctly
        //self.client.lock().await.close().await.expect("Failed to close client");

        for marionette in marionettes {
            marionette.await.expect("Marionette failed to close up properly");
        }
    }
}

