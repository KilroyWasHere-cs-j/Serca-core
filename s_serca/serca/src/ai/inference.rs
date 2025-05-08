use reqwest::Client;
use serde_json::json;
use std::env;

use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;

use crate::ai::model::Model;

pub trait AiTarget {
    fn new() -> Self;
}

pub enum Target {
    UNKNOWN,
    OLLAMA,
    COREML,
    AIROUTER,
}

pub struct InferenceEngine {
    target: Target,
    model: Model,
    key: String,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Query {
    pub query_number: i64,
    pub query_string: String,
    pub query_responce: String,
}

impl InferenceEngine {
    pub fn new() -> Self {
        InferenceEngine {
            target: Target::UNKNOWN,
            model: Model {
                model_name: "NULL".to_string(),
                sys_prompt: "ah2".to_string(),
            },
            key: "dsnmiUH(K BU&^op-sqjoxijsalk".to_string(),
        }
    }

    pub fn target(mut self, target: Target) -> Self {
        self.target = target;
        self
    }

    pub fn model_name(mut self, model_name: String) -> Self {
        self.model.model_name = model_name;
        self
    }

    pub fn key(mut self, key: String) -> Self {
        self.key = key;
        self
    }

    pub async fn inference(&mut self, query: Query) -> Result<Query, String> {
        if query.query_number == 0 {
            return Err("Query number is invalid".to_string());
        }

        match self.target {
            Target::UNKNOWN => Err("Query type is unknown".to_string()),
            Target::OLLAMA => self.handle_ollama(query).await,
            Target::COREML => self.handle_coreml(query).await,
            Target::AIROUTER => self.handle_airouter(query).await,
        }
    }

    async fn handle_ollama(&mut self, mut query: Query) -> Result<Query, String> {
        println!("Asking Ollama");
        let ollama = Ollama::default();
        let prompt = format!("{} {}", self.model.sys_prompt, query.query_string);
        let res = ollama
            .generate(GenerationRequest::new(
                self.model.model_name.clone(),
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

    async fn handle_coreml(&mut self, query: Query) -> Result<Query, String> {
        Ok(query)
    }

    async fn handle_airouter(&mut self, mut query: Query) -> Result<Query, String> {
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
            .header("Authorization", format!("Bearer {}", self.key))
            .json(&request_body)
            .send()
            .await
            .unwrap();

        query.query_responce = response.text().await.unwrap();
        Ok(query)
    }
}

impl Query {
    pub fn new() -> Self {
        Query {
            query_number: 0,
            query_string: "NULL".to_string(),
            query_responce: "NULL".to_string(),
        }
    }

    pub fn validate_query(self) -> bool {
        let temp_query = Query {
            query_number: 0,
            query_string: "NULL".to_string(),
            query_responce: "NULL".to_string(),
        };

        if self == temp_query {
            return false;
        }
        true
    }
}
