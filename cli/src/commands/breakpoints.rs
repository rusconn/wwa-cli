mod args;
mod error;

use std::{collections::BTreeMap, fs};

use itertools::Itertools;

use wwa::{Breakpoint, BreakpointOptions as Options, Enemy, breakpoint_map};

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

    let map = breakpoint_map(&enemies, &options);

    println!("{}", render(&map, args.json));

    Ok(())
}

fn render(map: &BTreeMap<Breakpoint, Vec<&Enemy>>, json: bool) -> String {
    if json {
        render_as_json(map)
    } else {
        render_as_plain(map)
    }
}

fn render_as_json(map: &BTreeMap<Breakpoint, Vec<&Enemy>>) -> String {
    let map = map
        .iter()
        .map(|(bp, enemies)| (bp, enemies.iter().map(|enemy| &enemy.name).collect()))
        .collect::<BTreeMap<&Breakpoint, Vec<&String>>>();

    serde_json::to_string(&map).unwrap()
}

fn render_as_plain(map: &BTreeMap<Breakpoint, Vec<&Enemy>>) -> String {
    map.iter()
        .map(|(breakpoint, enemies)| render_as_plain_one(breakpoint, enemies))
        .join("\n")
}

fn render_as_plain_one(breakpoint: &Breakpoint, enemies: &[&Enemy]) -> String {
    format!(
        "{breakpoint}: {}",
        enemies.iter().map(|enemy| &enemy.name).join(",")
    )
}
