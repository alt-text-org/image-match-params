use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod stats;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    GenSignature {
        path: PathBuf
    },
    Stats {
        path: PathBuf
    }
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Stats { path } => {
            stats::check_match_percentages(&path)
        },
        Commands::GenSignature { path } => {
            println!("{:x?}", stats::get_file_signature(&path))
        }
    }
}
