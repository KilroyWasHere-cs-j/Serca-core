use reqwest::Client;
use serde_json::json;
use std::env;

use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;

use crate::ai::model::Model;


#[derive(PartialEq, Debug, Clone)]
pub enum Target {
    UNKNOWN,
    OLLAMA,
    COREML,
    AIROUTER,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Query {
    pub query_string: String,
    pub query_responce: String,
    pub model: Model,
    pub target: Target,
}


pub async fn inference(query: Query) -> Result<Query, String> {
    match query.target {
        Target::UNKNOWN => Err("Query type is unknown".to_string()),
        Target::OLLAMA => handle_ollama(query).await,
        Target::COREML => handle_coreml(query).await,
        Target::AIROUTER => handle_airouter(query).await,
    }
}

async fn handle_ollama(mut query: Query) -> Result<Query, String> {
    //println!("Asking Ollama");
    let ollama = Ollama::default();
    let prompt = format!("{}", query.query_string);
    let res = ollama
        .generate(GenerationRequest::new(
            query.model.model_name.clone(),
            prompt,
        ))
        .await;
    if let Ok(res) = res {
        query.query_responce = res.response;
    } else {
        query.query_responce = "E".to_string();
    }
    Ok(query)
}

async fn handle_coreml(query: Query) -> Result<Query, String> {
    Ok(query)
}

async fn handle_airouter(mut query: Query) -> Result<Query, String> {
    let client = Client::new();

    let request_body = json!({
        "model": "meta-llama/llama-3.2-11b-vision-instruct:free",
        "messages": [
            {
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "What is in this image?"
                    },
                    {
                        "type": "image_url",
                        "image_url": {
                            "url": "https://upload.wikimedia.org/wikipedia/commons/thumb/d/dd/Gfp-wisconsin-madison-the-nature-boardwalk.jpg/2560px-Gfp-wisconsin-madison-the-nature-boardwalk.jpg"
                        }
                    }
                ]
            }
        ]
    });

    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", "joigji"))
        .json(&request_body)
        .send()
        .await
        .unwrap();

    query.query_responce = response.text().await.unwrap();
    Ok(query)
}

