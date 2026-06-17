pub trait InvDim {
    type Output: crate::Dimension;
}

impl<D: crate::Dimension> InvDim for D
where
    D::Time: core::ops::Neg,
    <D::Time as core::ops::Neg>::Output: typenum::Integer,
    D::Length: core::ops::Neg,
    <D::Length as core::ops::Neg>::Output: typenum::Integer,
    D::Mass: core::ops::Neg,
    <D::Mass as core::ops::Neg>::Output: typenum::Integer,
    D::ElectricCurrent: core::ops::Neg,
    <D::ElectricCurrent as core::ops::Neg>::Output: typenum::Integer,
    D::ThermodynamicTemperature: core::ops::Neg,
    <D::ThermodynamicTemperature as core::ops::Neg>::Output: typenum::Integer,
    D::AmountOfSubstance: core::ops::Neg,
    <D::AmountOfSubstance as core::ops::Neg>::Output: typenum::Integer,
    D::LuminousIntensity: core::ops::Neg,
    <D::LuminousIntensity as core::ops::Neg>::Output: typenum::Integer,
{
    type Output = crate::CompoundDimension<
        typenum::Negate<D::Time>,
        typenum::Negate<D::Length>,
        typenum::Negate<D::Mass>,
        typenum::Negate<D::ElectricCurrent>,
        typenum::Negate<D::ThermodynamicTemperature>,
        typenum::Negate<D::AmountOfSubstance>,
        typenum::Negate<D::LuminousIntensity>,
    >;
}

pub type InvertDim<D> = <D as InvDim>::Output;

pub trait MulDim<D: crate::Dimension> {
    type Output: crate::Dimension;
}

impl<D1: crate::Dimension, D2: crate::Dimension> MulDim<D2> for D1
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
    type Output = crate::CompoundDimension<
        typenum::Sum<D1::Time, D2::Time>,
        typenum::Sum<D1::Length, D2::Length>,
        typenum::Sum<D1::Mass, D2::Mass>,
        typenum::Sum<D1::ElectricCurrent, D2::ElectricCurrent>,
        typenum::Sum<D1::ThermodynamicTemperature, D2::ThermodynamicTemperature>,
        typenum::Sum<D1::AmountOfSubstance, D2::AmountOfSubstance>,
        typenum::Sum<D1::LuminousIntensity, D2::LuminousIntensity>,
    >;
}

pub type MultiplyDim<D1, D2> = <D1 as MulDim<D2>>::Output;

pub type DivideDim<D1, D2> = MultiplyDim<D1, InvertDim<D2>>;
