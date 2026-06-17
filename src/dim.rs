pub mod ops;
pub mod std;

pub trait Dimension {
    type Time: typenum::Integer;
    type Length: typenum::Integer;
    type Mass: typenum::Integer;
    type ElectricCurrent: typenum::Integer;
    type ThermodynamicTemperature: typenum::Integer;
    type AmountOfSubstance: typenum::Integer;
    type LuminousIntensity: typenum::Integer;

    /// Write the dimension representation into the provided buffer.
    ///
    /// This is basically a [`core::fmt::Display`] impl, without an instance required.
    ///
    /// Fixme: with const generics stabilized, we can generate a `const &'static str`
    /// for each dimension representation at compile time, which allows to remove this runtime logic.
    fn write(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompoundDimension<Time, Length, Mass, ElectricCurrent, ThermodynamicTemperature, AmountOfSubstance, LuminousIntensity>
where
    Time: typenum::Integer,
    Length: typenum::Integer,
    Mass: typenum::Integer,
    ElectricCurrent: typenum::Integer,
    ThermodynamicTemperature: typenum::Integer,
    AmountOfSubstance: typenum::Integer,
    LuminousIntensity: typenum::Integer,
{
    _m: core::marker::PhantomData<(
        Time,
        Length,
        Mass,
        ElectricCurrent,
        ThermodynamicTemperature,
        AmountOfSubstance,
        LuminousIntensity,
    )>,
}

impl<Time, Length, Mass, ElectricCurrent, ThermodynamicTemperature, AmountOfSubstance, LuminousIntensity>
    CompoundDimension<Time, Length, Mass, ElectricCurrent, ThermodynamicTemperature, AmountOfSubstance, LuminousIntensity>
where
    Time: typenum::Integer,
    Length: typenum::Integer,
    Mass: typenum::Integer,
    ElectricCurrent: typenum::Integer,
    ThermodynamicTemperature: typenum::Integer,
    AmountOfSubstance: typenum::Integer,
    LuminousIntensity: typenum::Integer,
{
    pub const fn new() -> Self {
        Self {
            _m: core::marker::PhantomData,
        }
    }
}

impl<Time, Length, Mass, ElectricCurrent, ThermodynamicTemperature, AmountOfSubstance, LuminousIntensity> Dimension
    for CompoundDimension<Time, Length, Mass, ElectricCurrent, ThermodynamicTemperature, AmountOfSubstance, LuminousIntensity>
where
    Time: typenum::Integer,
    Length: typenum::Integer,
    Mass: typenum::Integer,
    ElectricCurrent: typenum::Integer,
    ThermodynamicTemperature: typenum::Integer,
    AmountOfSubstance: typenum::Integer,
    LuminousIntensity: typenum::Integer,
{
    type Time = Time;
    type Length = Length;
    type Mass = Mass;
    type ElectricCurrent = ElectricCurrent;
    type ThermodynamicTemperature = ThermodynamicTemperature;
    type AmountOfSubstance = AmountOfSubstance;
    type LuminousIntensity = LuminousIntensity;

    fn write(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut dimensionless = true;

        fn write_dim<Power: typenum::Integer>(
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
                write!(f, "{symbol}")?;
                if Power::I32 != 1 {
                    write!(f, "^{}", Power::I32)?;
                }
            }
            Ok(())
        }

        /* Write positive exponent first */
        write_dim::<Self::Time>(f, "T", &mut dimensionless, true)?;
        write_dim::<Self::Length>(f, "L", &mut dimensionless, true)?;
        write_dim::<Self::Mass>(f, "M", &mut dimensionless, true)?;
        write_dim::<Self::ElectricCurrent>(f, "I", &mut dimensionless, true)?;
        write_dim::<Self::ThermodynamicTemperature>(f, "Θ", &mut dimensionless, true)?;
        write_dim::<Self::AmountOfSubstance>(f, "N", &mut dimensionless, true)?;
        write_dim::<Self::LuminousIntensity>(f, "J", &mut dimensionless, true)?;

        /* Write negative exponent second */
        write_dim::<Self::Time>(f, "T", &mut dimensionless, false)?;
        write_dim::<Self::Length>(f, "L", &mut dimensionless, false)?;
        write_dim::<Self::Mass>(f, "M", &mut dimensionless, false)?;
        write_dim::<Self::ElectricCurrent>(f, "I", &mut dimensionless, false)?;
        write_dim::<Self::ThermodynamicTemperature>(f, "Θ", &mut dimensionless, false)?;
        write_dim::<Self::AmountOfSubstance>(f, "N", &mut dimensionless, false)?;
        write_dim::<Self::LuminousIntensity>(f, "J", &mut dimensionless, false)?;

        Ok(())
    }
}
