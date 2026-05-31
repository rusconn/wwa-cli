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

    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    #[once]
    fn goblin() -> Enemy {
        Enemy {
            name: "goblin".to_string(),
            hp: NonZeroUsize::new(10).unwrap(),
            def: 5,
        }
    }

    #[fixture]
    #[once]
    fn slime() -> Enemy {
        Enemy {
            name: "slime".to_string(),
            hp: NonZeroUsize::new(5).unwrap(),
            def: 2,
        }
    }

    #[fixture]
    #[once]
    fn map(
        goblin: &'static Enemy,
        slime: &'static Enemy,
    ) -> BTreeMap<Breakpoint, Vec<&'static Enemy>> {
        let mut map = BTreeMap::new();
        map.insert(10, vec![goblin]);
        map.insert(15, vec![goblin, slime]);
        map
    }

    #[rstest]
    #[case(Format::Plain, "10: goblin\n15: goblin,slime")]
    #[case(Format::Json, r#"{"10":["goblin"],"15":["goblin","slime"]}"#)]
    fn test_format_writers(
        map: &BTreeMap<Breakpoint, Vec<&'static Enemy>>,
        #[case] format: Format,
        #[case] expect: &str,
    ) {
        let mut buf = Vec::new();
        format.write(&mut buf, map).unwrap();

        let output = String::from_utf8(buf).unwrap();
        assert_eq!(output, *expect);
    }
}
