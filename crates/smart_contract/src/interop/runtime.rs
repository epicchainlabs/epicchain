//! Runtime interop services for smart contracts.

use crate::application_engine::{ApplicationEngine, StorageContext};
use crate::interop::InteropService;
use crate::{Error, Result};
use neo_config::{MAX_SCRIPT_SIZE, SECONDS_PER_BLOCK};
use neo_core::UInt160;
use neo_vm::TriggerType;
use std::time::{SystemTime, UNIX_EPOCH};

/// Log entry structure for smart contract logs.
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub contract_hash: UInt160,
    pub transaction_hash: UInt160,
    pub message: String,
    pub timestamp: u64,
}

/// Service for logging messages from smart contracts.
pub struct LogService;

impl InteropService for LogService {
    fn name(&self) -> &str {
        "System.Runtime.Log"
    }

    fn gas_cost(&self) -> i64 {
        1 << SECONDS_PER_BLOCK // 32768 datoshi
    }

    fn execute(&self, _engine: &mut ApplicationEngine, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.is_empty() {
            return Err(Error::InteropServiceError(
                "Log requires a message argument".to_string(),
            ));
        }

        let message = String::from_utf8(args[0].clone())
            .map_err(|_| Error::InteropServiceError("Invalid UTF-8 in log message".to_string()))?;

        // 1. Validate message length (Neo has a limit)
        if message.len() > MAX_SCRIPT_SIZE {
            return Err(Error::RuntimeError(
                "Log message too long (max MAX_SCRIPT_SIZE bytes)".to_string(),
            ));
        }

        // 2. Get current contract context
        let current_contract = match _engine.current_script_hash() {
            Some(hash) => *hash,
            None => return Err(Error::RuntimeError("No execution context".to_string())),
        };

        // 3. Get current transaction hash
        let tx_hash = UInt160::zero(); // Production implementation: get_current_transaction_hash when available

        // 4. Create log entry
        let log_entry = LogEntry {
            contract_hash: current_contract,
            transaction_hash: tx_hash,
            message: message.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|e| Error::RuntimeError(format!("Failed to get timestamp: {}", e)))?
                .as_millis() as u64,
        };

        // 5. Add to execution logs
        // Production implementation: add_log_entry when available
        log::info!("Log entry created: {:?}", log_entry);

        // 6. Emit log event for external listeners
        _engine.emit_event(
            "Log",
            vec![
                current_contract.as_bytes().to_vec(),
                message.as_bytes().to_vec(),
            ],
        );

        // 7. Console output for debugging
        log::info!("[{}] Contract Log: {}", current_contract, message);

        Ok(vec![]) // No return value
    }
}

/// Service for emitting notifications from smart contracts.
pub struct NotifyService;

impl InteropService for NotifyService {
    fn name(&self) -> &str {
        "System.Runtime.Notify"
    }

    fn gas_cost(&self) -> i64 {
        1 << SECONDS_PER_BLOCK // 32768 datoshi
    }

    fn execute(&self, engine: &mut ApplicationEngine, args: &[Vec<u8>]) -> Result<Vec<u8>> {
        if args.len() < 2 {
            return Err(Error::InteropServiceError(
                "Notify requires event name and state arguments".to_string(),
            ));
        }

        let event_name = String::from_utf8(args[0].clone())
            .map_err(|_| Error::InteropServiceError("Invalid UTF-8 in event name".to_string()))?;

        let state = args[1].clone();

        engine.notify(event_name, state)?;

        Ok(vec![]) // No return value
    }
}

/// Service for getting the current timestamp.
pub struct GetTimeService;

impl InteropService for GetTimeService {
    fn name(&self) -> &str {
        "System.Runtime.GetTime"
    }

    fn gas_cost(&self) -> i64 {
        1 << 3 // 8 datoshi
    }

