// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// modifications are permitted.

//! Implementation of BigDecimal, a fixed-point number of arbitrary precision.

use crate::error::{CoreError, CoreResult};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Pow, Signed, Zero};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Mul};
use std::str::FromStr;

/// Represents a fixed-point number of arbitrary precision.
#[derive(Clone, Debug, Eq)]
pub struct BigDecimal {
    /// The value of the number.
    value: BigInt,

    /// The number of decimal places.
    decimals: u8,
}

impl BigDecimal {
    /// Creates a new BigDecimal.
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the number.
    /// * `decimals` - The number of decimal places.
    ///
    /// # Returns
    ///
    /// A new BigDecimal instance.
    pub fn new(value: BigInt, decimals: u8) -> Self {
        Self { value, decimals }
    }

    /// Gets the value of the number.
    ///
    /// # Returns
    ///
    /// The value of the number.
    pub fn value(&self) -> &BigInt {
        &self.value
    }

    /// Gets the number of decimal places.
    ///
    /// # Returns
    ///
    /// The number of decimal places.
    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    /// Gets the sign of the number.
    ///
    /// # Returns
    ///
    /// The sign of the number: -1 for negative, 0 for zero, 1 for positive.
    pub fn sign(&self) -> i8 {
        if self.value.is_zero() {
            0
        } else if self.value.is_negative() {
            -1
        } else {
            1
        }
    }

    /// Changes the decimals of the BigDecimal.
    ///
    /// # Arguments
    ///
    /// * `decimals` - The new decimals field.
    ///
    /// # Returns
    ///
    /// A Result containing either a new BigDecimal with the new number of decimal places or an error.
    pub fn change_decimals(&self, decimals: u8) -> CoreResult<Self> {
        if self.decimals == decimals {
            return Ok(self.clone());
        }

        let value = if self.decimals < decimals {
            // Increase precision
            let factor = BigInt::from(10).pow(decimals - self.decimals);
            self.value.clone() * factor
        } else {
            // Decrease precision
            let factor = BigInt::from(10).pow(self.decimals - decimals);
            let (quotient, remainder) = self.value.clone().div_rem(&factor);

            // Ensure no loss of precision
            if !remainder.is_zero() {
                return Err(CoreError::InvalidOperation {
                    message: "Cannot change decimals without losing precision".to_string(),
                });
            }

            quotient
        };

        Ok(Self { value, decimals })
    }

    /// Parses a BigDecimal from a string.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to parse.
    /// * `decimals` - The number of decimal places for the result.
    ///
    /// # Returns
    ///
    /// A Result containing either a new BigDecimal or an error.
    pub fn parse(s: &str, decimals: u8) -> CoreResult<Self> {
        // Handle scientific notation
        let mut e = 0i32;
        let mut s = s.to_string();

        if let Some(index) = s.find(['e', 'E']) {
            let e_str = &s[(index + 1)..];
            e = e_str.parse::<i32>().map_err(|_| CoreError::InvalidFormat {
                message: "Invalid exponent".to_string(),
            })?;
            s = s[..index].to_string();
        }

        // Handle decimal point
        let mut decimal_places = 0;
        if let Some(index) = s.find('.') {
            decimal_places = s.len() - index - 1;
            s = s.replace('.', "");

            // Trim trailing zeros
            while s.ends_with('0') {
                s.pop();
                decimal_places -= 1;
            }
        }

        let adjusted_decimals = decimal_places as i32 - e;
        if adjusted_decimals < 0 {
            return Err(CoreError::InvalidFormat {
                message: "Negative decimals not supported".to_string(),
            });
        }

        // Parse the integer part
        let value = BigInt::from_str(&s).map_err(|_| CoreError::InvalidFormat {
            message: "Invalid number format".to_string(),
        })?;

        // Adjust to the requested number of decimals
        let mut result = Self::new(value, adjusted_decimals as u8);
        if adjusted_decimals as u8 != decimals {
            result = result.change_decimals(decimals)?;
        }

        Ok(result)
    }
}

impl PartialEq for BigDecimal {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for BigDecimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigDecimal {
    fn cmp(&self, other: &Self) -> Ordering {
        let (left, right) = if self.decimals < other.decimals {
            let factor = BigInt::from(10).pow(other.decimals - self.decimals);
            (self.value.clone() * factor, other.value.clone())
        } else if self.decimals > other.decimals {
            let factor = BigInt::from(10).pow(self.decimals - other.decimals);
            (self.value.clone(), other.value.clone() * factor)
        } else {
            (self.value.clone(), other.value.clone())
        };

        left.cmp(&right)
    }
}

impl fmt::Display for BigDecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.decimals == 0 {
            return write!(f, "{}", self.value);
        }

        let divisor = BigInt::from(10).pow(self.decimals);
        let (quotient, mut remainder) = self.value.clone().div_rem(&divisor);

        // Handle negative remainders
        let is_negative = remainder.is_negative();
        if is_negative {
            remainder = -remainder;
        }

