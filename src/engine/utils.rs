use std::{error::Error, fs};
use rdev::Event;
use serde_json::{Map, Value};
use std::string::String;

fn initialize_json(path: &str) -> Result<Map<String, Value>, Box<dyn Error>> {
    let config = fs::read_to_string(path)?;
    let parsed: Value = serde_json::from_str(&config)?;
    let obj = parsed.as_object().unwrap().clone(); 
    Ok(obj)
}

pub struct File {
    pub content: Option<Map<String, Value>>,
}

impl File {
    pub fn initialize(&mut self, directory: String) {
        let soundpack_config = &format!("{}/config.json", directory)[..];
        self.content = Some(initialize_json(soundpack_config).unwrap());
    }

    pub fn event_handler(&self, event: Event, directory: String, vol: u16) {
        match &self.content {
            Some(value) => {},
            None => {
                println!("JSON wasn't initialized");
            }
        }
    }
}
