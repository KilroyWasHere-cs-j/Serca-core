use tokio;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use anyhow::Result;
use std::process::Command;

mod media;

use crate::media::streamer::{ stream_frames };
use crate::media::ripper;

fn create_file(path: &str) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(b"Kilroy Was Here")?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    stream_frames("https://a.nwps.fi/1399544348751.jpg").await;
    println!("DONE");
    Ok(())
}

