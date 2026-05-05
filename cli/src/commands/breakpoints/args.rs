use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// Path to enemies.json5
    pub(super) enemies_json5_path: PathBuf,

    /// Min atk to list
    #[arg(long)]
    pub(super) min: Option<usize>,

    /// Max atk to list
    #[arg(long)]
    pub(super) max: Option<usize>,

    /// Output in JSON format
    #[arg(long)]
    pub(super) json: bool,

    /// Pretty print JSON output
    #[arg(long, requires = "json")]
    pub(super) pretty: bool,
}
