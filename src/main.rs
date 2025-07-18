// src/main.rs
/*
 * Main executable for ArtificialCanvasGeneratorAPINext
 */

use clap::Parser;
use artificialcanvasgeneratorapinext::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialCanvasGeneratorAPINext - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
