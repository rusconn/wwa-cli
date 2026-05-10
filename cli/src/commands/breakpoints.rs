mod args;
mod error;
mod writers;

use std::{
    fs,
    io::{self, Write},
};

use wwa::{BreakpointOptions as Options, EnemiesBreakpointExt, Enemy};

pub(crate) use {args::Args, error::Error};

pub fn breakpoints(args: Args) -> Result<(), Error> {
    let content = fs::read_to_string(&args.enemies_json5_path) //
        .map_err(|source| Error::ReadEnemies {
            path: args.enemies_json5_path.clone(),
            source,
        })?;

    let enemies = json5::from_str::<Vec<Enemy>>(&content) //
        .map_err(|source| Error::ParseEnemies {
            path: args.enemies_json5_path.clone(),
            source,
        })?;

    let options = Options::new(args.min, args.max);

    let map = enemies.breakpoints(&options);

    let stdout = io::stdout().lock();
    let mut stdout = io::BufWriter::new(stdout);
    args.format.write(&mut stdout, &map)?;
    writeln!(stdout)?;

    Ok(())
}
