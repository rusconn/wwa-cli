use strum::VariantArray;

#[derive(Debug, Clone, Copy, PartialEq, VariantArray)]
pub(super) enum Focus {
    Params,
    Output,
}

impl Focus {
    pub(super) fn cycle(&self, delta: isize) -> Self {
        let n = Self::VARIANTS.len() as isize;
        Self::VARIANTS[(*self as isize + delta).rem_euclid(n) as usize]
    }
}
