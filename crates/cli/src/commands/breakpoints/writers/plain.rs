use std::{collections::BTreeMap, io::Write};

use anyhow::Result;

use wwa::{Breakpoint, Enemy};

pub(in super::super) fn write(
    w: &mut impl Write,
    map: &BTreeMap<Breakpoint, Vec<&Enemy>>,
) -> Result<()> {
    let mut iter = map.iter();

    if let Some((breakpoint, enemies)) = iter.next() {
        write!(w, "{breakpoint}: ")?;
        write_enemies(w, enemies)?;
        for (breakpoint, enemies) in iter {
            write!(w, "\n{breakpoint}: ")?;
            write_enemies(w, enemies)?;
        }
    }

    Ok(())
}

fn write_enemies(w: &mut impl Write, enemies: &[&Enemy]) -> Result<()> {
    let mut iter = enemies.iter();

    if let Some(enemy) = iter.next() {
        write!(w, "{}", enemy.name)?;
        for enemy in iter {
            write!(w, ",{}", enemy.name)?;
        }
    }

    Ok(())
}
