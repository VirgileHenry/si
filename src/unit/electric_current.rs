pub trait ElectricCurrentUnit {
    const SYMBOL: &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ampere;

impl ElectricCurrentUnit for Ampere {
    const SYMBOL: &'static str = "A";
}
