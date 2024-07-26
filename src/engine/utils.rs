use std::{collections::HashSet, error::Error, fs, sync::Mutex};
use once_cell::sync::Lazy;
use rdev::{Event, EventType};
use serde_json::{Map, Value};
use std::string::String;
use crate::{keys, sounds};

static KEY_DEPRESSED: Lazy<Mutex<HashSet<i32>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub struct File {
    pub content: Option<Map<String, Value>>,
}

impl File {
    pub fn initialize(&mut self, soundpack: String) {
        let soundpack_config = &format!("{}/config.json", soundpack)[..];
        self.content = Some(initialize_json(soundpack_config).unwrap());
    }

    pub fn event_handler(&self, event: Event, soundpack: String, vol: u16) {
        match &self.content {
            Some(value) => {
                callback(event, value.clone(), soundpack, vol);
            },
            None => {
                println!("JSON wasn't initialized");
            }
        }
    }
}

fn initialize_json(path: &str) -> Result<Map<String, Value>, Box<dyn Error>> {
    let config = fs::read_to_string(path)?;
    let parsed: Value = serde_json::from_str(&config)?;
    let obj = parsed.as_object().unwrap().clone(); 
    Ok(obj)
}

fn callback(event: Event, json_file: Map<String, Value> ,soundpack: String, vol: u16) {
    match event.event_type {
        EventType::KeyPress(key) => {
            let key_code = keys::code_from_key(key);
            let key_down = KEY_DEPRESSED.lock().expect("Can't open key_depressed set").insert(key_code.unwrap_or(0));
            if key_down {
                let mut dest = match key_code {
                    Some(code) => json_file["defines"][&code.to_string()].to_string(),
                    None => {
                        println!("Unmapped key: {:?}", key);
                        let default_key = 30;
                        json_file["defines"][&default_key.to_string()].to_string()
                    }
                };
                dest.remove(0);
                dest.remove(dest.len()-1);
                sounds::play_sound(format!("{}/{}", soundpack, dest), vol);
            }
        }
        EventType::KeyRelease(key) => {
            let key_code = keys::code_from_key(key);
            KEY_DEPRESSED.lock().expect("Can't open key_depressed set for removal").remove(&key_code.unwrap_or(0));
        }
        _ => ()
    }
}