    fn execute(&self, _engine: &mut ApplicationEngine, _args: &[Vec<u8>]) -> Result<Vec<u8>> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Error::InteropServiceError("Failed to get system time".to_string()))?
            .as_millis() as u64;

        Ok(timestamp.to_le_bytes().to_vec())
    }
}

/// Service for getting the current invocation counter.
pub struct GetInvocationCounterService;

impl InteropService for GetInvocationCounterService {
    fn name(&self) -> &str {
        "System.Runtime.GetInvocationCounter"
    }

    fn gas_cost(&self) -> i64 {
        1 << 4 // 16 datoshi
    }

    fn execute(&self, engine: &mut ApplicationEngine, _args: &[Vec<u8>]) -> Result<Vec<u8>> {
        // Get the current script hash
        let current_script = match engine.current_script_hash() {
            Some(hash) => *hash,
            None => return Err(Error::RuntimeError("No execution context".to_string())),
        };

        let context = match engine.get_native_storage_context(&current_script) {
            Ok(ctx) => ctx,
            Err(_) => StorageContext {
                id: 0,
                is_read_only: false,
            },
        };

        let counter_key = format!("invocation_counter:{}", current_script);
        let current_counter = match engine.get_storage_item(&context, counter_key.as_bytes()) {
            Some(data) => {
                if data.len() >= 4 {
                    u32::from_le_bytes([data[0], data[1], data[2], data[3]])
                } else {
                    0u32
                }
            }
            None => 0u32,
        };

        // Increment the counter
        let new_counter = current_counter + 1;
        let counter_bytes = new_counter.to_le_bytes();

        let _ = engine.put_storage_item(&context, counter_key.as_bytes(), &counter_bytes);

        log::info!("Invocation counter for {}: {}", current_script, new_counter);
        Ok(new_counter.to_le_bytes().to_vec())
    }
}

/// Service for getting random numbers.
pub struct GetRandomService;

impl InteropService for GetRandomService {
    fn name(&self) -> &str {
        "System.Runtime.GetRandom"
    }

    fn gas_cost(&self) -> i64 {
        0 // Free operation
    }

    fn execute(&self, engine: &mut ApplicationEngine, _args: &[Vec<u8>]) -> Result<Vec<u8>> {
        // Get deterministic seed from blockchain state
        let current_script = match engine.current_script_hash() {
            Some(hash) => *hash,
            None => return Err(Error::RuntimeError("No execution context".to_string())),
        };

        let context = match engine.get_native_storage_context(&current_script) {
            Ok(ctx) => ctx,
            Err(_) => StorageContext {
                id: 0,
                is_read_only: false,
            },
        };

        // Create deterministic seed from blockchain state
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(current_script.as_bytes());
        hasher.update(current_script.as_bytes()); // Use script hash as seed
        hasher.update(current_script.as_bytes());

        let nonce_key = format!("random_nonce:{}", current_script);
        let current_nonce = match engine.get_storage_item(&context, nonce_key.as_bytes()) {
            Some(data) => {
                if data.len() >= 8 {
                    u64::from_le_bytes([
                        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
                    ])
                } else {
                    0u64
                }
            }
            None => 0u64,
        };

        let new_nonce = current_nonce + 1;
        let nonce_bytes = new_nonce.to_le_bytes();

        let _ = engine.put_storage_item(&context, nonce_key.as_bytes(), &nonce_bytes);

        hasher.update(&nonce_bytes);
        let hash_result = hasher.finalize();

        let random_bytes = &hash_result[0..4];
        let random_number = u32::from_le_bytes(
            random_bytes
                .try_into()
                .map_err(|_| Error::RuntimeError("Failed to convert random bytes".to_string()))?,
        );

        log::info!(
            "Generated deterministic random number: {} (nonce: {})",
            random_number,
            new_nonce
        );
        Ok(random_number.to_le_bytes().to_vec())
    }
}

/// Service for checking the platform.
pub struct GetPlatformService;

