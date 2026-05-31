#[derive(Debug, Default)]
pub struct Options {
    pub(super) min: Option<usize>,
    pub(super) max: Option<usize>,
}

impl Options {
    pub fn new(min: Option<usize>, max: Option<usize>) -> Self {
        Self { min, max }
    }
}
