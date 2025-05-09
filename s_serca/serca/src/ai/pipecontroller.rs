use crate::ai::model::Model;
use crate::ai::inference::{ inference, Target, Query };
use std::env;

#[derive(Debug, PartialEq)]
pub enum Step {
    STANDBY,
    AUDIO,
    VIDEO,
    LLM,
    END,
}

#[derive(Debug)]
pub enum OS {
    UNKNOWN,
    LINUX,
    WINDOWS,
    MAC
}

pub struct PipeController {
    step: Step,
    os: OS,
    global_thoughts: String,
}

impl PipeController {
    pub fn new() -> Self {
        PipeController {
            step: Step::STANDBY,
            os: Self::get_os(),
            global_thoughts: "".to_string(),
        }
    }

    fn get_os() -> OS {
        #[cfg(target_os = "windows")]
        {
            return OS::WINDOWS;
        }
        #[cfg(target_os = "linux")]
        {
            return OS::LINUX;
        }
        #[cfg(target_os = "macos")]
        {
            return OS::MACOS;
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

    pub async fn control(&mut self, num_batches: i64, batch_size: i64, bat_bat_thots: bool) {
        println!("\n-- Running batch/es");
        println!("-- {} batch/es to run", num_batches);
        println!("-- Batch size is {}", batch_size);
        println!("-- {} to retain thoughts batch to batch", bat_bat_thots);
        //self.run_batch(batch_size).await;
        for i in 0..num_batches {
            println!("{}/{}", i , num_batches);
            self.run_batch(batch_size).await;
            if bat_bat_thots != true {
                self.global_thoughts = "".to_string();
                println!("Thoughts forgurt");
            }
        }
    }

    async fn run_batch(&mut self, batch_size: i64) {
        if self.step == Step::STANDBY {
            loop {
                match self.step {
                    Step::STANDBY => {
                        self.standby().await;
                    }
                    Step::AUDIO => {
                        self.audio().await;
                    }
                    Step::VIDEO => {
                        self.video().await;
                    }
                    Step::LLM => {
                        self.llm().await;
                    }
                    Step::END => {
                        self.end().await;
                    }
                }
                self.next_step();

                if self.step == Step::END {
                    break;
                }
            }
        }
    }

    pub fn check(&self) {
        println!("\n------------PipeController Check------------");
        println!("{:?}", self.step);
        println!("{:?}", self.os);
        println!("--------------------------------------------\n");
    }

    async fn standby(&self) { 
        println!("Standby")
    }

    async fn audio(&mut self) {
        println!("Audio")
    }

    async fn video(&self) {
        println!("Video")
    }

    async fn llm(&self) { 
        let query = Query {
            query_string: "NULL".to_string(),
            query_responce: "NULL".to_string(),
            model: Model { model_name: "llama3.2:latest".to_string() },
            target: Target::OLLAMA,
        };
        let resp = inference(query).await;

        println!("LLM")
    }

    async fn end(&self) {
        println!("END")
    }
}
