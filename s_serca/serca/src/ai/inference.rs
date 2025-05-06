use ollama_rs::Ollama;

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
}

#[derive(PartialEq)]
pub struct Query {
    query_number: i64,
    model: Model,
    query_string: String,
    query_responce: String,
}


impl InferenceEngine {
    pub fn new() -> Self {
        InferenceEngine {
            target: Target::UNKNOWN,
        }
    }

    pub fn set_target(mut self, target: Target) {
        self.target = target;
    }

    pub fn inference(&mut self, query: Query) -> Result<(), String> {
        if query.query_number == 0 {
            return Err("Query number is invalid".to_string());
        }

        match self.target {
            Target::UNKNOWN => return Err("Query type is unknown".to_string()),
            Target::OLLAMA => self.handle_ollama(query),
            Target::COREML => self.handle_coreml(query),
            Target::AIROUTER => self.handle_airouter(query),
        }
        // inference logic

        Ok(())
    }

    fn handle_ollama(&mut self, query: Query) {
        let ollama = Ollama::default();
    }

    fn handle_coreml(&mut self, query: Query) {}

    fn handle_airouter(&mut self, query: Query) {}
}

impl Query {
    pub fn new() -> Self {
        Query {
            query_number: 0,
            model: Model { model_name: "NULL".to_string() },
            query_string: "NULL".to_string(),
            query_responce: "NULL".to_string(),
        }
    }

    pub fn validate_query(self) -> bool {
        let temp_query = Query { query_number: 0, model: Model { model_name: "NULL".to_string() }, query_string: "NULL".to_string(), query_responce: "NULL".to_string() };
        
        if self == temp_query {
            return false
        }
        true
    }
}
