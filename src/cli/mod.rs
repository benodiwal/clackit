use clap::Parser;

mod args;
mod utils;

pub fn parse_args() {
    args::Args::parse();
}
