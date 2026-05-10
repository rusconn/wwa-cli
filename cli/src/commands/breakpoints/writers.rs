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
