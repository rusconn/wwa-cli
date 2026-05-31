use std::num::NonZeroUsize;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, PartialEq)]
pub struct Enemy {
    pub name: String,
    pub hp: NonZeroUsize,
    pub def: usize,
}
