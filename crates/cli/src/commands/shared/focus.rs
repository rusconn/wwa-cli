use strum::VariantArray;

#[derive(Debug, Clone, Copy, PartialEq, VariantArray)]
pub(crate) enum Focus {
    Params,
    Output,
}

impl Focus {
    pub(crate) fn cycle(&self, delta: isize) -> Self {
        let n = Self::VARIANTS.len() as isize;
        Self::VARIANTS[(*self as isize + delta).rem_euclid(n) as usize]
    }
}
