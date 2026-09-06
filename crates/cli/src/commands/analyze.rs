mod args;
mod writers;

use std::{fs, io};

use anyhow::{Context, Result};

use wwa::{Enemy, Player};

pub(crate) use args::Args;

pub fn analyze(args: Args) -> Result<()> {
    let content = fs::read_to_string(&args.enemies_json5_path)
        .with_context(|| format!("failed to read {}", args.enemies_json5_path.display()))?;

    let enemies = json5::from_str::<Vec<Enemy>>(&content)
        .with_context(|| format!("failed to parse {}", args.enemies_json5_path.display()))?;

    let player = Player::new(args.atk, args.def);

    let analyses = enemies
        .iter()
        .map(|enemy| player.analyze(enemy))
        .collect::<Vec<_>>();

    let stdout = io::stdout().lock();
    let mut stdout = io::BufWriter::new(stdout);
    args.format
        .writeln(&mut stdout, &analyses, &enemies)
        .context("failed to write output")
}
