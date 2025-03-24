use serca;
use serca::web::puppeteer::Puppeteer;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Launching Puppeteer");
    let puppeteer = Puppeteer::new().url_db("./urls.txt".to_string());
    puppeteer.control();
    Ok(())
}

