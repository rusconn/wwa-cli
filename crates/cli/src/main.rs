use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;

use wwa_cli::{Args, Commands, analyze, breakpoints, completion};

fn main() -> ExitCode {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e:#}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn run(args: Args) -> Result<()> {
    match args.command {
        Commands::Analyze(args) => analyze(args),
        Commands::Breakpoints(args) => breakpoints(args),
        Commands::Completion(args) => completion(args),
    }
}
