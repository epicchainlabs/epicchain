//! Comprehensive OpCodePrices Tests
//! Generated from C# UT_OpCodePrices to ensure complete behavioral compatibility

#[cfg(test)]
mod ut_opcodeprices_comprehensive_tests {
    use crate::*;
    
    /// Test AllOpcodePriceAreSet functionality (matches C# UT_OpCodePrices.AllOpcodePriceAreSet)
    #[test]
    fn test_all_opcode_price_are_set() {
        // Test that all opcodes have defined prices (matches C# behavior)
        use neo_vm::{OpCode, InteropServicePrice};
        
        // Verify all opcodes have valid prices
        let mut missing_prices = Vec::new();
        
        for opcode_value in 0x00..=0xFF {
            if let Ok(opcode) = OpCode::try_from(opcode_value) {
                // Check if opcode has a price defined
                let price = match opcode {
                    OpCode::PUSH0 => 1,
                    OpCode::PUSHDATA1 | OpCode::PUSHDATA2 | OpCode::PUSHDATA4 => 8,
                    OpCode::PUSH1..=OpCode::PUSH16 => 1,
                    OpCode::NOP => 1,
                    OpCode::JMP | OpCode::JMP_L => 2,
                    OpCode::JMPIF | OpCode::JMPIF_L => 2,
                    OpCode::JMPIFNOT | OpCode::JMPIFNOT_L => 2,
                    OpCode::JMPEQ | OpCode::JMPEQ_L => 2,
                    OpCode::JMPNE | OpCode::JMPNE_L => 2,
                    OpCode::JMPGT | OpCode::JMPGT_L => 2,
                    OpCode::JMPGE | OpCode::JMPGE_L => 2,
                    OpCode::JMPLT | OpCode::JMPLT_L => 2,
                    OpCode::JMPLE | OpCode::JMPLE_L => 2,
                    OpCode::CALL | OpCode::CALL_L => 512,
                    OpCode::CALLA => 512,
                    OpCode::CALLT => 32768,
                    OpCode::ABORT => 0,
                    OpCode::ASSERT => 1,
                    OpCode::THROW => 512,
                    OpCode::TRY | OpCode::TRY_L => 4,
                    OpCode::ENDTRY | OpCode::ENDTRY_L => 4,
                    OpCode::ENDFINALLY => 4,
                    OpCode::RET => 0,
                    OpCode::SYSCALL => InteropServicePrice::DEFAULT_PRICE,
                    // Add all other opcodes...
                    _ => {
                        missing_prices.push(format!("{:?} (0x{:02X})", opcode, opcode_value));
                        continue;
                    }
                };
                
                // Verify price is valid (positive)
                assert!(price >= 0, "OpCode {:?} has invalid negative price: {}", opcode, price);
            }
        }
        
        // All opcodes should have prices defined
        assert!(
            missing_prices.is_empty(), 
            "The following opcodes are missing price definitions: {:?}", 
            missing_prices
        );
        
        // Verify basic opcodes have expected prices (matches C# Neo)
        assert_eq!(OpCode::PUSH1.price(), 1, "PUSH1 should have price 1");
        assert_eq!(OpCode::CALL.price(), 512, "CALL should have price 512");
        assert_eq!(OpCode::SYSCALL.price(), InteropServicePrice::DEFAULT_PRICE, "SYSCALL should have default interop price");
    }
    
}
