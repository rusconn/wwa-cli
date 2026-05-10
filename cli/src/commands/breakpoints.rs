mod args;
mod error;

use std::{
    collections::BTreeMap,
    fs,
    io::{self, Write},
};

use wwa::{Breakpoint, BreakpointOptions as Options, EnemiesBreakpointExt, Enemy};

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
    write(&mut stdout, &map, args.json, args.pretty)?;
    writeln!(stdout)?;

    Ok(())
}

fn write(
    w: &mut impl Write,
    map: &BTreeMap<Breakpoint, Vec<&Enemy>>,
    json: bool,
    pretty: bool,
) -> Result<(), Error> {
    if json {
        write_as_json(w, map, pretty)
    } else {
        write_as_plain(w, map)
    }
}

fn write_as_json(
    w: &mut impl Write,
    map: &BTreeMap<Breakpoint, Vec<&Enemy>>,
    pretty: bool,
) -> Result<(), Error> {
    let map = map
        .iter()
        .map(|(bp, enemies)| (bp, enemies.iter().map(|enemy| &enemy.name).collect()))
        .collect::<BTreeMap<&Breakpoint, Vec<&String>>>();

    if pretty {
        serde_json::to_writer_pretty(w, &map)?;
    } else {
        serde_json::to_writer(w, &map)?;
    }

    Ok(())
}

fn write_as_plain(
    w: &mut impl Write,
    map: &BTreeMap<Breakpoint, Vec<&Enemy>>,
) -> Result<(), Error> {
    for (i, (breakpoint, enemies)) in map.iter().enumerate() {
        if i > 0 {
            writeln!(w)?;
        }
        write!(w, "{breakpoint}: ")?;
        for (j, enemy) in enemies.iter().enumerate() {
            if j > 0 {
                write!(w, ",")?;
            }
            write!(w, "{}", enemy.name)?;
        }
    }

    Ok(())
}
