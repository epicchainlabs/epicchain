//! Elliptic curve point implementation.

use num_bigint::BigInt;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, Mul, Neg};

use super::{ECCError, ECCResult, ECCurve, ECFieldElement};
use neo_config::HASH_SIZE;

/// Represents a point on an elliptic curve.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ECPoint {
    /// The x-coordinate of the point, or None for the point at infinity
    pub x: Option<ECFieldElement>,

    /// The y-coordinate of the point, or None for the point at infinity
    pub y: Option<ECFieldElement>,

    /// The curve this point is on
    pub curve: ECCurve,
}

impl ECPoint {
    /// Creates a new point on an elliptic curve.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the point, or None for the point at infinity
    /// * `y` - The y-coordinate of the point, or None for the point at infinity
    /// * `curve` - The curve this point is on
    ///
    /// # Returns
    ///
    /// A new `ECPoint` or an error if the point is not on the curve
    pub fn new(
        x: Option<ECFieldElement>,
        y: Option<ECFieldElement>,
        curve: ECCurve,
    ) -> ECCResult<Self> {
        match (x, y) {
            (None, None) => {
                // Point at infinity is valid
                Ok(Self {
                    x: None,
                    y: None,
                    curve,
                })
            }
            (Some(x), Some(y)) => {
                let left = y.square();
                let right = &(&x.cube()
                    + &(&(&ECFieldElement::new(curve.a.clone(), x.p.clone())? * &x)
                        + &ECFieldElement::new(curve.b.clone(), x.p.clone())?));

                if left != *right {
                    return Err(ECCError::PointNotOnCurve);
                }

                Ok(Self {
                    x: Some(x),
                    y: Some(y),
                    curve,
                })
            }
            _ => {
                // One coordinate is None but the other isn't
                Err(ECCError::InvalidPointFormat)
            }
        }
    }

    /// Checks if this point is the point at infinity.
    pub fn is_infinity(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }

    /// Returns the point at infinity for the given curve.
    pub fn infinity(curve: ECCurve) -> Self {
        Self {
            x: None,
            y: None,
            curve,
        }
    }

    /// Creates an ECPoint from bytes using the default secp256r1 curve.
    ///
    /// # Arguments
    ///
    /// * `data` - The encoded point data
    ///
    /// # Returns
    ///
    /// A new `ECPoint` or an error if the data is invalid
    pub fn from_bytes(data: &[u8]) -> ECCResult<Self> {
        let curve = ECCurve::secp256r1();
        Self::decode(data, curve)
    }

    /// Decodes a point from its compressed or uncompressed form.
    ///
    /// # Arguments
    ///
    /// * `data` - The encoded point data
    /// * `curve` - The curve this point is on
    ///
    /// # Returns
    ///
    /// A new `ECPoint` or an error if the data is invalid
    pub fn decode(data: &[u8], curve: ECCurve) -> ECCResult<Self> {
        if data.is_empty() {
            return Err(ECCError::InvalidPointFormat);
        }

        match data[0] {
            // Infinity point
            0x00 => Ok(Self::infinity(curve)),
            // Uncompressed point format
            0x04 => {
                if data.len() != 65 {
                    return Err(ECCError::InvalidPointFormat);
                }

                let x_bytes = &data[1..33];
                let y_bytes = &data[33..65];

                let x_value = BigInt::from_bytes_be(num_bigint::Sign::Plus, x_bytes);
                let y_value = BigInt::from_bytes_be(num_bigint::Sign::Plus, y_bytes);

                let x = ECFieldElement::new(x_value, curve.p.clone())?;
                let y = ECFieldElement::new(y_value, curve.p.clone())?;

                Self::new(Some(x), Some(y), curve)
            }
            // Compressed point format
            0x02 | 0x03 => {
                if data.len() != 33 {
                    return Err(ECCError::InvalidPointFormat);
                }

                let x_bytes = &data[1..33];
                let x_value = BigInt::from_bytes_be(num_bigint::Sign::Plus, x_bytes);
                let x = ECFieldElement::new(x_value, curve.p.clone())?;

                let a = ECFieldElement::new(curve.a.clone(), curve.p.clone())?;
                let b = ECFieldElement::new(curve.b.clone(), curve.p.clone())?;

                let alpha = &(&x.cube() + &(&(&a * &x) + &b));

                // Calculate square root modulo p
                let exponent = (&curve.p + BigInt::one()) / BigInt::from(4);
                let beta = alpha.pow(&exponent);

                let y_bit = (data[0] & 1) == 1;
                let y = if (beta.value.clone() & BigInt::one())
                    == (if y_bit { BigInt::one() } else { BigInt::zero() })
                {
                    beta
                } else {
                    ECFieldElement::new(curve.p.clone() - beta.value, curve.p.clone())?
                };

                Self::new(Some(x), Some(y), curve)
            }
            _ => Err(ECCError::InvalidPointFormat),
        }
    }

