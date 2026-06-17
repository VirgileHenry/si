pub trait MassUnit {
    const SYMBOL: &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Kilogram;

impl MassUnit for Kilogram {
    const SYMBOL: &'static str = "kg";
}
