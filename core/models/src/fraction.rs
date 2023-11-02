use num_bigint::BigInt;
use num_derive::{Num, NumOps, One, ToPrimitive, Zero};
use num_rational::{BigRational, ParseRatioError};
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Inv, One, Signed, Zero};
use serde::{
    de::{self, Deserialize, Deserializer, MapAccess, Visitor},
    ser::{Serialize, SerializeStruct, Serializer},
};
use std::{
    fmt::{self, Display},
    ops::{AddAssign, Deref, Neg, SubAssign},
    str::FromStr,
};

/// A representation of fractions using the `BigRational` data type.
///
/// This struct is a thin wrapper around the `BigRational` type from the `num-rational` crate.
/// It provides methods for common arithmetic operations, checked arithmetic, conversions, and more.
#[derive(
    Debug, Clone, PartialEq, PartialOrd, NumOps, One, Zero, Num, Ord, Eq, Default, ToPrimitive, Hash,
)]
pub struct Fraction(pub BigRational);

impl Fraction {
    /// Rounds down the fraction to the nearest multiple of the provided accuracy.
    ///
    /// # Arguments
    ///
    /// * `accuracy` - The granularity to which the number should be floored.
    ///
    /// # Returns
    ///
    /// Returns an `Option<Self>` containing the floored fraction. If the operation
    /// cannot be completed safely, it returns `None`.
    pub fn checked_floor_with_accuracy(self, accuracy: &Self) -> Option<Self> {
        Some(Self(
            self.0
                .checked_div(&accuracy.0)?
                .floor()
                .checked_mul(&accuracy.0)?,
        ))
    }

    /// Rounds the fraction to the nearest multiple of the provided accuracy.
    ///
    /// # Arguments
    ///
    /// * `accuracy` - The granularity to which the number should be rounded.
    ///
    /// # Returns
    ///
    /// Returns an `Option<Self>` containing the rounded fraction. If the operation
    /// cannot be completed safely, it returns `None`.
    pub fn checked_round_with_accuracy(self, accuracy: &Self) -> Option<Self> {
        Some(Self(
            self.0
                .checked_div(&accuracy.0)?
                .round()
                .checked_mul(&accuracy.0)?,
        ))
    }

    /// Rounds up the fraction to the nearest multiple of the provided accuracy.
    ///
    /// # Arguments
    ///
    /// * `accuracy` - The granularity to which the number should be ceiled.
    ///
    /// # Returns
    ///
    /// Returns an `Option<Self>` containing the ceiled fraction. If the operation
    /// cannot be completed safely, it returns `None`.
    pub fn checked_ceil_with_accuracy(self, accuracy: &Self) -> Option<Self> {
        Some(Self(
            self.0
                .checked_div(&accuracy.0)?
                .ceil()
                .checked_mul(&accuracy.0)?,
        ))
    }

    /// Constructs a new `Fraction` from the given raw numerator and denominator.
    ///
    /// # Arguments
    ///
    /// * `data` - A tuple consisting of a `BigInt` numerator and a `BigInt` denominator.
    ///
    /// # Returns
    ///
    /// Returns an `Option<Self>` containing the constructed fraction. Returns `None` if the
    /// denominator is zero.
    pub fn from_raw(numer: BigInt, denom: BigInt) -> Option<Self> {
        if denom == BigInt::zero() {
            return None;
        }
        Some(Self(BigRational::new_raw(numer, denom)))
    }

    /// Converts the fraction to its string representation using the specified number of decimal places.
    ///
    /// # Arguments
    ///
    /// * `max_digits` - The maximum number of decimal places to display.
    ///
    /// # Returns
    ///
    /// Returns a `String` representation of the fraction.
    pub fn to_string_numeric(&self, mut max_digits: usize) -> String {
        let mut output = self.to_integer().to_string();
        let mut remainder = self.fract();
        if !remainder.is_zero() && max_digits > 0 {
            output.push('.');
            while max_digits > 0 && !remainder.is_zero() {
                remainder *= BigInt::from(10);
                output.push_str(&remainder.to_integer().to_string());
                remainder = remainder.fract();
                max_digits -= 1;
            }
        }
        output
    }

    /// Parses a `Fraction` from a numeric string representation.
    ///
    /// # Arguments
    ///
    /// * `str` - A string slice representing the fraction.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Self, &'static str>` which is `Ok` if the parsing is successful, and `Err` otherwise.
    pub fn from_str_numeric(str: &str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = str.split('.').collect();
        match parts.len() {
            1 => {
                let integer = BigInt::from_str(parts[0])
                    .map_err(|_| "Failed to parse the whole number part")?;
                Ok(Self(BigRational::from_integer(integer)))
            }
            2 => {
                let integer = BigInt::from_str(parts[0])
                    .map_err(|_| "Failed to parse the whole number part")?;
                let fractional = BigInt::from_str(parts[1])
                    .map_err(|_| "Failed to parse the fractional part")?;
                let denominator = BigInt::from(10).pow(parts[1].len() as u32);
                let rational = BigRational::new(integer * &denominator + fractional, denominator);
                Ok(Self(rational))
            }
            _ => Err("Invalid string format"),
        }
    }
}

impl Neg for Fraction {
    type Output = Self;
    fn neg(self) -> Self {
        Fraction(self.0.neg())
    }
}

impl Inv for Fraction {
    type Output = Self;
    fn inv(self) -> Self::Output {
        Fraction(self.0.inv())
    }
}

impl Deref for Fraction {
    type Target = BigRational;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for Fraction {
    fn sub_assign(&mut self, rhs: Fraction) {
        self.0 -= rhs.0;
    }
}

impl CheckedAdd for Fraction {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        Some(Fraction(self.0.checked_add(&v.0)?))
    }
}