    /// Encodes this point in compressed form.
    ///
    /// # Returns
    ///
    /// The encoded point data or an error if the point is at infinity
    pub fn encode_compressed(&self) -> ECCResult<Vec<u8>> {
        if self.is_infinity() {
            return Ok(vec![0x00]);
        }

        let x = self.x.as_ref().ok_or(ECCError::InvalidPointFormat)?;
        let y = self.y.as_ref().ok_or(ECCError::InvalidPointFormat)?;

        let mut result = Vec::with_capacity(33);

        // Determine the prefix based on the y-coordinate's parity
        let prefix = if &y.value & BigInt::one() == BigInt::one() {
            0x03
        } else {
            0x02
        };
        result.push(prefix);

        // Add the x-coordinate
        let x_bytes = x.value.to_bytes_be().1;
        let padding = vec![0; HASH_SIZE - x_bytes.len()];
        result.extend_from_slice(&padding);
        result.extend_from_slice(&x_bytes);

        Ok(result)
    }

    /// Encodes this point in uncompressed form.
    ///
    /// # Returns
    ///
    /// The encoded point data or an error if the point is at infinity
    pub fn encode_uncompressed(&self) -> ECCResult<Vec<u8>> {
        if self.is_infinity() {
            return Ok(vec![0x00]);
        }

        let x = self.x.as_ref().ok_or(ECCError::InvalidPointFormat)?;
        let y = self.y.as_ref().ok_or(ECCError::InvalidPointFormat)?;

        let mut result = Vec::with_capacity(65);
        result.push(0x04); // Uncompressed point format

        // Add the x-coordinate
        let x_bytes = x.value.to_bytes_be().1;
        let padding_x = vec![0; HASH_SIZE - x_bytes.len()];
        result.extend_from_slice(&padding_x);
        result.extend_from_slice(&x_bytes);

        // Add the y-coordinate
        let y_bytes = y.value.to_bytes_be().1;
        let padding_y = vec![0; HASH_SIZE - y_bytes.len()];
        result.extend_from_slice(&padding_y);
        result.extend_from_slice(&y_bytes);

        Ok(result)
    }

    /// Encodes this point with the specified compression.
    ///
    /// # Arguments
    ///
    /// * `compressed` - Whether to use compressed encoding
    ///
    /// # Returns
    ///
    /// The encoded point data or an error if the point is at infinity
    pub fn encode_point(&self, compressed: bool) -> ECCResult<Vec<u8>> {
        if compressed {
            self.encode_compressed()
        } else {
            self.encode_uncompressed()
        }
    }

    /// Checks if this point is valid on its curve.
    ///
    /// # Returns
    ///
    /// True if the point is valid, false otherwise
    pub fn is_valid(&self) -> bool {
        if self.is_infinity() {
            return true;
        }

        let x = match &self.x {
            Some(x) => x,
            None => return false,
        };

        let y = match &self.y {
            Some(y) => y,
            None => return false,
        };

        let y_squared = y.square();
        let x_cubed = &x.square() * x;
        let ax = &ECFieldElement::new(self.curve.a.clone(), x.p.clone()).unwrap_or_else(|_| {
            ECFieldElement::new(BigInt::zero(), x.p.clone()).expect("clone should succeed")
        }) * x;
        let b = ECFieldElement::new(self.curve.b.clone(), x.p.clone()).unwrap_or_else(|_| {
            ECFieldElement::new(BigInt::zero(), x.p.clone()).expect("clone should succeed")
        });

        let right_side = &(&x_cubed + &ax) + &b;
        y_squared == right_side
    }

    /// Decodes a compressed point from bytes.
    ///
    /// # Arguments
    ///
    /// * `data` - The compressed point data
    /// * `curve` - The elliptic curve
    ///
    /// # Returns
    ///
    /// The decoded point or an error if the data is invalid
    pub fn decode_compressed(data: &[u8], curve: ECCurve) -> ECCResult<Self> {
        if data.is_empty() {
            return Err(ECCError::InvalidPointFormat);
        }

        if data[0] == 0x00 {
            return Ok(Self::infinity(curve));
        }

        if data.len() != 33 {
            return Err(ECCError::InvalidPointFormat);
        }

        let prefix = data[0];
        if prefix != 0x02 && prefix != 0x03 {
            return Err(ECCError::InvalidPointFormat);
        }

        // Extract x-coordinate
        let x_bytes = &data[1..33];
        let x_value = BigInt::from_bytes_be(num_bigint::Sign::Plus, x_bytes);
        let x = ECFieldElement::new(x_value, curve.p.clone())?;

        let x_cubed = &x.square() * &x;
        let ax = &ECFieldElement::new(curve.a.clone(), curve.p.clone())? * &x;
        let b = ECFieldElement::new(curve.b.clone(), curve.p.clone())?;
        let y_squared = &(&x_cubed + &ax) + &b;

        let y = y_squared.sqrt()?;

        // Choose the correct y based on the prefix
        let y_final = if (prefix == 0x03) == (&y.value & BigInt::one() == BigInt::one()) {
            y
        } else {
            -&y
        };

        Self::new(Some(x), Some(y_final), curve)
    }

