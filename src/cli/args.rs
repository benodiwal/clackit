use clap::Parser;

use super::utils::HELP_MESSAGES;

#[derive(Debug, Parser)]
#[clap(
    name = "clackit",
    version,
    about = "A rust cli tool to make a normal keyboard to sound like a  mechanical keyboard"
)]

pub struct Args {
    #[arg(long, short, help = HELP_MESSAGES[0].as_str())]
    pub soundpack: String,
    
    #[arg(long, short, help = HELP_MESSAGES[1].as_str())]
    pub volume: Option<u16>
}
