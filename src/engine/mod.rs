use rdev::{listen, Event};
use utils::File;

mod utils;

pub fn start(soundpack: String, vol: u16) {
    {
        unsafe {
            use libc::nice;
            nice(-20);
        };
    }

    let mut file = File { content: None };
    file.initialize(soundpack.clone());

    let event_handler = move |event: Event| {
        file.event_handler(event, soundpack.clone(), vol);
    };

    if let Err(err) = listen(event_handler) {
        println!("Error: {:?}", err);
    }
}
