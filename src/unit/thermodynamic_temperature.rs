pub trait ThermodynamicTemperatureUnit {
    const SYMBOL: &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Kelvin;

impl ThermodynamicTemperatureUnit for Kelvin {
    const SYMBOL: &'static str = "K";
}
