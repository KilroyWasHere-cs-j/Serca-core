// let query = Query {
//     query_number: 1,
//     query_string: "Hello".to_string(),
//     query_responce: "".to_string(),
// };

// let mut inference_engine = InferenceEngine::new()
//     .target(Target::AIROUTER)
//     .model_name("llama3.2:latest".to_string())
//     .key(env::var("OPENROUTER_API_KEY").expect("OPENROUTER_API_KEY not set in .env"));

// let resp = inference_engine.inference(query).await;
// println!("{:?}", resp);

use crate::ai::inference::InferenceEngine;
use crate::ai::inference::Target;

#[derive(Debug, PartialEq)]
pub enum Step {
    STANDBY,
    AUDIO,
    VIDEO,
    LLM,
    END,
}

pub struct PipeController {
    step: Step,
    inference_engine: InferenceEngine,
}

impl PipeController {
    pub fn new() -> Self {
        PipeController {
            step: Step::STANDBY,
            inference_engine: InferenceEngine::new(),
        }
    }

    fn next_step(&mut self) {
        match self.step {
            Step::STANDBY => self.step = Step::AUDIO,
            Step::AUDIO => self.step = Step::VIDEO,
            Step::VIDEO => self.step = Step::LLM,
            Step::LLM => self.step = Step::END,
            Step::END => self.step = Step::STANDBY,
        }
    }

    pub fn run_batch(&mut self) {
        if self.step == Step::STANDBY {
            loop {
                match self.step {
                    Step::STANDBY => {
                        self.standby();
                    }
                    Step::AUDIO => {
                        self.audio();
                    }
                    Step::VIDEO => {
                        self.video();
                    }
                    Step::LLM => {
                        self.llm();
                    }
                    Step::END => {
                        self.end();
                    }
                }
                self.next_step();

                if self.step == Step::END {
                    break;
                }
            }
        }
    }

    fn standby(&self) {
        println!("Standby")
    }

    fn audio(&mut self) {
        // self.inference_engine
        //     .target(Target::OLLAMA)
        //     .model_name("llama3.2:latest".to_string());
        println!("Audio")
    }

    fn video(&self) {
        println!("Video")
    }

    fn llm(&self) {
        println!("LLM")
    }

    fn end(&self) {
        println!("END")
    }
}
