use std::{collections::BTreeMap, io::Write};

use wwa::{Breakpoint, Enemy};

use crate::BreakpointsError;

pub(in super::super) fn write(
    w: &mut impl Write,
    map: &BTreeMap<Breakpoint, Vec<&Enemy>>,
    pretty: bool,
) -> Result<(), BreakpointsError> {
    let map = MapWrapper(map);

    if pretty {
        serde_json::to_writer_pretty(w, &map)?;
    } else {
        serde_json::to_writer(w, &map)?;
    }

    Ok(())
}

use serde::{Serialize, Serializer};

struct MapWrapper<'a>(&'a BTreeMap<Breakpoint, Vec<&'a Enemy>>);

impl Serialize for MapWrapper<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_map(
            self.0
                .iter()
                .map(|(breakpoint, enemies)| (breakpoint, SeqWrapper(enemies))),
        )
    }
}

struct SeqWrapper<'a>(&'a Vec<&'a Enemy>);

impl Serialize for SeqWrapper<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.0.iter().map(|enemy| &enemy.name))
    }
}
