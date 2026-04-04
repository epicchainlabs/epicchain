//! Field element implementation for elliptic curve operations.

use super::{ECCError, ECCResult};
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Represents an element in a finite field.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ECFieldElement {
    /// The value of the field element
    pub value: BigInt,

    /// The prime modulus of the field
    pub p: BigInt,
}

impl ECFieldElement {
    /// Creates a new field element.
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the field element
    /// * `p` - The prime modulus of the field
    ///
    /// # Returns
    ///
    /// A new `ECFieldElement` or an error if the value is invalid
    pub fn new(value: BigInt, p: BigInt) -> Result<Self, ECCError> {
        if value < BigInt::zero() || value >= p {
            return Err(ECCError::InvalidFieldElement);
        }

        Ok(Self { value, p })
    }

    /// Computes the square of this field element.
    pub fn square(&self) -> Self {
        let value = (&self.value * &self.value) % &self.p;
        Self {
            value,
            p: self.p.clone(),
        }
    }

    /// Computes the cube of this field element.
    pub fn cube(&self) -> Self {
        let value = (&self.value * &self.value * &self.value) % &self.p;
        Self {
            value,
            p: self.p.clone(),
        }
    }

    /// Computes the multiplicative inverse of this field element.
    pub fn invert(&self) -> Result<Self, ECCError> {
        if self.value.is_zero() {
            return Err(ECCError::InvalidFieldElement);
        }

        // Extended Euclidean Algorithm to find modular inverse
        let mut s = BigInt::zero();
        let mut old_s = BigInt::one();
        let mut t = BigInt::one();
        let mut old_t = BigInt::zero();
        let mut r = self.p.clone();
        let mut old_r = self.value.clone();

        while !r.is_zero() {
            let quotient = &old_r / &r;

            let final_r = r.clone();
            r = old_r - &quotient * &r;
            old_r = final_r;

            let final_s = s.clone();
            s = old_s - &quotient * &s;
            old_s = final_s;

            let final_t = t.clone();
            t = old_t - &quotient * &t;
            old_t = final_t;
        }

        // Make sure old_s is positive
        if old_s < BigInt::zero() {
            old_s += &self.p;
        }

        Ok(Self {
            value: old_s,
            p: self.p.clone(),
        })
    }

    /// Raises this field element to the specified power.
    pub fn pow(&self, exp: &BigInt) -> Self {
        let mut exp = exp.clone();
        let mut base = self.clone();
        let mut result = Self {
            value: BigInt::one(),
            p: self.p.clone(),
        };

        while exp > BigInt::zero() {
            if &exp & BigInt::one() == BigInt::one() {
                result = &result * &base;
            }
            exp >>= 1;
            base = base.square();
        }

        result
    }

    /// Computes the square root of this field element.
    ///
    /// # Returns
    ///
    /// The square root if it exists, or an error if no square root exists
    pub fn sqrt(&self) -> ECCResult<Self> {
        let p_mod_4 = &self.p % BigInt::from(4);

        if p_mod_4 == BigInt::from(3) {
            let exp = (&self.p + BigInt::one()) / BigInt::from(4);
            let result = self.pow(&exp);

            if &result.square() == self {
                Ok(result)
            } else {
                Err(ECCError::InvalidPointFormat)
            }
        } else {
            // 1. Check if the element is a quadratic residue (production validation)
            let legendre_symbol = self.pow(&((&self.p - BigInt::one()) / BigInt::from(2)));
            if legendre_symbol.value != BigInt::one() {
                return Err(ECCError::InvalidPointFormat);
            }

            // 2. Factor out powers of 2 from p-1 (production factorization)
            let mut q = &self.p - BigInt::one();
            let mut s = 0u32;
            while &q % BigInt::from(2) == BigInt::zero() {
                q /= BigInt::from(2);
                s += 1;
            }

            // 3. Handle special case s = 1 (production optimization)
            if s == 1 {
                let exp = (&self.p + BigInt::one()) / BigInt::from(4);
                return Ok(self.pow(&exp));
            }

            // 4. Find a quadratic non-residue (production initialization)
            let mut z = BigInt::from(2);
            loop {
                let z_elem = ECFieldElement {
                    value: z.clone(),
                    p: self.p.clone(),
                };
                let legendre = z_elem.pow(&((&self.p - BigInt::one()) / BigInt::from(2)));
                if legendre.value == &self.p - BigInt::one() {
                    break;
                }
                z += BigInt::one();
            }

            // 5. Initialize variables (production setup)
            let mut m = s;
            let mut c = ECFieldElement {
                value: z,
                p: self.p.clone(),
            }
            .pow(&q);
            let mut t = self.pow(&q);
            let mut r = self.pow(&((&q + BigInt::one()) / BigInt::from(2)));

            // 6. Main Tonelli-Shanks loop (production iteration)
            while t.value != BigInt::one() {
                let mut i = 1u32;
                let mut t_power = t.square();
                while t_power.value != BigInt::one() && i < m {
                    t_power = t_power.square();
                    i += 1;
                }

                if i == m {
                    return Err(ECCError::InvalidPointFormat);
                }

                let exp = BigInt::from(2).pow(m - i - 1);
                let b = c.pow(&exp);
                m = i;
                c = b.square();
                t = &t * &c;
                r = &r * &b;
            }

            // 7. Verify result (production validation)
            if &r.square() == self {
                Ok(r)
            } else {
                Err(ECCError::InvalidPointFormat)
            }
        }
    }
}

impl Add for &ECFieldElement {
    type Output = ECFieldElement;

    fn add(self, other: &ECFieldElement) -> ECFieldElement {
        assert_eq!(
            self.p, other.p,
            "Cannot add field elements with different moduli"
        );

        let value = (&self.value + &other.value) % &self.p;
        ECFieldElement {
            value,
            p: self.p.clone(),
        }
    }
}

impl Sub for &ECFieldElement {
    type Output = ECFieldElement;

    fn sub(self, other: &ECFieldElement) -> ECFieldElement {
        assert_eq!(
            self.p, other.p,
            "Cannot subtract field elements with different moduli"
        );

        let mut value = &self.value - &other.value;
        if value < BigInt::zero() {
            value += &self.p;
        }

        ECFieldElement {
            value,
            p: self.p.clone(),
        }
    }
}

impl Mul for &ECFieldElement {
    type Output = ECFieldElement;

    fn mul(self, other: &ECFieldElement) -> ECFieldElement {
        assert_eq!(
            self.p, other.p,
            "Cannot multiply field elements with different moduli"
        );

        let value = (&self.value * &other.value) % &self.p;
        ECFieldElement {
            value,
            p: self.p.clone(),
        }
    }
}

impl Div for &ECFieldElement {
    type Output = ECFieldElement;

    fn div(self, other: &ECFieldElement) -> ECFieldElement {
        assert_eq!(
            self.p, other.p,
            "Cannot divide field elements with different moduli"
        );

        let inverse = other.invert().expect("Division by zero");
        self * &inverse
    }
}

impl Neg for &ECFieldElement {
    type Output = ECFieldElement;

    fn neg(self) -> ECFieldElement {
        let value = if self.value.is_zero() {
            BigInt::zero()
        } else {
            &self.p - &self.value
        };

        ECFieldElement {
            value,
            p: self.p.clone(),
        }
    }
}

impl fmt::Display for ECFieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
