use std::{collections::BTreeMap, io::Write};

use wwa::{Breakpoint, Enemy};

use crate::BreakpointsError;

pub(in super::super) fn write(
    w: &mut impl Write,
    map: &BTreeMap<Breakpoint, Vec<&Enemy>>,
) -> Result<(), BreakpointsError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroUsize;

    #[test]
    fn test_plain_writer() {
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
        write(&mut buf, &map).unwrap();

        let output = String::from_utf8(buf).unwrap();
        assert_eq!(output, "10: goblin\n15: goblin,slime");
    }
}
