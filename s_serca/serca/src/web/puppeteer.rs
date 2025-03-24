use anyhow::Result;
use crate::web::marionette;
use crate::web::marionette::Marionette;
use std::thread;
use std::fs::read_to_string;

pub struct Puppeteer {
    url_db: String,
    marionettes: Vec<Marionette>,
}

impl Puppeteer {
    pub fn new() -> Self {
        Puppeteer {
            url_db: "".to_string(),
            marionettes: Vec::new(),
        }
    }

    pub fn url_db(mut self, url_db: String) -> Self{
        self.url_db = url_db;
        self
    }

    pub fn control(mut self) {
        let control_thread = thread::spawn(move || {
            println!("{}", self.url_db);
            for line in read_to_string(self.url_db).unwrap().lines() {
                println!("{}", line);
                create_new_marionette()
            }
        });
    }

    fn create_new_marionette(mut self) {
        let marionette = Marionette::new();
        self.marionettes.push(marionette);
    }
}