        let remainder_str = remainder.to_string();
        let padding = self.decimals as usize - remainder_str.len();
        let remainder_str = format!("{}{}", "0".repeat(padding), remainder_str);

        // Trim trailing zeros
        let remainder_str = remainder_str.trim_end_matches('0');

        if remainder_str.is_empty() {
            write!(f, "{quotient}")
        } else {
            let sign = if is_negative && quotient.is_zero() {
                "-"
            } else {
                ""
            };
            write!(f, "{sign}{quotient}.{remainder_str}")
        }
    }
}

impl FromStr for BigDecimal {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s, 8)
    }
}

impl Add for BigDecimal {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        // Normalize to the same number of decimals
        let max_decimals = self.decimals.max(other.decimals);
        let self_normalized = self.change_decimals(max_decimals).unwrap_or(self);
        let other_normalized = other.change_decimals(max_decimals).unwrap_or(other);

        // Add the values
        Self {
            value: self_normalized.value + other_normalized.value,
            decimals: max_decimals,
        }
    }
}

impl Mul for BigDecimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        // Multiply the values and add the decimals
        Self {
            value: self.value * other.value,
            decimals: self.decimals + other.decimals,
        }
    }
}

impl Zero for BigDecimal {
    fn zero() -> Self {
        Self::new(BigInt::zero(), 0)
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl One for BigDecimal {
    fn one() -> Self {
        Self::new(BigInt::one(), 0)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    // No extra imports needed for these unit tests

    #[test]
    fn test_big_decimal_new() {
        let bd = BigDecimal::new(BigInt::from(123), 2);
        assert_eq!(bd.value(), &BigInt::from(123));
        assert_eq!(bd.decimals(), 2);
    }
    #[test]
    fn test_big_decimal_sign() {
        let positive = BigDecimal::new(BigInt::from(123), 2);
        let zero = BigDecimal::new(BigInt::from(0), 2);
        let negative = BigDecimal::new(BigInt::from(-123), 2);
        assert_eq!(positive.sign(), 1);
        assert_eq!(zero.sign(), 0);
        assert_eq!(negative.sign(), -1);
    }
    #[test]
    fn test_big_decimal_change_decimals() {
        let bd = BigDecimal::new(BigInt::from(123), 2);
        // Increase precision
        let increased = bd.change_decimals(4).unwrap();
        assert_eq!(increased.value(), &BigInt::from(12300));
        assert_eq!(increased.decimals(), 4);
        // Decrease precision with no remainder
        let bd = BigDecimal::new(BigInt::from(12300), 4);
        let decreased = bd.change_decimals(2).unwrap();
        assert_eq!(decreased.value(), &BigInt::from(123));
        assert_eq!(decreased.decimals(), 2);
        let bd = BigDecimal::new(BigInt::from(1234), 3);
        let result = bd.change_decimals(2);
        assert!(result.is_err());
    }
    #[test]
    fn test_big_decimal_parse() {
        // Simple integer
        let bd = BigDecimal::parse("123", 2).unwrap();
        assert_eq!(bd.value(), &BigInt::from(12300));
        assert_eq!(bd.decimals(), 2);
        // Decimal
        let bd = BigDecimal::parse("123.45", 2).unwrap();
        assert_eq!(bd.value(), &BigInt::from(12345));
        assert_eq!(bd.decimals(), 2);
        // Scientific notation
        let bd = BigDecimal::parse("1.2345e2", 2).unwrap();
        assert_eq!(bd.value(), &BigInt::from(12345));
        assert_eq!(bd.decimals(), 2);
        // Negative
        let bd = BigDecimal::parse("-123.45", 2).unwrap();
        assert_eq!(bd.value(), &BigInt::from(-12345));
        assert_eq!(bd.decimals(), 2);
    }
    #[test]
    fn test_big_decimal_display() {
        // Integer
        let bd = BigDecimal::new(BigInt::from(123), 0);
        assert_eq!(bd.to_string(), "123");
        // With decimals
        let bd = BigDecimal::new(BigInt::from(12345), 2);
        assert_eq!(bd.to_string(), "123.45");
        // With trailing zeros
        let bd = BigDecimal::new(BigInt::from(12300), 2);
        assert_eq!(bd.to_string(), "123");
        // Negative
        let bd = BigDecimal::new(BigInt::from(-12345), 2);
        assert_eq!(bd.to_string(), "-123.45");
    }
    #[test]
    fn test_big_decimal_comparison() {
        let bd1 = BigDecimal::new(BigInt::from(12345), 2);
        let bd2 = BigDecimal::new(BigInt::from(12345), 2);
        let bd3 = BigDecimal::new(BigInt::from(12346), 2);
        let bd4 = BigDecimal::new(BigInt::from(123450), 3);
        assert_eq!(bd1, bd2);
        assert!(bd1 < bd3);
        assert_eq!(bd1, bd4);
    }
}
