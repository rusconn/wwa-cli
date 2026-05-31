mod args;
mod writers;

use std::{fs, io};

use anyhow::{Context, Result};

use wwa::{BreakpointOptions as Options, EnemiesBreakpointExt, Enemy};

pub(crate) use args::Args;

pub fn breakpoints(args: Args) -> Result<()> {
    let content = fs::read_to_string(&args.enemies_json5_path)
        .with_context(|| format!("failed to read {}", args.enemies_json5_path.display()))?;

    let enemies = json5::from_str::<Vec<Enemy>>(&content)
        .with_context(|| format!("failed to parse {}", args.enemies_json5_path.display()))?;

    let options = Options::new(args.min, args.max);

    let map = enemies.breakpoints(&options);

    let stdout = io::stdout().lock();
    let mut stdout = io::BufWriter::new(stdout);
    args.format
        .writeln(&mut stdout, &map)
        .context("failed to write output")
}
