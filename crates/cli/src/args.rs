use clap::{Parser, Subcommand};

use crate::{AnalyzeArgs, BreakpointsArgs, CompletionArgs};

#[derive(Debug, Parser)]
#[command(name = "wwa", version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Analyze damage change for each enemy
    Analyze(AnalyzeArgs),

    /// List breakpoint atks
    Breakpoints(BreakpointsArgs),

    /// Generate shell completion script
    Completion(CompletionArgs),
}
