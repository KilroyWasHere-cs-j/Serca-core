use serca;
use serca::web::puppeteer::Puppeteer;
use tokio;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use anyhow::Result;
use std::process::Command;
use std::fs;

fn create_file(path: &str) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(b"Kilroy Was Here")?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::remove_file("spent_urls.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    println!("Launching Puppeteer");

    let puppeteer = Puppeteer::new();
    puppeteer.await.control().await;

    println!("DONE");
    Ok(())
}

