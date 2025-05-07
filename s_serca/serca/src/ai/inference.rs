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
    model: Model
}

#[derive(PartialEq)]
pub struct Query {
    pub query_number: i64,
    pub query_string: String,
    pub query_responce: String,
}

impl InferenceEngine {
    pub fn new() -> Self {
        InferenceEngine {
            target: Target::UNKNOWN,
            model: Model { model_name: "NULL".to_string(), sys_prompt: "ah2".to_string() }
        }
    }

    pub fn target(mut self, target: Target) -> Self {
        self.target = target;
        self
    }

    pub fn model(mut self, model: String) -> Self {
        self.model.model_name = model;
        self
    }

    pub async fn inference(&mut self, query: Query) -> Result<(), String> { 
        if query.query_number == 0 {
            return Err("Query number is invalid".to_string());
        }

        match self.target {
            Target::UNKNOWN => return Err("Query type is unknown".to_string()),
            Target::OLLAMA(e) => self.handle_ollama(query).await,
            Target::COREML => self.handle_coreml(query).await,
            Target::AIROUTER => self.handle_airouter(query).await,
        }
        // inference logic

        Ok(())
    }

    async fn handle_ollama(&mut self, mut query: Query) -> Query {
        println!("Asking Ollama");
        let ollama = Ollama::default();
        let prompt = format!("{} {}", self.model.sys_prompt, query.query_string);
        let res = ollama.generate(GenerationRequest::new(self.model.model_name.clone(), prompt)).await;
        if let Ok(res) = res {
            query.query_responce = res.response;
        } else {
            query.query_responce = "E".to_string();
        }
        return query
    }

    async fn handle_coreml(&mut self, query: Query) {}

    async fn handle_airouter(&mut self, query: Query) {}
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
        let temp_query = Query { query_number: 0, query_string: "NULL".to_string(), query_responce: "NULL".to_string() };
        
        if self == temp_query {
            return false
        }
        true
    }
}
