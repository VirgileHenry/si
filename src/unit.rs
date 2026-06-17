pub mod ops;
pub mod std;

mod amount_of_substance;
mod electric_current;
mod length;
mod luminous_intensity;
mod mass;
mod thermodynamic_temperature;
mod time;

pub trait Unit {
    /// Associated dimension with this unit.
    ///
    /// This associated type holds which power is used for each unit.
    type Dimension: crate::Dimension;

    type Time: time::TimeUnit;
    type Length: length::LengthUnit;
    type Mass: mass::MassUnit;
    type ElectricCurrent: electric_current::ElectricCurrentUnit;
    type ThermodynamicTemperature: thermodynamic_temperature::ThermodynamicTemperatureUnit;
    type AmountOfSubstance: amount_of_substance::AmountOfSubstanceUnit;
    type LuminousIntensity: luminous_intensity::LuminousIntensityUnit;

    /// Write the unit representation into the provided buffer.
    ///
    /// This is basically a [`core::fmt::Display`] impl, without an instance required.
    ///
    /// Fixme: with const generics stabilized, we can generate a `const &'static str`
    /// for each unit representation at compile time, which allows to remove this runtime logic.
    fn write(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompoundUnit<
    Dimension,
    Time,
    Length,
    Mass,
    ElectricCurrent,
    ThermodynamicTemperature,
    AmountOfSubstance,
    LuminousIntensity,
> where
    Dimension: crate::Dimension,
    Time: time::TimeUnit,
    Length: length::LengthUnit,
    Mass: mass::MassUnit,
    ElectricCurrent: electric_current::ElectricCurrentUnit,
    ThermodynamicTemperature: thermodynamic_temperature::ThermodynamicTemperatureUnit,
    AmountOfSubstance: amount_of_substance::AmountOfSubstanceUnit,
    LuminousIntensity: luminous_intensity::LuminousIntensityUnit,
{
    _m: core::marker::PhantomData<(
        Dimension,
        Time,
        Length,
        Mass,
        ElectricCurrent,
        ThermodynamicTemperature,
        AmountOfSubstance,
        LuminousIntensity,
    )>,
}

impl<
    Dimension: crate::Dimension,
    Time,
    Length,
    Mass,
    ElectricCurrent,
    ThermodynamicTemperature,
    AmountOfSubstance,
    LuminousIntensity,
> Unit
    for CompoundUnit<
        Dimension,
        Time,
        Length,
        Mass,
        ElectricCurrent,
        ThermodynamicTemperature,
        AmountOfSubstance,
        LuminousIntensity,
    >
where
    Dimension: crate::Dimension,
    Time: time::TimeUnit,
    Length: length::LengthUnit,
    Mass: mass::MassUnit,
    ElectricCurrent: electric_current::ElectricCurrentUnit,
    ThermodynamicTemperature: thermodynamic_temperature::ThermodynamicTemperatureUnit,
    AmountOfSubstance: amount_of_substance::AmountOfSubstanceUnit,
    LuminousIntensity: luminous_intensity::LuminousIntensityUnit,
{
    type Dimension = Dimension;

    type Time = Time;
    type Length = Length;
    type Mass = Mass;
    type ElectricCurrent = ElectricCurrent;
    type ThermodynamicTemperature = ThermodynamicTemperature;
    type AmountOfSubstance = AmountOfSubstance;
    type LuminousIntensity = LuminousIntensity;

    fn write(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut dimensionless = true;

        fn write_unit<Power: typenum::Integer>(
            f: &mut core::fmt::Formatter<'_>,
            symbol: &str,
            dimensionless: &mut bool,
            write_positive: bool,
        ) -> core::fmt::Result {
            if (Power::I32 > 0 && write_positive) || (Power::I32 < 0 && !write_positive) {
                if !*dimensionless {
                    write!(f, "·")?;
                }
                *dimensionless = false;
                write!(f, "{}", symbol)?;
                if Power::I32 != 1 {
                    write!(f, "^{}", Power::I32)?;
                }
            }
            Ok(())
        }

        /* Write positive exponent first */
        write_unit::<Dimension::Time>(f, Self::Time::SYMBOL, &mut dimensionless, true)?;
        write_unit::<Dimension::Length>(f, Self::Length::SYMBOL, &mut dimensionless, true)?;
        write_unit::<Dimension::Mass>(f, Self::Mass::SYMBOL, &mut dimensionless, true)?;
        write_unit::<Dimension::ElectricCurrent>(f, Self::ElectricCurrent::SYMBOL, &mut dimensionless, true)?;
        write_unit::<Dimension::ThermodynamicTemperature>(f, Self::ThermodynamicTemperature::SYMBOL, &mut dimensionless, true)?;
        write_unit::<Dimension::AmountOfSubstance>(f, Self::AmountOfSubstance::SYMBOL, &mut dimensionless, true)?;
        write_unit::<Dimension::LuminousIntensity>(f, Self::LuminousIntensity::SYMBOL, &mut dimensionless, true)?;

        /* Write negative exponent second */
        write_unit::<Dimension::Time>(f, Self::Time::SYMBOL, &mut dimensionless, false)?;
        write_unit::<Dimension::Length>(f, Self::Length::SYMBOL, &mut dimensionless, false)?;
        write_unit::<Dimension::Mass>(f, Self::Mass::SYMBOL, &mut dimensionless, false)?;
        write_unit::<Dimension::ElectricCurrent>(f, Self::ElectricCurrent::SYMBOL, &mut dimensionless, false)?;
        write_unit::<Dimension::ThermodynamicTemperature>(f, Self::ThermodynamicTemperature::SYMBOL, &mut dimensionless, false)?;
        write_unit::<Dimension::AmountOfSubstance>(f, Self::AmountOfSubstance::SYMBOL, &mut dimensionless, false)?;
        write_unit::<Dimension::LuminousIntensity>(f, Self::LuminousIntensity::SYMBOL, &mut dimensionless, false)?;

        Ok(())
    }
}
