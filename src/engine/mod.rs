use rdev::{listen, Event};
use utils::File;

mod utils;

pub fn start(soundpack: String, vol: u16) {
    
    {
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        unsafe {
            use libc::nice;
            if nice(-20) != 0 {
                eprintln!("Failed to set process priority");
            } else {
                println!("Process priority set successfully.");
            }
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
