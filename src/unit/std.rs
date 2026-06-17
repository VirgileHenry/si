/// Shortcut to write use the `Unitless` value as a unit.
pub type Unitless = crate::CompoundUnit<
    crate::dim::std::Dimensionless,
    crate::unit::time::Second,
    crate::unit::length::Metre,
    crate::unit::mass::Kilogram,
    crate::unit::electric_current::Ampere,
    crate::unit::thermodynamic_temperature::Kelvin,
    crate::unit::amount_of_substance::Mole,
    crate::unit::luminous_intensity::Candela,
>;

/// Shortcut to write use the `Second` (s) as a unit.
pub type Second = crate::CompoundUnit<
    crate::dim::std::Time,
    crate::unit::time::Second,
    crate::unit::length::Metre,
    crate::unit::mass::Kilogram,
    crate::unit::electric_current::Ampere,
    crate::unit::thermodynamic_temperature::Kelvin,
    crate::unit::amount_of_substance::Mole,
    crate::unit::luminous_intensity::Candela,
>;

/// Shortcut to write use the `Metre` (m) as a unit.
pub type Metre = crate::CompoundUnit<
    crate::dim::std::Length,
    crate::unit::time::Second,
    crate::unit::length::Metre,
    crate::unit::mass::Kilogram,
    crate::unit::electric_current::Ampere,
    crate::unit::thermodynamic_temperature::Kelvin,
    crate::unit::amount_of_substance::Mole,
    crate::unit::luminous_intensity::Candela,
>;

/// Shortcut to write use the `Kilogram` (kg) as a unit.
pub type Kilogram = crate::CompoundUnit<
    crate::dim::std::Mass,
    crate::unit::time::Second,
    crate::unit::length::Metre,
    crate::unit::mass::Kilogram,
    crate::unit::electric_current::Ampere,
    crate::unit::thermodynamic_temperature::Kelvin,
    crate::unit::amount_of_substance::Mole,
    crate::unit::luminous_intensity::Candela,
>;

/// Shortcut to write use the `Ampere` (A) as a unit.
pub type Ampere = crate::CompoundUnit<
    crate::dim::std::ElectricCurrent,
    crate::unit::time::Second,
    crate::unit::length::Metre,
    crate::unit::mass::Kilogram,
    crate::unit::electric_current::Ampere,
    crate::unit::thermodynamic_temperature::Kelvin,
    crate::unit::amount_of_substance::Mole,
    crate::unit::luminous_intensity::Candela,
>;

/// Shortcut to write use the `Kelvin` (K) as a unit.
pub type Kelvin = crate::CompoundUnit<
    crate::dim::std::ThermodynamicTemperature,
    crate::unit::time::Second,
    crate::unit::length::Metre,
    crate::unit::mass::Kilogram,
    crate::unit::electric_current::Ampere,
    crate::unit::thermodynamic_temperature::Kelvin,
    crate::unit::amount_of_substance::Mole,
    crate::unit::luminous_intensity::Candela,
>;

/// Shortcut to write use the `Mole` (mol) as a unit.
pub type Mole = crate::CompoundUnit<
    crate::dim::std::AmountOfSubstance,
    crate::unit::time::Second,
    crate::unit::length::Metre,
    crate::unit::mass::Kilogram,
    crate::unit::electric_current::Ampere,
    crate::unit::thermodynamic_temperature::Kelvin,
    crate::unit::amount_of_substance::Mole,
    crate::unit::luminous_intensity::Candela,
>;

/// Shortcut to write use the `Candela` (cd) as a unit.
pub type Candela = crate::CompoundUnit<
    crate::dim::std::LuminousIntensity,
    crate::unit::time::Second,
    crate::unit::length::Metre,
    crate::unit::mass::Kilogram,
    crate::unit::electric_current::Ampere,
    crate::unit::thermodynamic_temperature::Kelvin,
    crate::unit::amount_of_substance::Mole,
    crate::unit::luminous_intensity::Candela,
>;
