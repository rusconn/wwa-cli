mod json;
mod plain;

use std::{collections::BTreeMap, io::Write};

use wwa::{Breakpoint, Enemy};

use super::{Error, args::Format};

impl Format {
    pub(super) fn write(
        &self,
        w: &mut impl Write,
        map: &BTreeMap<Breakpoint, Vec<&Enemy>>,
    ) -> Result<(), Error> {
        match self {
            Format::Plain => plain::write(w, map),
            Format::Json => json::write(w, map, false),
            Format::JsonPretty => json::write(w, map, true),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    use super::*;

    #[test]
    fn test_writers() {
        let mut map = BTreeMap::new();
        let goblin = Enemy {
            name: "goblin".to_string(),
            hp: NonZeroUsize::new(10).unwrap(),
            def: 5,
        };
        let slime = Enemy {
            name: "slime".to_string(),
            hp: NonZeroUsize::new(5).unwrap(),
            def: 2,
        };
        map.insert(10, vec![&goblin]);
        map.insert(15, vec![&goblin, &slime]);

        let format_cases: [(Format, &str); _] = [
            (Format::Plain, "10: goblin\n15: goblin,slime"),
            (Format::Json, r#"{"10":["goblin"],"15":["goblin","slime"]}"#),
        ];

        for (writer, expect) in format_cases {
            let mut buf = Vec::new();
            writer.write(&mut buf, &map).unwrap();

            let output = String::from_utf8(buf).unwrap();
            assert_eq!(output, *expect);
        }
    }
}