impl CheckedSub for Fraction {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        Some(Fraction(self.0.checked_sub(&v.0)?))
    }
}

impl CheckedMul for Fraction {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        Some(Fraction(self.0.checked_mul(&v.0)?))
    }
}

impl CheckedDiv for Fraction {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        Some(Fraction(self.0.checked_div(&v.0)?))
    }
}

impl From<(BigInt, BigInt)> for Fraction {
    fn from(value: (BigInt, BigInt)) -> Self {
        Fraction(BigRational::from(value))
    }
}

impl From<usize> for Fraction {
    fn from(value: usize) -> Self {
        Self(BigRational::from_integer(value.into()))
    }
}

impl From<BigInt> for Fraction {
    fn from(value: BigInt) -> Self {
        Self(BigRational::from(value))
    }
}

impl FromStr for Fraction {
    type Err = ParseRatioError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(BigRational::from_str(s)?))
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Signed for Fraction {
    fn abs(&self) -> Self {
        Fraction(self.0.abs())
    }
    fn abs_sub(&self, other: &Self) -> Self {
        Fraction(self.0.abs_sub(&other.0))
    }
    fn signum(&self) -> Self {
        if self.is_positive() {
            Self::one()
        } else if self.is_zero() {
            Self::zero()
        } else {
            -Self::one()
        }
    }
    fn is_negative(&self) -> bool {
        self.0.is_negative()
    }
    fn is_positive(&self) -> bool {
        self.0.is_positive()
    }
}

// serialization

const STRUCT: &str = "Fraction";
const FIELDS: &[&str] = &["numer", "denom"];

impl Serialize for Fraction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (numer, denom) = (self.0.numer(), self.0.denom());
        let mut state = serializer.serialize_struct(STRUCT, 2)?;
        state.serialize_field(FIELDS[0], numer.to_string().as_str())?;
        state.serialize_field(FIELDS[1], denom.to_string().as_str())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Fraction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Numer,
            Denom,
        }

        struct FractionVisitor;

        impl<'de> Visitor<'de> for FractionVisitor {
            type Value = Fraction;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(STRUCT)
            }

            fn visit_map<V>(self, mut map: V) -> Result<Fraction, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut numer: Option<BigInt> = None;
                let mut denom: Option<BigInt> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Numer => {
                            if numer.is_some() {
                                return Err(de::Error::duplicate_field(FIELDS[0]));
                            }
                            numer = Some(
                                BigInt::from_str(map.next_value::<String>()?.as_str())
                                    .map_err(de::Error::custom)?,
                            );
                        }
                        Field::Denom => {
                            if denom.is_some() {
                                return Err(de::Error::duplicate_field(FIELDS[1]));
                            }
                            denom = Some(
                                BigInt::from_str(map.next_value::<String>()?.as_str())
                                    .map_err(de::Error::custom)?,
                            );
                        }
                    }
                }
                let numer = numer.ok_or_else(|| de::Error::missing_field(FIELDS[0]))?;
                let denom = denom.ok_or_else(|| de::Error::missing_field(FIELDS[1]))?;
                Ok(Fraction(
                    BigRational::try_from((numer, denom)).map_err(de::Error::custom)?,
                ))
            }
        }

        deserializer.deserialize_struct(STRUCT, FIELDS, FractionVisitor)
    }
}

#[cfg(test)]
use num_bigint::Sign;
#[cfg(test)]
use proptest::{
    prelude::{Arbitrary, *},
    sample::size_range,
};

#[cfg(test)]
impl Arbitrary for Fraction {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary() -> Self::Strategy {
        let sign = Sign::Plus;
        let numer = any::<Vec<u8>>().prop_map(move |v| BigInt::from_bytes_le(sign, v.as_slice()));
        let denom = any::<Vec<u8>>()
            .prop_map(move |v| BigInt::from_bytes_le(sign, v.as_slice()))
            .prop_map(|n| if n.is_zero() { n + 1 } else { n });
        (numer, denom)
            .prop_map(|(numer, denom)| Fraction(BigRational::from((numer, denom))))
            .boxed()
    }

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        let sign = Sign::Plus;
        let numer = any_with::<Vec<u8>>(size_range(args).lift())
            .prop_map(move |v| BigInt::from_bytes_le(sign, v.as_slice()));
        let denom = any_with::<Vec<u8>>(size_range(args).lift())
            .prop_map(move |v| BigInt::from_bytes_le(sign, v.as_slice()))
            .prop_map(|n| if n.is_zero() { n + 1 } else { n });
        (numer, denom)
            .prop_map(|(numer, denom)| Fraction(BigRational::from((numer, denom))))
            .boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::Fraction;
    use num_bigint::BigInt;
    use proptest::{prelude::*, proptest};

    const TESTS_CASES: u32 = 1000;
    const FRACTION_BYTES: usize = 4;
    
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(TESTS_CASES))]
        #[test]
        fn serialization(fraction in any_with::<Fraction>(FRACTION_BYTES)) {
            assert_eq!(fraction, serde_json::from_str(&serde_json::to_string(&fraction).unwrap()).unwrap());
        }
    }
    
    const ACCURACY: usize = 1000;
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(TESTS_CASES))]

        #[test]
        fn to_numeric_from_numeric(fraction in any_with::<Fraction>(FRACTION_BYTES)) {
            let f_rounded = fraction.checked_floor_with_accuracy(&Fraction::from(BigInt::from(ACCURACY))).unwrap();
            let f_rounded_tostr = f_rounded.to_string_numeric(ACCURACY);
            let f_rounded_fromstr = Fraction::from_str_numeric(&f_rounded_tostr).unwrap();
            assert_eq!(f_rounded, f_rounded_fromstr);
        }
    }
}
