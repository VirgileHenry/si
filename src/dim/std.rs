use typenum::P1;
use typenum::Z0;

pub type Dimensionless = crate::CompoundDimension<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Time = crate::CompoundDimension<P1, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Length = crate::CompoundDimension<Z0, P1, Z0, Z0, Z0, Z0, Z0>;
pub type Mass = crate::CompoundDimension<Z0, Z0, P1, Z0, Z0, Z0, Z0>;
pub type ElectricCurrent = crate::CompoundDimension<Z0, Z0, Z0, P1, Z0, Z0, Z0>;
pub type ThermodynamicTemperature = crate::CompoundDimension<Z0, Z0, Z0, Z0, P1, Z0, Z0>;
pub type AmountOfSubstance = crate::CompoundDimension<Z0, Z0, Z0, Z0, Z0, P1, Z0>;
pub type LuminousIntensity = crate::CompoundDimension<Z0, Z0, Z0, Z0, Z0, Z0, P1>;
