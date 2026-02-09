mod aipipe;

use serca;
use serca::web::puppeteer::Puppeteer;
use tokio;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use anyhow::Result;
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("")?;
    let response = gemini_generate_content(
        &api_key,
        "gemini-3-flash-preview",
        "Explain how AI works in a few words",
    )
        .await?;

    println!("Gemini says: {}", response);
    // fs::remove_file("spent_urls.txt").unwrap_or_else(|why| {
    //     println!("! {:?}", why.kind());
    // });
    // println!("Launching Puppeteer");
    //
    // let puppeteer = Puppeteer::new();
    // match puppeteer.await.control().await {
    //     Ok(()) => println!("The loop exited safely, but it still shouldn't have ended"),
    //     Err(e) => println!("The loop exited with an error {}", e)
    // }

    println!("DONE");
    Ok(())
}

