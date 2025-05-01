use anyhow::Result;
use crate::web::marionette;
use crate::web::marionette::{ Marionette, PageData };
use std::thread;
use std::fs::read_to_string;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashSet;
use fantoccini::{Client, Locator};
use std::fs::File;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use sysinfo::{ Process, System, Signal };
//use std::collections::HashSet;

pub struct Puppeteer {
    //client: Arc<Mutex<Client>>,
    url_db: Vec<String>,
    max: i64,
    c_total: i64,
    marionettes: Vec<Marionette>,
    cached_urls: Arc<Mutex<HashSet<String>>>,
}

impl Puppeteer {
    pub async fn new() -> Self {
        let contents = read_to_string("urls.txt").expect("urls.txt blew up in our face");
        let lines: Vec<String> = contents
            .lines()
            .map(|line| line.trim().to_string())
            .collect();

        println!("Got urls");

        Puppeteer {
            //client: Arc::new(Mutex::new(Client::new("http://localhost:4444").await.expect("10 bucks says geckodriver isn't running and/or isn't installed"))),
            url_db: lines,
            max: 5,
            c_total: 0,
            marionettes: Vec::new(),
            cached_urls: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn url_db(mut self, url_db: Vec<String>) -> Self{
        self.url_db = url_db;
        self
    }

    pub async fn control(mut self) {

        loop {
            {
                self.run_cycle().await;
                if self.url_db.len() == 0 {
                    break;
                }
            }
        }
    }

    async fn run_cycle(&mut self) {
        println!("New cycle");
        //Self::start_geco();
        let mut marionettes = vec![];
        //let mut recovered_data = vec![];
        self.c_total += 1;
        let id = self.c_total;

        //println!("URL db len: {}", self.c_total);
        let mut url_db_clone = self.url_db.clone();
        for (i, url) in url_db_clone.into_iter().enumerate() {
            //let client = Arc::clone(&self.client); // move this inside the loop
            self.c_total + i as i64;
            let id = self.c_total;
            //url_db_clone.pop();
            log_spent_url(&url);
            //println!("New Marionette spawned. Count is now {}", id);
            //println!("{}", url);
            let handle = tokio::spawn(async move {
                let mut marionette = Marionette::new()
                    .url(url)
                    .id(id);

                //let data = marionette.walk().await
                    //.expect("Welp that page isn't accessible");

                match marionette.walk().await {
                    Some(data) => return data.urls,
                    None => Vec::new()
                }
            });
            marionettes.push(handle);
        }
        println!("Marionettes spawned");
        self.url_db.clear();
        println!("Waiting waiting Marionettes return {}", self.c_total);
        for marionette in marionettes {
            self.c_total - 1;
            let data = marionette.await.expect("Marionette failed to close up properly"); 
            let mut new_urls = Vec::new();
            if data.len() != 0 {
                for url in data {
                    new_urls.push(url);
                }
            }
            else {
                //println!("Nothing can be found");
            }
            if self.c_total == 0 {
                break;
            }
            self.url_db.extend(new_urls);
        }
        println!("Cycle done");
    }
}

fn log_spent_url(url: &str) -> Result<()> {
    //println!("Logging url {}", url);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("spent_urls.txt")?;

    writeln!(file, "{}", format!("{}", url));
    Ok(())
}

fn dump_file() -> Result<()> {
    Ok(())
}

fn flush_to_file(page_data: PageData) -> Result<()> {
    println!("Flushing page_data");

    let mut file = OpenOptions::new()
        .create(true)  // Create the file if it doesn't exist
        .append(true)  // Append mode
        .open("found_media.txt")?;

    writeln!(file, "{}", format!("{}", "{------------------------------------------------}"));
    writeln!(file, "{}", format!("MEDIA -> {:?}", page_data.media))?;
    writeln!(file, "{}", format!("{}", "{------------------------------------------------}"));

    let mut s_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("found_urls.txt")?;

    for url in page_data.urls {
        writeln!(s_file, "{}", format!("{:?}", url));
    }
    
    Ok(())
}
