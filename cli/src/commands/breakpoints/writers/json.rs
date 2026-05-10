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

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroUsize;

    #[test]
    fn test_json_writer() {
        let mut map = BTreeMap::new();
        let e1 = Enemy {
            name: "goblin".to_string(),
            hp: NonZeroUsize::new(10).unwrap(),
            def: 5,
        };
        let e2 = Enemy {
            name: "slime".to_string(),
            hp: NonZeroUsize::new(5).unwrap(),
            def: 2,
        };
        map.insert(10, vec![&e1]);
        map.insert(15, vec![&e1, &e2]);

        let mut buf = Vec::new();
        write(&mut buf, &map, false).unwrap();

        let output = String::from_utf8(buf).unwrap();
        assert_eq!(output, r#"{"10":["goblin"],"15":["goblin","slime"]}"#);
    }
}
