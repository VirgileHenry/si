pub trait LuminousIntensityUnit {
    const SYMBOL: &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Candela;

impl LuminousIntensityUnit for Candela {
    const SYMBOL: &'static str = "cd";
}
