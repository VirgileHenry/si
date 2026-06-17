pub trait InvUnit {
    type Output: crate::Unit;
}

impl<U: crate::Unit> InvUnit for U
where
    <U::Dimension as crate::Dimension>::Time: core::ops::Neg,
    <<U::Dimension as crate::Dimension>::Time as core::ops::Neg>::Output: typenum::Integer,
    <U::Dimension as crate::Dimension>::Length: core::ops::Neg,
    <<U::Dimension as crate::Dimension>::Length as core::ops::Neg>::Output: typenum::Integer,
    <U::Dimension as crate::Dimension>::Mass: core::ops::Neg,
    <<U::Dimension as crate::Dimension>::Mass as core::ops::Neg>::Output: typenum::Integer,
    <U::Dimension as crate::Dimension>::ElectricCurrent: core::ops::Neg,
    <<U::Dimension as crate::Dimension>::ElectricCurrent as core::ops::Neg>::Output: typenum::Integer,
    <U::Dimension as crate::Dimension>::ThermodynamicTemperature: core::ops::Neg,
    <<U::Dimension as crate::Dimension>::ThermodynamicTemperature as core::ops::Neg>::Output: typenum::Integer,
    <U::Dimension as crate::Dimension>::AmountOfSubstance: core::ops::Neg,
    <<U::Dimension as crate::Dimension>::AmountOfSubstance as core::ops::Neg>::Output: typenum::Integer,
    <U::Dimension as crate::Dimension>::LuminousIntensity: core::ops::Neg,
    <<U::Dimension as crate::Dimension>::LuminousIntensity as core::ops::Neg>::Output: typenum::Integer,
{
    type Output = crate::CompoundUnit<
        crate::dim::ops::InvertDim<U::Dimension>,
        U::Time,
        U::Length,
        U::Mass,
        U::ElectricCurrent,
        U::ThermodynamicTemperature,
        U::AmountOfSubstance,
        U::LuminousIntensity,
    >;
}

pub type InvertUnit<U> = <U as InvUnit>::Output;

pub trait MulUnit<U: crate::Unit> {
    type Output: crate::Unit;
}

impl<
    D1: crate::Dimension,
    D2: crate::Dimension,
    Time: crate::unit::time::TimeUnit,
    Length: crate::unit::length::LengthUnit,
    Mass: crate::unit::mass::MassUnit,
    ElectricCurrent: crate::unit::electric_current::ElectricCurrentUnit,
    ThermodynamicTemperature: crate::unit::thermodynamic_temperature::ThermodynamicTemperatureUnit,
    AmountOfSubstance: crate::unit::amount_of_substance::AmountOfSubstanceUnit,
    LuminousIntensity: crate::unit::luminous_intensity::LuminousIntensityUnit,
>
    MulUnit<
        crate::CompoundUnit<
            D2,
            Time,
            Length,
            Mass,
            ElectricCurrent,
            ThermodynamicTemperature,
            AmountOfSubstance,
            LuminousIntensity,
        >,
    >
    for crate::CompoundUnit<
        D1,
        Time,
        Length,
        Mass,
        ElectricCurrent,
        ThermodynamicTemperature,
        AmountOfSubstance,
        LuminousIntensity,
    >
where
    D1::Time: core::ops::Add<D2::Time>,
    <D1::Time as core::ops::Add<D2::Time>>::Output: typenum::Integer,
    D1::Length: core::ops::Add<D2::Length>,
    <D1::Length as core::ops::Add<D2::Length>>::Output: typenum::Integer,
    D1::Mass: core::ops::Add<D2::Mass>,
    <D1::Mass as core::ops::Add<D2::Mass>>::Output: typenum::Integer,
    D1::ElectricCurrent: core::ops::Add<D2::ElectricCurrent>,
    <D1::ElectricCurrent as core::ops::Add<D2::ElectricCurrent>>::Output: typenum::Integer,
    D1::ThermodynamicTemperature: core::ops::Add<D2::ThermodynamicTemperature>,
    <D1::ThermodynamicTemperature as core::ops::Add<D2::ThermodynamicTemperature>>::Output: typenum::Integer,
    D1::AmountOfSubstance: core::ops::Add<D2::AmountOfSubstance>,
    <D1::AmountOfSubstance as core::ops::Add<D2::AmountOfSubstance>>::Output: typenum::Integer,
    D1::LuminousIntensity: core::ops::Add<D2::LuminousIntensity>,
    <D1::LuminousIntensity as core::ops::Add<D2::LuminousIntensity>>::Output: typenum::Integer,
{
    type Output = crate::CompoundUnit<
        crate::dim::ops::MultiplyDim<D1, D2>,
        Time,
        Length,
        Mass,
        ElectricCurrent,
        ThermodynamicTemperature,
        AmountOfSubstance,
        LuminousIntensity,
    >;
}

pub type MultiplyUnit<U1, U2> = <U1 as MulUnit<U2>>::Output;

pub type DivideUnit<U1, U2> = MultiplyUnit<U1, InvertUnit<U2>>;
