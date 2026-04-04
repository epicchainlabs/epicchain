//! BLS12-381 cryptographic constants
//!
//! This module contains all the cryptographic constants used in BLS12-381 operations,
//! matching the C# Neo.Cryptography.BLS12_381 constants exactly.

/// Size of a hash in bytes (32 bytes for SHA256)
pub const HASH_SIZE: usize = 32;

/// Size of a private key in bytes (HASH_SIZE bytes for scalar field)
pub const PRIVATE_KEY_SIZE: usize = HASH_SIZE;

/// Size of a public key in bytes (48 bytes for compressed G1 point)
pub const PUBLIC_KEY_SIZE: usize = 48;

/// Size of a signature in bytes (96 bytes for compressed G2 point)
pub const SIGNATURE_SIZE: usize = 96;

/// Size of an aggregate signature in bytes (same as regular signature)
pub const AGGREGATE_SIGNATURE_SIZE: usize = 96;

/// Size of an aggregate public key in bytes (same as regular public key)
pub const AGGREGATE_PUBLIC_KEY_SIZE: usize = 48;

/// BLS12-381 curve order (scalar field modulus)
/// This is the order of the scalar field Fr
pub const CURVE_ORDER: &str =
    "52435875175126190479447740508185965837690552500527637822603658699938581184513";

/// BLS12-381 field modulus (base field modulus)
/// This is the modulus of the base field Fp
pub const FIELD_MODULUS: &str = "4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787";

/// Generator point for G1 (public key group)
/// This is the standard generator for the G1 group
pub const G1_GENERATOR_X: &str = "3685416753713387016781088315183077757961620795782546409894578378688607592378376318836054947676345821548104185464507";
pub const G1_GENERATOR_Y: &str = "1339506544944476473020471379941921221584933875938349620426543736416511423956333506472724655353366534992391756441569";

/// Generator point for G2 (signature group)
/// This is the standard generator for the G2 group
pub const G2_GENERATOR_X_C0: &str = "352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160";
pub const G2_GENERATOR_X_C1: &str = "3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758";
pub const G2_GENERATOR_Y_C0: &str = "1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905";
pub const G2_GENERATOR_Y_C1: &str = "927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582";

/// Domain separation tag for hash-to-curve operations
/// This matches the IETF standard for BLS signatures
pub const HASH_TO_CURVE_DST: &[u8] = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";

/// Neo-specific domain separation tag
/// This is used for Neo blockchain specific operations
pub const NEO_DST: &[u8] = b"NEO_BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_";

/// Maximum number of signatures that can be aggregated efficiently
/// This is a practical limit for performance reasons
pub const MAX_AGGREGATE_SIZE: usize = 1000;

/// Batch verification threshold
/// Above this number of signatures, batch verification becomes more efficient
pub const BATCH_VERIFICATION_THRESHOLD: usize = 10;

/// Security parameter for hash-to-curve operations
/// This ensures sufficient security for cryptographic operations
pub const HASH_TO_CURVE_SECURITY_BITS: usize = 128;

/// Length of the expand message for hash-to-curve
/// This is used in the expand_message_xmd function
pub const EXPAND_MESSAGE_LENGTH: usize = 256;

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_sizes() {
        assert_eq!(PRIVATE_KEY_SIZE, HASH_SIZE);
        assert_eq!(PUBLIC_KEY_SIZE, 48);
        assert_eq!(SIGNATURE_SIZE, 96);
        assert_eq!(AGGREGATE_SIGNATURE_SIZE, 96);
        assert_eq!(AGGREGATE_PUBLIC_KEY_SIZE, 48);
    }

    #[test]
    fn test_domain_separation_tags() {
        assert!(!HASH_TO_CURVE_DST.is_empty());
        assert!(!NEO_DST.is_empty());
        assert_ne!(HASH_TO_CURVE_DST, NEO_DST);
    }

    #[test]
    fn test_practical_limits() {
        assert!(MAX_AGGREGATE_SIZE > 0);
        assert!(BATCH_VERIFICATION_THRESHOLD > 0);
        assert!(BATCH_VERIFICATION_THRESHOLD < MAX_AGGREGATE_SIZE);
    }

    #[test]
    fn test_security_parameters() {
        assert_eq!(HASH_TO_CURVE_SECURITY_BITS, 128);
        assert!(EXPAND_MESSAGE_LENGTH >= 256);
    }
}
