use serca;
use serca::web::puppeteer::Puppeteer;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Launching Puppeteer");
    let puppeteer = Puppeteer::new();
    puppeteer.control().await;
    Ok(())
}

