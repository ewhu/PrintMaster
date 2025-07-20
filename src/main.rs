// src/main.rs
/*
 * Main executable for PrintMaster
 */

use clap::Parser;
use printmaster::{Result, run};

#[derive(Parser)]
#[command(version, about = "PrintMaster - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
