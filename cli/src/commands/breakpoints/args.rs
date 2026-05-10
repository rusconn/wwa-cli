use std::path::PathBuf;

use clap::{Parser, ValueEnum};

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

    /// Output format
    #[arg(long, value_enum, default_value_t)]
    pub(super) format: Format,
}

#[derive(Debug, PartialEq, Eq, Clone, Default, ValueEnum)]
pub(super) enum Format {
    #[default]
    Plain,
    Json,
    JsonPretty,
}
