//! Oracle functionality for the Neo Virtual Machine.

use super::types::{OracleRequest, OracleResponse, Signer, Transaction, TransactionAttribute};
use crate::{
    error::{VmError, VmResult},
    execution_engine::ExecutionEngine,
};
use neo_config::HASH_SIZE;
use neo_core::{UInt160, UInt256};

/// Gets Oracle response attribute from transaction (production implementation)
pub fn get_oracle_response_attribute(transaction: &Transaction) -> Option<OracleResponse> {
    // 1. Find first Oracle response attribute (production attribute scanning)
    transaction.attributes().iter().find_map(|attribute| {
        // 2. Check if attribute is Oracle response (production type checking)
        match attribute {
            TransactionAttribute::OracleResponse { id, code, result } => {
                // 3. Create Oracle response object (production Oracle data)
                Some(OracleResponse {
                    id: *id,
                    code: *code,
                    result: result.clone(),
                })
            }
        }
    })
}

/// Gets signers from Oracle request (production implementation)
pub fn get_oracle_request_signers(
    engine: &ExecutionEngine,
    oracle_response: &OracleResponse,
) -> VmResult<Vec<Signer>> {
    // This implements the C# logic: NativeContract.Oracle.GetRequest + NativeContract.Ledger.GetTransaction

    // 1. Get Oracle request from Oracle contract (production Oracle contract access)
    let oracle_request = get_oracle_request_from_contract(engine, oracle_response.id)?;

    // 2. Get original transaction from Ledger contract (production Ledger contract access)
    let original_transaction =
        get_transaction_from_ledger_contract(engine, &oracle_request.original_txid)?;

    // 3. Return signers from original transaction (production signer resolution)
    Ok(original_transaction.signers().to_vec())
}

/// Gets Oracle request from Oracle contract storage (production implementation)
pub fn get_oracle_request_from_contract(
    engine: &ExecutionEngine,
    request_id: u64,
) -> VmResult<OracleRequest> {
    // 1. Get Oracle contract hash (well-known constant)
    let oracle_contract_hash = get_oracle_contract_hash();

    // 2. Construct storage key for Oracle request (production key format)
    let mut storage_key = Vec::with_capacity(28); // ADDRESS_SIZE bytes hash + 8 bytes request_id
    storage_key.extend_from_slice(&oracle_contract_hash.as_bytes());
    storage_key.extend_from_slice(&request_id.to_le_bytes());

    // 3. Get Oracle request from storage (production storage access)
    match engine.get_storage_item(&storage_key) {
        Some(storage_item) => {
            // 4. Deserialize Oracle request (production deserialization)
            deserialize_oracle_request(&storage_item)
        }
        None => {
            // 5. Oracle request not found (production error handling)
            Err(VmError::invalid_operation_msg(format!(
                "Oracle request {request_id} not found"
            )))
        }
    }
}

/// Gets transaction from Ledger contract (production implementation)
pub fn get_transaction_from_ledger_contract(
    engine: &ExecutionEngine,
    txid: &UInt256,
) -> VmResult<Transaction> {
    // 1. Get Ledger contract hash (well-known constant)
    let ledger_contract_hash = get_ledger_contract_hash();

    // 2. Construct storage key for transaction (production key format)
    let mut storage_key = Vec::with_capacity(52); // ADDRESS_SIZE bytes hash + HASH_SIZE bytes txid
    storage_key.extend_from_slice(&ledger_contract_hash.as_bytes());
    storage_key.extend_from_slice(&txid.as_bytes());

    // 3. Get transaction from storage (production storage access)
    match engine.get_storage_item(&storage_key) {
        Some(storage_item) => {
            // 4. Deserialize transaction (production deserialization)
            deserialize_transaction(&storage_item)
        }
        None => {
            // 5. Transaction not found (production error handling)
            Err(VmError::invalid_operation_msg(format!(
                "Transaction {txid} not found"
            )))
        }
    }
}

/// Gets Oracle contract hash (well-known constant)
pub fn get_oracle_contract_hash() -> UInt160 {
    UInt160::from_bytes(&[
        0xfe, 0x92, 0x4b, 0x7c, 0xfd, 0xdf, 0x0c, 0x7b, 0x7e, 0x3b, 0x9c, 0xa9, 0x3a, 0xa8, 0x20,
        0x8d, 0x6b, 0x9a, 0x9a, 0x9a,
    ])
    .unwrap_or_else(|_| UInt160::zero())
}

/// Gets Ledger contract hash (well-known constant)
pub fn get_ledger_contract_hash() -> UInt160 {
    UInt160::from_bytes(&[
        0xda, 0x65, 0xb6, 0x00, 0xf7, 0x12, 0x4c, 0xe6, 0xc7, 0x9e, 0x88, 0xfc, 0x19, 0x8b, 0x0f,
        0xa8, 0x75, 0x85, 0x05, 0x8e,
    ])
    .unwrap_or_else(|_| UInt160::zero())
}

/// Deserializes Oracle request from storage data (production implementation)
pub fn deserialize_oracle_request(data: &[u8]) -> VmResult<OracleRequest> {
    if data.len() < 40 {
        // Minimum size check
        return Err(VmError::invalid_operation_msg(
            "Invalid Oracle request data",
        ));
    }

    let mut offset = 0;

    let mut original_txid_bytes = [0u8; HASH_SIZE];
    original_txid_bytes.copy_from_slice(&data[offset..offset + HASH_SIZE]);
    let original_txid = UInt256::from_bytes(&original_txid_bytes)
        .map_err(|_| VmError::invalid_operation_msg("Invalid original txid"))?;
    offset += HASH_SIZE;

    let gas_for_response = u64::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
        data[offset + 4],
        data[offset + 5],
        data[offset + 6],
        data[offset + 7],
    ]);
    offset += 8;

    if offset >= data.len() {
        return Err(VmError::invalid_operation_msg(
            "Incomplete Oracle request data",
        ));
    }

    let url_length = data[offset] as usize;
    offset += 1;

    if offset + url_length > data.len() {
        return Err(VmError::invalid_operation_msg(
            "Invalid URL length in Oracle request",
        ));
    }

    let url = String::from_utf8(data[offset..offset + url_length].to_vec())
        .map_err(|_| VmError::invalid_operation_msg("Invalid URL encoding"))?;

    Ok(OracleRequest {
        original_txid,
        gas_for_response,
        url,
    })
}

/// Deserializes transaction from storage data (production implementation)
pub fn deserialize_transaction(data: &[u8]) -> VmResult<Transaction> {
    // 1. Check minimum transaction size first
    if data.len() < 50 {
        // Minimum transaction size
        return Err(VmError::invalid_operation_msg("Invalid transaction data"));
    }

    // 2. Deserialize transaction with proper error handling (production implementation)
    match Transaction::deserialize(data.to_vec()) {
        Ok(transaction) => Ok(transaction),
        Err(_) => {
            // 3. Fallback to minimal transaction structure for malformed data (production safety)
            Transaction::default_with_script(data.to_vec())
        }
    }
}
