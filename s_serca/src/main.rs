use serca;
use serca::web::puppeteer::Puppeteer;
use serca::ai::inference::InferenceEngine;
use serca::ai::inference::Target;
use serca::ai::inference::Query;

use tokio;
use anyhow::Result;
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
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
    
    let mut inference_engine = InferenceEngine::new()
        .target(Target::OLLAMA);

    let query = Query{ 
        query_number: 1, 
        query_string: "Hello".to_string(), 
        query_responce: "".to_string() };

    inference_engine.inference(query).await;

    println!("{}", query.query_responce);
    println!("DONE");
    Ok(())
}

