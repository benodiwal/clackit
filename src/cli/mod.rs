use args::Args;
use clap::Parser;

mod args;
mod utils;

pub fn parse_args() -> Args {
    args::Args::parse()
}
