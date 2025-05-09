use dotenv::dotenv;
use serca;
use serca::ai::pipecontroller::PipeController;
use std::env;
use tokio::net::unix::pipe::pipe;

use anyhow::Result;
use std::fs;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    //fs::remove_file("spent_urls.txt").unwrap_or_else(|why| {
    //    println!("! {:?}", why.kind());
    //});
    //println!("Launching Puppeteer");
    //
    //let puppeteer = Puppeteer::new();
    //match puppeteer.await.control().await {
    //    Ok(()) => println!("The loop exited safely, but it still shouldn't have ended"),
    //    Err(e) => println!("The loop exited with an error {}", e)
    //}
    //

    let mut pipe_controller = PipeController::new();
    pipe_controller.check();
    pipe_controller.control(32, 4, true).await;

    println!("DONE");
    Ok(())
}
