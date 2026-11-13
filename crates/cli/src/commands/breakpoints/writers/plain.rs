use std::{collections::BTreeMap, io::Write};

use anyhow::Result;
use itertools::Itertools;

use wwa::{Breakpoint, Enemy};

pub(in super::super) fn write(
    w: &mut impl Write,
    map: &BTreeMap<Breakpoint, Vec<&Enemy>>,
) -> Result<()> {
    let lines = map.iter().format_with("\n", |(breakpoint, enemies), f| {
        let enemy_names = enemies.iter().format_with(",", |enemy, f| f(&enemy.name));
        f(&format_args!("{breakpoint}: {enemy_names}"))
    });

    write!(w, "{lines}")?;

    Ok(())
}
