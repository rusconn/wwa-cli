mod json;
mod plain;

use std::io::Write;

use anyhow::Result;

use wwa::{Analysis, Enemy};

use super::args::Format;

impl Format {
    pub(super) fn writeln(
        &self,
        w: &mut impl Write,
        analyses: &[Analysis],
        enemies: &[Enemy],
    ) -> Result<()> {
        match self {
            Format::Plain => plain::write(w, analyses, enemies),
            Format::Json => json::write(w, analyses, enemies, false),
            Format::JsonPretty => json::write(w, analyses, enemies, true),
        }?;
        writeln!(w)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn enemies() -> Vec<Enemy> {
        vec![
            Enemy {
                name: "goblin".to_string(),
                hp: NonZeroUsize::new(30).unwrap(),
                atk: 10,
                def: 1,
            },
            Enemy {
                name: "wolf".to_string(),
                hp: NonZeroUsize::new(25).unwrap(),
                atk: 12,
                def: 3,
            },
            Enemy {
                name: "slime".to_string(),
                hp: NonZeroUsize::new(5).unwrap(),
                atk: 10,
                def: 1,
            },
            Enemy {
                name: "golem".to_string(),
                hp: NonZeroUsize::new(22).unwrap(),
                atk: 15,
                def: 22,
            },
        ]
    }

    #[fixture]
    fn analyses() -> Vec<Analysis> {
        vec![
            Analysis::Damage {
                atk_inc: 5,
                atk_reduction: 5,
                def_reduction: 2,
            },
            Analysis::Damage {
                atk_inc: 1,
                atk_reduction: 7,
                def_reduction: 3,
            },
            Analysis::NoDamage,
            Analysis::Blocked { atk_inc: 12 },
        ]
    }

    #[rstest]
    #[case(
        Format::Plain,
        concat!(
            "goblin\n",
            "  †+5 ⇒ -5\n",
            "  ⛨+1 ⇒ -2\n",
            "wolf\n",
            "  †+1 ⇒ -7\n",
            "  ⛨+1 ⇒ -3\n",
            "golem\n",
            "  †+12 ⇒ ok\n",
        ),
    )]
    #[case(
        Format::Json,
        concat!(
            r#"[{"name":"goblin","kind":"damage","atk_inc":5,"atk_reduction":5,"def_reduction":2},"#,
            r#"{"name":"wolf","kind":"damage","atk_inc":1,"atk_reduction":7,"def_reduction":3},"#,
            r#"{"name":"golem","kind":"blocked","atk_inc":12}]"#,
            "\n",
        ),
    )]
    fn test_format_writers(
        enemies: Vec<Enemy>,
        analyses: Vec<Analysis>,
        #[case] format: Format,
        #[case] expect: &str,
    ) {
        let mut buf = Vec::new();
        format.writeln(&mut buf, &analyses, &enemies).unwrap();

        let output = String::from_utf8(buf).unwrap();
        assert_eq!(output, *expect);
    }
}
