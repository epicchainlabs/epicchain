//! Elliptic curve parameters for Neo.

use num_bigint::BigInt;
use std::fmt;

/// Represents the parameters of an elliptic curve.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ECCurve {
    /// The a parameter in the curve equation y² = x³ + ax + b
    pub a: BigInt,

    /// The b parameter in the curve equation y² = x³ + ax + b
    pub b: BigInt,

    /// The prime field modulus
    pub p: BigInt,

    /// The order of the curve (number of points)
    pub n: BigInt,

    /// The base point G (generator point)
    pub g: [u8; 65],

    /// The cofactor of the curve
    pub h: BigInt,

    /// The curve name
    pub name: &'static str,
}

impl ECCurve {
    /// Returns the secp256r1 (P-256) curve parameters.
    pub fn secp256r1() -> Self {
        Self {
            a: BigInt::from(-3),
            b: BigInt::parse_bytes(
                b"5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b",
                16,
            )
            .expect("Operation failed"),
            p: BigInt::parse_bytes(
                b"ffffffff00000001000000000000000000000000ffffffffffffffffffffffff",
                16,
            )
            .expect("Operation failed"),
            n: BigInt::parse_bytes(
                b"ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551",
                16,
            )
            .expect("Operation failed"),
            g: [
                0x04, // Uncompressed point format
                0x6b, 0x17, 0xd1, 0xf2, 0xe1, 0x2c, 0x42, 0x47, 0xf8, 0xbc, 0xe6, 0xe5, 0x63, 0xa4,
                0x40, 0xf2, 0x77, 0x03, 0x7d, 0x81, 0x2d, 0xeb, 0x33, 0xa0, 0xf4, 0xa1, 0x39, 0x45,
                0xd8, 0x98, 0xc2, 0x96, 0x4f, 0xe3, 0x42, 0xe2, 0xfe, 0x1a, 0x7f, 0x9b, 0x8e, 0xe7,
                0xeb, 0x4a, 0x7c, 0x0f, 0x9e, 0x16, 0x2b, 0xce, 0x33, 0x57, 0x6b, 0x31, 0x5e, 0xce,
                0xcb, 0xb6, 0x40, 0x68, 0x37, 0xbf, 0x51, 0xf5,
            ],
            h: BigInt::from(1),
            name: "secp256r1",
        }
    }

    /// Returns the secp256k1 curve parameters.
    pub fn secp256k1() -> Self {
        Self {
            a: BigInt::from(0),
            b: BigInt::from(7),
            p: BigInt::parse_bytes(
                b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
                16,
            )
            .expect("Operation failed"),
            n: BigInt::parse_bytes(
                b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
                16,
            )
            .expect("Operation failed"),
            g: [
                0x04, // Uncompressed point format
                0x79, 0xbe, 0x66, 0x7e, 0xf9, 0xdc, 0xbb, 0xac, 0x55, 0xa0, 0x62, 0x95, 0xce, 0x87,
                0x0b, 0x07, 0x02, 0x9b, 0xfc, 0xdb, 0x2d, 0xce, 0x28, 0xd9, 0x59, 0xf2, 0x81, 0x5b,
                0x16, 0xf8, 0x17, 0x98, 0x48, 0x3a, 0xda, 0x77, 0x26, 0xa3, 0xc4, 0x65, 0x5d, 0xa4,
                0xfb, 0xfc, 0x0e, 0x11, 0x08, 0xa8, 0xfd, 0x17, 0xb4, 0x48, 0xa6, 0x85, 0x54, 0x19,
                0x9c, 0x47, 0xd0, 0x8f, 0xfb, 0x10, 0xd4, 0xb8,
            ],
            h: BigInt::from(1),
            name: "secp256k1",
        }
    }
}

impl fmt::Display for ECCurve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ECCurve: {}", self.name)
    }
}
