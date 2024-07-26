use cli::parse_args;

mod cli;
mod keys;
mod sounds;
mod constants;
mod engine;

pub fn start() {
    let args = parse_args();
    let soundpack = args.soundpack;
    let volume = args.volume.unwrap_or(constants::DEFAULT_VOLUME);
    engine::start(soundpack, volume);
} 
