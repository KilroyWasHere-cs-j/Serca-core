use anyhow::Result;
use crate::web::marionette;
use crate::web::marionette::Marionette;
use std::thread;
use std::fs::read_to_string;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashSet;

pub struct Puppeteer {
    url_db: Vec<String>,
    max: i64,
    marionettes: Vec<Marionette>,
    cached_urls: Arc<Mutex<HashSet<String>>>,
}

impl Puppeteer {
    pub fn new() -> Self {
        let contents = read_to_string("urls.txt").expect("Damnit");
        let lines: Vec<String> = contents
            .lines()
            .map(|line| line.trim().to_string())
            .collect();

        Puppeteer {
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
            let marionette = tokio::spawn(async move {
                let mut marionette = Marionette::new()
                    .url(url);
                let html = marionette.walk().await; 
                println!("Marionette alive");
            });
            marionettes.push(marionette);
        }
        
        for marionette in marionettes {
            marionette.await.unwrap();
        }
    }
}