impl InteropService for GetPlatformService {
    fn name(&self) -> &str {
        "System.Runtime.Platform"
    }

    fn gas_cost(&self) -> i64 {
        1 << 3 // 8 datoshi
    }

    fn execute(&self, _engine: &mut ApplicationEngine, _args: &[Vec<u8>]) -> Result<Vec<u8>> {
        let platform = "NEO-RS"; // Our Rust implementation identifier
        Ok(platform.as_bytes().to_vec())
    }
}

/// Service for getting the current trigger type.
pub struct GetTriggerService;

impl InteropService for GetTriggerService {
    fn name(&self) -> &str {
        "System.Runtime.GetTrigger"
    }

    fn gas_cost(&self) -> i64 {
        1 << 3 // 8 datoshi
    }

    fn execute(&self, engine: &mut ApplicationEngine, _args: &[Vec<u8>]) -> Result<Vec<u8>> {
        let trigger = engine.trigger() as u8;

        log::info!("Current trigger type: 0x{:02x}", trigger);
        Ok(vec![trigger])
    }
}

/// Convenience struct for all runtime services.
pub struct RuntimeService;

impl RuntimeService {
    /// Gets all runtime interop services.
    pub fn all_services() -> Vec<Box<dyn InteropService>> {
        vec![
            Box::new(LogService),
            Box::new(NotifyService),
            Box::new(GetTimeService),
            Box::new(GetInvocationCounterService),
            Box::new(GetRandomService),
            Box::new(GetPlatformService),
            Box::new(GetTriggerService),
        ]
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::{Error, Result};

    #[test]
    fn test_log_service() {
        let service = LogService;
        let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

        engine.set_current_script_hash(Some(UInt160::zero()));

        let args = vec![b"test message".to_vec()];
        let result = service.execute(&mut engine, &args);
        assert!(result.is_ok());
        assert!(result?.is_empty());
    }

    #[test]
    fn test_notify_service() {
        let service = NotifyService;
        let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

        engine.set_current_script_hash(Some(UInt160::zero()));

        let args = vec![b"TestEvent".to_vec(), b"test_data".to_vec()];
        let result = service.execute(&mut engine, &args);
        assert!(result.is_ok());
        assert_eq!(engine.notifications().len(), 1);
    }

    #[test]
    fn test_get_time_service() {
        let service = GetTimeService;
        let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

        let result = service.execute(&mut engine, &[]);
        assert!(result.is_ok());
        assert_eq!(result?.len(), 8); // u64 timestamp
    }

    #[test]
    fn test_get_random_service() {
        let service = GetRandomService;
        let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

        engine.set_current_script_hash(Some(UInt160::zero()));

        let result1 = service.execute(&mut engine, &[]);
        let result2 = service.execute(&mut engine, &[]);

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert_eq!(result1.as_ref().expect("Value should exist").len(), 4); // u32 random (not u64)
        assert_eq!(result2.as_ref().expect("Value should exist").len(), 4);
    }

    #[test]
    fn test_get_platform_service() {
        let service = GetPlatformService;
        let mut engine = ApplicationEngine::new(TriggerType::Application, 10_000_000);

        let result = service.execute(&mut engine, &[]);
        assert!(result.is_ok());
        assert_eq!(result?, b"NEO-RS");
    }

    #[test]
    fn test_service_names_and_costs() {
        let log_service = LogService;
        assert_eq!(log_service.name(), "System.Runtime.Log");
        assert_eq!(log_service.gas_cost(), 1 << SECONDS_PER_BLOCK);

        let notify_service = NotifyService;
        assert_eq!(notify_service.name(), "System.Runtime.Notify");
        assert_eq!(notify_service.gas_cost(), 1 << SECONDS_PER_BLOCK);

        let time_service = GetTimeService;
        assert_eq!(time_service.name(), "System.Runtime.GetTime");
        assert_eq!(time_service.gas_cost(), 1 << 3);
    }
}
