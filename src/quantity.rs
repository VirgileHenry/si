#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Quantity<T, U: crate::Unit> {
    _unit: core::marker::PhantomData<U>,
    value: T,
}

impl<T, U: crate::Unit> Quantity<T, U> {
    pub fn new(value: T) -> Self {
        Self {
            _unit: core::marker::PhantomData,
            value,
        }
    }
}

impl<T: core::fmt::Display, U: crate::Unit> core::fmt::Display for Quantity<T, U> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)?;
        U::write(f)?;
        Ok(())
    }
}

impl<T, U: crate::Unit> core::ops::Deref for Quantity<T, U> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T1, T2, U> core::ops::Add<Quantity<T2, U>> for Quantity<T1, U>
where
    U: crate::Unit,
    T1: core::ops::Add<T2>,
{
    type Output = Quantity<<T1 as core::ops::Add<T2>>::Output, U>;
    fn add(self, rhs: Quantity<T2, U>) -> Self::Output {
        Quantity {
            _unit: core::marker::PhantomData,
            value: self.value + rhs.value,
        }
    }
}

impl<T1, T2, U> core::ops::AddAssign<Quantity<T2, U>> for Quantity<T1, U>
where
    U: crate::Unit,
    T1: core::ops::AddAssign<T2>,
{
    fn add_assign(&mut self, rhs: Quantity<T2, U>) {
        self.value += rhs.value
    }
}

impl<T1, T2, U1, U2> core::ops::Mul<Quantity<T2, U2>> for Quantity<T1, U1>
where
    T1: core::ops::Mul<T2>,
    U1: crate::Unit,
    U2: crate::Unit,
    U1: crate::unit::ops::MulUnit<U2>,
    U1::Time: core::ops::Add<U2::Time>,
    <U1::Time as core::ops::Add<U2::Time>>::Output: typenum::Integer,
    U1::Length: core::ops::Add<U2::Length>,
    <U1::Length as core::ops::Add<U2::Length>>::Output: typenum::Integer,
    U1::Mass: core::ops::Add<U2::Mass>,
    <U1::Mass as core::ops::Add<U2::Mass>>::Output: typenum::Integer,
    U1::ElectricCurrent: core::ops::Add<U2::ElectricCurrent>,
    <U1::ElectricCurrent as core::ops::Add<U2::ElectricCurrent>>::Output: typenum::Integer,
    U1::ThermodynamicTemperature: core::ops::Add<U2::ThermodynamicTemperature>,
    <U1::ThermodynamicTemperature as core::ops::Add<U2::ThermodynamicTemperature>>::Output: typenum::Integer,
    U1::AmountOfSubstance: core::ops::Add<U2::AmountOfSubstance>,
    <U1::AmountOfSubstance as core::ops::Add<U2::AmountOfSubstance>>::Output: typenum::Integer,
    U1::LuminousIntensity: core::ops::Add<U2::LuminousIntensity>,
    <U1::LuminousIntensity as core::ops::Add<U2::LuminousIntensity>>::Output: typenum::Integer,
{
    type Output = Quantity<<T1 as core::ops::Mul<T2>>::Output, crate::unit::ops::MultiplyUnit<U1, U2>>;
    fn mul(self, rhs: Quantity<T2, U2>) -> Self::Output {
        Quantity {
            _unit: core::marker::PhantomData,
            value: self.value * rhs.value,
        }
    }
}

impl<T1, T2, U1, U2> core::ops::Div<Quantity<T2, U2>> for Quantity<T1, U1>
where
    T1: core::ops::Div<T2>,
    U1: crate::Unit,
    U2: crate::Unit,
    U1: crate::unit::ops::MulUnit<<U2 as crate::unit::ops::InvUnit>::Output>,
    <U2::Dimension as crate::Dimension>::Time: core::ops::Neg,
    <<U2::Dimension as crate::Dimension>::Time as core::ops::Neg>::Output: typenum::Integer,
    <U2::Dimension as crate::Dimension>::Length: core::ops::Neg,
    <<U2::Dimension as crate::Dimension>::Length as core::ops::Neg>::Output: typenum::Integer,
    <U2::Dimension as crate::Dimension>::Mass: core::ops::Neg,
    <<U2::Dimension as crate::Dimension>::Mass as core::ops::Neg>::Output: typenum::Integer,
    <U2::Dimension as crate::Dimension>::ElectricCurrent: core::ops::Neg,
    <<U2::Dimension as crate::Dimension>::ElectricCurrent as core::ops::Neg>::Output: typenum::Integer,
    <U2::Dimension as crate::Dimension>::ThermodynamicTemperature: core::ops::Neg,
    <<U2::Dimension as crate::Dimension>::ThermodynamicTemperature as core::ops::Neg>::Output: typenum::Integer,
    <U2::Dimension as crate::Dimension>::AmountOfSubstance: core::ops::Neg,
    <<U2::Dimension as crate::Dimension>::AmountOfSubstance as core::ops::Neg>::Output: typenum::Integer,
    <U2::Dimension as crate::Dimension>::LuminousIntensity: core::ops::Neg,
    <<U2::Dimension as crate::Dimension>::LuminousIntensity as core::ops::Neg>::Output: typenum::Integer,
{
    type Output =
        Quantity<<T1 as core::ops::Div<T2>>::Output, crate::unit::ops::MultiplyUnit<U1, crate::unit::ops::InvertUnit<U2>>>;
    fn div(self, rhs: Quantity<T2, U2>) -> Self::Output {
        Quantity {
            _unit: core::marker::PhantomData,
            value: self.value / rhs.value,
        }
    }
}
