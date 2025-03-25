use serca;
use serca::web::puppeteer::Puppeteer;
use tokio;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use anyhow::Result;
use std::process::Command;

fn create_file(path: &str) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(b"Kilroy Was Here")?;
    Ok(())
}

fn setup() -> Result<()> {
    println!("Validating dirs...");

    let urls_file = Path::new("urls.txt");
    let spent_urls_file = Path::new("spent_urls.txt");
    let media_found_cache_file = Path::new("found_media.txt");

    if urls_file.exists() {
        println!("urls_file valid");
    } else {
        println!("urls_file ain't there");
        let _ = create_file(urls_file.to_str().unwrap());
        println!("We made it for you. Please populate it with real urls");
    }

    if spent_urls_file.exists() {
        println!("spent_urls_file is valid");
    } else {
        println!("spent_urls_file is not here bro");
        let _ = create_file(spent_urls_file.to_str().unwrap());
        println!("We made it for you");
    }

    if media_found_cache_file.exists() {
        println!("media_found_cache_file is valid");
    } else {
        println!("media_found_cache_file not in this bag");
        let _ = create_file(media_found_cache_file.to_str().unwrap());
        println!("We made for you. Your welcome.");
    }

    println!("All file tests passed with flying colors");

    //println!("Starting geckodriver");

    //Command::new("cmd")
    //    .args(["geckodriver"])
    //    .output()
    //    .expect("failed to execute process");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running setup...");
    let _ = setup();
    println!("Launching Puppeteer");

    let puppeteer = Puppeteer::new();
    puppeteer.await.control().await;

    println!("DONE");
    Ok(())
}

