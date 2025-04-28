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

pub struct Puppeteer {
    client: Arc<Mutex<Client>>,
    url_db: Vec<String>,
    max: i64,
    c_total: i64,
    marionettes: Vec<Marionette>,
    cached_urls: Arc<Mutex<HashSet<String>>>,
    black_list: Arc<Mutex<Vec<String>>>,
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
            client: Arc::new(Mutex::new(Client::new("http://localhost:4444").await.expect("10 bucks says geckodriver isn't running and/or isn't installed"))),
            url_db: lines,
            max: 5,
            c_total: 0,
            marionettes: Vec::new(),
            cached_urls: Arc::new(Mutex::new(HashSet::new())),
            black_list: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn url_db(mut self, url_db: Vec<String>) -> Self{
        self.url_db = url_db;
        self
    }

    fn start_geco() {
        let mut sys = System::new_all();
        sys.refresh_all();

        for (pid, process) in sys.processes() {
            if process.name() == "geckodriver" {
                println!("Found Geckodriver");
                if process.kill_with(Signal::Kill).is_none() {
                    println!("This signal isn't supported on this platform");
                } else {
                    println!("Restarting Geckodriver");
                    let output = Command::new("geckodriver")
                        .output()
                        .expect("Failed to run start geckodriver command");
                }
            }
        }
    }

    pub async fn load_blacklist(mut self) -> Result<()> {
        let mut file = File::open("foo.txt")?;
        let mut list = String::new();
        file.read_to_string(&mut list);
        let parts = list.split(" ");

        for part in parts {
            self.black_list.lock().await.push(part.to_string());
        }
        Ok(())
    }

    pub async fn check_blacklist(mut self, url: String) -> Result<bool> {
        let is_blacklisted = self.black_list.lock().await.iter().any(|blacklisted_url| blacklisted_url == &url);
        return Ok(is_blacklisted)
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

        let url_db_clone = self.url_db.clone();
        for (i, url) in url_db_clone.into_iter().enumerate() {
            pop_first(&url);
            let client = Arc::clone(&self.client); // move this inside the loop
            let id = self.c_total + i as i64; // or however you're assigning unique IDs

            log_spent_url(&url);
            println!("New Marionette spawned. Count is now {}", id);

            let handle = tokio::spawn(async move {
                println!("Made Marionette {}", url);
                let mut marionette = Marionette::new()
                    .url(url)
                    .id(id);

                let data = marionette.walk(client).await
                    .expect("Welp that page isn't accessible");

            data.urls
            });
            marionettes.push(handle);
        }
        println!("Marionettes spawned");

        println!("Waiting waiting Marionettes return");
        for marionette in marionettes { 
            let data = marionette.await.expect("Marionette failed to close up properly"); 
            let mut new_urls = Vec::new();
            for url in data {
                new_urls.push(url);
            }
            self.url_db.extend(new_urls);
        }
        println!("Cycle done");
    }
}

fn log_spent_url(url: &str) -> Result<()> {
    println!("Logging url");
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

fn pop_first(s: &str) {
    let mut chars = s.chars();

    if chars.next() == Some('/') {
        println!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    }
    else {
        println!("hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh");
    }
}