    /// Gets the curve this point is on.
    /// This matches the C# ECPoint.Curve property.
    ///
    /// # Returns
    ///
    /// A reference to the curve this point is on
    pub fn get_curve(&self) -> &ECCurve {
        &self.curve
    }

    /// Converts this point to bytes using compressed encoding (matches C# ECPoint.ToByteArray exactly).
    ///
    /// # Returns
    ///
    /// The encoded point data as bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        self.encode_compressed().unwrap_or_else(|_| vec![0x00])
    }

    /// Multiplies this point by a scalar.
    ///
    /// # Arguments
    ///
    /// * `k` - The scalar to multiply by
    ///
    /// # Returns
    ///
    /// The resulting point
    pub fn multiply(&self, k: &BigInt) -> ECCResult<Self> {
        if k.is_zero() {
            return Ok(Self::infinity(self.curve.clone()));
        }

        if self.is_infinity() {
            return Ok(self.clone());
        }

        let mut k = k.clone();
        let mut result = Self::infinity(self.curve.clone());
        let mut addend = self.clone();

        while k > BigInt::zero() {
            if &k & BigInt::one() == BigInt::one() {
                result = (&result + &addend)?;
            }
            addend = (&addend + &addend)?;
            k >>= 1;
        }

        Ok(result)
    }
}

impl<'b> Add<&'b ECPoint> for &ECPoint {
    type Output = ECCResult<ECPoint>;

    fn add(self, other: &'b ECPoint) -> ECCResult<ECPoint> {
        // Check that the points are on the same curve
        if self.curve.name != other.curve.name {
            return Err(ECCError::InvalidCurveParameters);
        }

        // Handle special cases
        if self.is_infinity() {
            return Ok(other.clone());
        }

        if other.is_infinity() {
            return Ok(self.clone());
        }

        let x1 = self.x.as_ref().expect("Field should be initialized");
        let y1 = self.y.as_ref().expect("Field should be initialized");
        let x2 = other.x.as_ref().expect("Value should exist");
        let y2 = other.y.as_ref().expect("Value should exist");

        if x1 == x2 && y1 != y2 {
            return Ok(ECPoint::infinity(self.curve.clone()));
        }

        let curve = self.curve.clone();

        // Calculate the slope of the line
        let slope = if x1 == x2 {
            // Point doubling
            if y1 == y2 {
                if y1.value.is_zero() {
                    return Ok(ECPoint::infinity(curve));
                }

                let three = ECFieldElement::new(BigInt::from(3), x1.p.clone())?;
                let two = ECFieldElement::new(BigInt::from(2), y1.p.clone())?;

                let numerator = &(&(&three * &x1.square())
                    + &ECFieldElement::new(curve.a.clone(), x1.p.clone())?);
                let denominator = &(&two * y1);

                numerator / denominator
            } else {
                // Points are inverses of each other
                return Ok(ECPoint::infinity(curve));
            }
        } else {
            // Point addition
            let numerator = &(y2 - y1);
            let denominator = &(x2 - x1);

            numerator / denominator
        };

        let x3 = &(&slope.square() - x1) - x2;

        let y3 = &(&(&slope * &(x1 - &x3)) - y1);

        ECPoint::new(Some(x3), Some(y3.clone()), curve)
    }
}

impl Neg for &ECPoint {
    type Output = ECPoint;

    fn neg(self) -> ECPoint {
        if self.is_infinity() {
            return self.clone();
        }

        let x = self.x.clone();
        let y = self.y.as_ref().map(|y| -y);

        // This should always be valid since we're just negating the y-coordinate
        ECPoint::new(x, y, self.curve.clone()).expect("Operation failed")
    }
}

impl<'a> Mul<&'a BigInt> for &'a ECPoint {
    type Output = ECCResult<ECPoint>;

    fn mul(self, rhs: &'a BigInt) -> ECCResult<ECPoint> {
        self.multiply(rhs)
    }
}

impl Serialize for ECPoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::Error;

        let encoded = self
            .encode_compressed()
            .map_err(|e| S::Error::custom(format!("Failed to encode ECPoint: {e}")))?;

        serializer.serialize_bytes(&encoded)
    }
}

impl<'de> Deserialize<'de> for ECPoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let bytes = Vec::<u8>::deserialize(deserializer)?;

        let curve = ECCurve::secp256r1();

        Self::decode_compressed(&bytes, curve)
            .map_err(|e| D::Error::custom(format!("Failed to decode ECPoint: {e}")))
    }
}

impl fmt::Display for ECPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_infinity() {
            write!(f, "ECPoint: Infinity")
        } else {
            write!(
                f,
                "ECPoint: ({}, {})",
                self.x.as_ref().expect("Field should be initialized"),
                self.y.as_ref().expect("Field should be initialized")
            )
        }
    }
}
