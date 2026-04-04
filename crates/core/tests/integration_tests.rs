//
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
//
// modifications are permitted.

//! Integration tests for the Neo Core module.

use neo_core::big_decimal::BigDecimal;
use neo_core::builders::{SignerBuilder, TransactionBuilder, WitnessBuilder};
use neo_core::events::{EventHandler, EventManager};
use neo_core::extensions::byte_extensions::ByteExtensions;
use neo_core::hardfork::{Hardfork, HardforkManager};
use neo_core::neo_system::{NeoSystem, ProtocolSettings};
use neo_core::transaction_type::ContainsTransactionType;
use neo_core::uint160::{UInt160, UINT160_SIZE};
use neo_core::uint256::{UInt256, UINT256_SIZE};

use num_bigint::BigInt;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[test]
fn test_uint160_creation_and_comparison() {
    // Create UInt160 instance
    let mut uint1 = UInt160::new();
    uint1.value1 = 1;

    // Create UInt160 from bytes
    let mut data = [0u8; UINT160_SIZE];
    data[0] = 1;
    let uint2 = UInt160::from_bytes(&data).unwrap();

    // Compare UInt160 instances - they should be equal
    assert_eq!(uint1, uint2);

    let uint3 = UInt160::parse("0x0000000000000000000000000000000000000001").unwrap();
    let array = uint3.to_array();
    // So when converted back to array, it should be in the last position
    assert_eq!(array[19], 1);
    for &item in array.iter().take(19) {
        assert_eq!(item, 0);
    }

    // Test ordering
    let mut uint4 = UInt160::new();
    uint4.value3 = 1; // Most significant part

    let mut uint5 = UInt160::new();
    uint5.value3 = 2;

    assert!(uint4 < uint5);
}

#[test]
fn test_uint256_creation_and_comparison() {
    // Create UInt256 instance
    let mut uint1 = UInt256::new();
    uint1.value1 = 1;

    // Create UInt256 from bytes
    let mut data = [0u8; UINT256_SIZE];
    data[0] = 1;
    let uint2 = UInt256::from_bytes(&data).unwrap();

    // Compare UInt256 instances
    assert_eq!(uint1, uint2);

    let uint3 =
        UInt256::parse("0x0000000000000000000000000000000000000000000000000000000000000001")
            .unwrap();
    let array = uint3.to_array();
    assert_eq!(array[0], 1);
    for &item in array.iter().take(UINT256_SIZE).skip(1) {
        assert_eq!(item, 0);
    }

    // Test ordering
    let mut uint4 = UInt256::new();
    uint4.value4 = 1; // Most significant part

    let mut uint5 = UInt256::new();
    uint5.value4 = 2;

    assert!(uint4 < uint5);
}

#[test]
fn test_big_decimal_operations() {
    // Create BigDecimal instances
    let bd1 = BigDecimal::new(BigInt::from(12345), 2);
    let bd2 = BigDecimal::new(BigInt::from(12345), 2);
    let bd3 = BigDecimal::new(BigInt::from(12346), 2);

    // Test equality
    assert_eq!(bd1, bd2);
    assert!(bd1 < bd3);

    // Test changing decimals
    let increased = bd1.change_decimals(4).unwrap();
    assert_eq!(increased.value(), &BigInt::from(1234500));
    assert_eq!(increased.decimals(), 4);

    // Test parsing
    let parsed = BigDecimal::parse("123.45", 2).unwrap();
    assert_eq!(parsed, bd1);

    // Test scientific notation
    let scientific = BigDecimal::parse("1.2345e2", 2).unwrap();
    assert_eq!(scientific, bd1);

    // Test formatting
    assert_eq!(bd1.to_string(), "123.45");

    // Test with trailing zeros
    let bd4 = BigDecimal::new(BigInt::from(12300), 2);
    assert_eq!(bd4.to_string(), "123");
}

#[test]
fn test_transaction_type_enum() {
    // Test ContainsTransactionType enum
    let not_exist = ContainsTransactionType::NotExist;
    let exists_in_pool = ContainsTransactionType::ExistsInPool;
    let exists_in_ledger = ContainsTransactionType::ExistsInLedger;

    // Test Display implementation
    assert_eq!(not_exist.to_string(), "NotExist");
    assert_eq!(exists_in_pool.to_string(), "ExistsInPool");
    assert_eq!(exists_in_ledger.to_string(), "ExistsInLedger");
}

#[test]
fn test_byte_extensions() {
    // Test ByteExtensions
    let bytes = [0x01, 0x02, 0x03, 0x04];

    // Test to_hex_string
    assert_eq!(bytes.to_hex_string(false), "01020304");
    assert_eq!(bytes.to_hex_string(true), "04030201");
}

#[test]
fn test_uint160_extensions() {
    // Test UInt160Extensions
    let mut uint = UInt160::new();
    uint.value1 = 1;

    // Test to_array
    let array = uint.to_array();
    assert_eq!(array[0], 1);
    for &item in array.iter().take(UINT160_SIZE).skip(1) {
        assert_eq!(item, 0);
    }
}

#[test]
fn test_hardfork_manager() {
    // Test HardforkManager
    let mut manager = HardforkManager::new();

    // Register hardforks
    manager.register(Hardfork::HfAspidochelone, 100);
    manager.register(Hardfork::HfBasilisk, 200);

    // Test is_enabled
    assert!(!manager.is_enabled(Hardfork::HfAspidochelone, 99));
    assert!(manager.is_enabled(Hardfork::HfAspidochelone, 100));
    assert!(manager.is_enabled(Hardfork::HfAspidochelone, 101));

    assert!(!manager.is_enabled(Hardfork::HfBasilisk, 199));
    assert!(manager.is_enabled(Hardfork::HfBasilisk, 200));
    assert!(manager.is_enabled(Hardfork::HfBasilisk, 201));
}

struct TestHandler {
    called: Arc<AtomicBool>,
}

impl EventHandler for TestHandler {
    fn handle(&self, _sender: &dyn std::any::Any, _args: &dyn std::any::Any) {
        self.called.store(true, Ordering::SeqCst);
    }
}

#[test]
fn test_event_manager() {
    // Test EventManager
    let manager = EventManager::new();
    let called = Arc::new(AtomicBool::new(false));

    let handler = TestHandler {
        called: called.clone(),
    };

    // Register handler
    assert!(manager.register("test_event", handler));

    // Trigger event
    manager.trigger("test_event", &"sender", &"args");

    assert!(called.load(Ordering::SeqCst));
}

#[test]
fn test_neo_system() {
    // Test NeoSystem
    let settings = ProtocolSettings::new();
    let system = NeoSystem::new(settings);

    // Test adding and getting services
    let service = "test_service".to_string();
    system.add_service("test", service.clone()).unwrap();

    let retrieved: Arc<String> = system.get_service("test").unwrap();
    assert_eq!(*retrieved, service);
}

#[test]
fn test_builders() {
    // Test TransactionBuilder
    let tx_builder = TransactionBuilder::create_empty();
    let _tx = tx_builder.build();

    // Test SignerBuilder
    let signer_builder = SignerBuilder::create_empty();
    let _signer = signer_builder.build();

    // Test WitnessBuilder
    let witness_builder = WitnessBuilder::create_empty();
    let _witness = witness_builder.build();
}
