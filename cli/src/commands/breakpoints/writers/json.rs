use std::{collections::BTreeMap, io::Write};

use wwa::{Breakpoint, Enemy};

use crate::BreakpointsError;

pub(in super::super) fn write(
    w: &mut impl Write,
    map: &BTreeMap<Breakpoint, Vec<&Enemy>>,
    pretty: bool,
) -> Result<(), BreakpointsError> {
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
