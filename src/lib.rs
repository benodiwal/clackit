use cli::parse_args;

mod cli;
mod keys;
mod sounds;
mod constants;

pub fn start() {
    let args = parse_args();
    let soundpack = args.soundpack;
    let volume = args.volume.unwrap_or(constants::DEFAULT_VOLUME);
    println!("{}", soundpack);
    println!("{}", volume);
} 
