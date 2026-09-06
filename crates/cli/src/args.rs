use clap::{Parser, Subcommand};

use crate::{AnalyzeArgs, BreakpointsArgs};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
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
}
