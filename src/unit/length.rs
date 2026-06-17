pub trait LengthUnit {
    const SYMBOL: &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Metre;

impl LengthUnit for Metre {
    const SYMBOL: &'static str = "m";
}
