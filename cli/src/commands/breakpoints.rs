mod args;
mod error;

use std::{collections::BTreeMap, fmt::Write, fs};

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

    println!("{}", render(&map, args.json, args.pretty)?);

    Ok(())
}

fn render(
    map: &BTreeMap<Breakpoint, Vec<&Enemy>>,
    json: bool,
    pretty: bool,
) -> Result<String, Error> {
    if json {
        render_as_json(map, pretty)
    } else {
        render_as_plain(map)
    }
}

fn render_as_json(map: &BTreeMap<Breakpoint, Vec<&Enemy>>, pretty: bool) -> Result<String, Error> {
    let map = map
        .iter()
        .map(|(bp, enemies)| (bp, enemies.iter().map(|enemy| &enemy.name).collect()))
        .collect::<BTreeMap<&Breakpoint, Vec<&String>>>();

    let s = if pretty {
        serde_json::to_string_pretty(&map)?
    } else {
        serde_json::to_string(&map)?
    };

    Ok(s)
}

fn render_as_plain(map: &BTreeMap<Breakpoint, Vec<&Enemy>>) -> Result<String, Error> {
    let mut s = String::with_capacity(map.len() * 32);

    for (i, (breakpoint, enemies)) in map.iter().enumerate() {
        if i > 0 {
            s.push('\n');
        }
        write!(s, "{breakpoint}: ")?;
        for (j, enemy) in enemies.iter().enumerate() {
            if j > 0 {
                s.push(',');
            }
            s.push_str(&enemy.name);
        }
    }

    Ok(s)
}
