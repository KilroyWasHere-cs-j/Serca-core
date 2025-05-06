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
use url::Url;
use std::path::Path;

pub struct Puppeteer {
    //client: Arc<Mutex<Client>>,
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
            //client: Arc::new(Mutex::new(Client::new("http://localhost:4444").await.expect("10 bucks says geckodriver isn't running and/or isn't installed"))),
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
  
    pub async fn control(mut self) -> Result<()>{
        loop {
            {
                self.url_db = self.run_batch().await?;
                if self.url_db.len() == 0 {
                    break Ok(());
                }
            }
        }
    }

    async fn run_batch(&mut self) -> Result<Vec<String>> {

        let mut marionettes = vec![];

        println!("\n*******************************************");
        println!("Loading in URL batch");
        println!("*******************************************");

        let mut count = 0;

        marionettes.clear();
        let url_db_clone = self.url_db.clone();
        for url in url_db_clone {
            count += 1;
            println!("{}", url);
            let mari_handle = tokio::spawn(async move {
                let mut marionette = Marionette::new()
                    .url(url.to_string())
                    .id(0);
                let data = marionette.walk().await
                    .expect("Welp that page isn't accessible");

                match marionette.walk().await {
                    Ok(data) => return data,
                    Err(e) => PageData {
                                spawn_url: "NULL".to_string(),
                                meta_data: "NULL".to_string(),
                                urls: Vec::new(),
                                media: Vec::new(),
                            }
                }

            });
            marionettes.push(mari_handle);
        }
        self.url_db.clear();

        println!("\n*******************************************");
        println!("Waiting on Marionettes");
        println!("*******************************************");


        if marionettes.len() == 0 {
            println!("Batch is done returning with empty hands");
            return Ok(vec![]);
        } else if marionettes.len() != 0 {
            println!("A marionette has returned, current length of deployed marionettes is {}", marionettes.len());
            let mut uncovered_urls = vec![];
            for m_hndl in marionettes {
                let m_data = m_hndl.await.expect("Marionette failed to properly resolve");
 
                println!("\n----------------------------------------------------");
                println!("{}", m_data.spawn_url);
                println!("Validating the returned urls");
                println!("----------------------------------------------------");

                for url in &m_data.urls {
                    // Is URL is terminal
                    if Self::is_terminal_url(url) {
                        // Is terminal
                        log_spent_url(url);
                    } else {
                        // Is not terminal
                        uncovered_urls.push(url.to_string());
                    }
                }
            }
            println!("Batch is done");
            return Ok(uncovered_urls);
        }
        println!("Batch is returning for an unknown reason");
        Ok(vec![])
    }

    fn is_terminal_url(url_str: &str) -> bool {
        let known_exts = [
            // Video
            "mp4", "webm", "mov", "avi", "mkv", "flv", "wmv", "mpeg", "mpg", "m4v",
            // Audio
            "mp3", "wav", "flac", "aac", "ogg", "m4a", "wma",
            // Images
            "jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "svg", "ico",
            // Documents (optional, often considered "final")
            "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "rtf",
            // Archives
            "zip", "rar", "7z", "tar", "gz", "bz2", "xz",
            // Code/files
            "exe", "bin", "apk", "iso", "dmg",
            // Web pages
            "html", "js", "css", "htm",
            // Text
            "pdf", "txt", "doc", "rtf",
        ];

        let parsed = match Url::parse(url_str) {
            Ok(u) => u,
            Err(_) => return false,
        };

        let path = Path::new(parsed.path());

        match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => known_exts.iter().any(|&e| e.eq_ignore_ascii_case(ext)),
            None => false,
        }
    }
}

fn log_spent_url(url: &str) -> Result<()> {
    //println!("Logging url {}", url);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("./spent_urls.txt")?;

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
