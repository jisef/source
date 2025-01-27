
use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {

    #[arg(short , long)]
    file: std::path::PathBuf,
    
    #[arg(short = 's', long = "seperator")]
    seperator: char,

    #[arg(short = 'h', long = "help")]
    help: bool,

    
}
