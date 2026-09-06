use std::io::Write;

use anyhow::{Context, Result};

use wwa::{Analysis, Enemy};

pub(in super::super) fn write(
    w: &mut impl Write,
    analyses: &[Analysis],
    enemies: &[Enemy],
    pretty: bool,
) -> Result<()> {
    let items: Vec<Item> = enemies
        .iter()
        .zip(analyses)
        .filter_map(|(enemy, analysis)| match analysis {
            Analysis::NoDamage => None,
            a => Some(Item::make(enemy, a)),
        })
        .collect();

    if pretty {
        serde_json::to_writer_pretty(w, &items)
    } else {
        serde_json::to_writer(w, &items)
    }
    .context("failed to serialize to JSON")
}

struct Item<'a> {
    name: &'a str,
    kind: &'static str,
    atk_inc: usize,
    damage: Option<Damage>,
}

struct Damage {
    atk_reduction: usize,
    def_reduction: usize,
}

impl<'a> Item<'a> {
    fn make(enemy: &'a Enemy, analysis: &Analysis) -> Self {
        match analysis {
            Analysis::Blocked { atk_inc } => Self {
                name: &enemy.name,
                kind: "blocked",
                atk_inc: *atk_inc,
                damage: None,
            },
            Analysis::Damage {
                atk_inc,
                atk_reduction,
                def_reduction,
            } => Self {
                name: &enemy.name,
                kind: "damage",
                atk_inc: *atk_inc,
                damage: Some(Damage {
                    atk_reduction: *atk_reduction,
                    def_reduction: *def_reduction,
                }),
            },
            Analysis::NoDamage => {
                unreachable!("NoDamage is filtered out by the caller");
            }
        }
    }
}

use serde::{Serialize, Serializer, ser::SerializeStruct};

impl Serialize for Item<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Item", 5)?;
        s.serialize_field("name", self.name)?;
        s.serialize_field("kind", self.kind)?;
        s.serialize_field("atk_inc", &self.atk_inc)?;
        if let Some(damage) = &self.damage {
            s.serialize_field("atk_reduction", &damage.atk_reduction)?;
            s.serialize_field("def_reduction", &damage.def_reduction)?;
        }
        s.end()
    }
}
