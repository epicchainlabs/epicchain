//! Consensus message types and handling.
//!
//! This module provides comprehensive consensus message functionality,
//! including all dBFT message types and their serialization/deserialization.

use crate::{BlockIndex, ConsensusPayload, ConsensusSignature, Error, Result, ViewNumber};
use neo_config::{HASH_SIZE, MAX_BLOCK_SIZE, MAX_TRANSACTIONS_PER_BLOCK};
use neo_core::{UInt160, UInt256};
use neo_io::{BinaryWriter, MemoryReader, Serializable};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Consensus message types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsensusMessageType {
    /// Prepare request message (from primary)
    PrepareRequest = 0x00,
    /// Prepare response message (from backups)
    PrepareResponse = 0x01,
    /// Commit message (from all validators)
    Commit = 0x02,
    /// Change view message (view change request)
    ChangeView = 0x03,
    /// Recovery request message
    RecoveryRequest = 0x04,
    /// Recovery response message
    RecoveryResponse = 0x05,
}

impl ConsensusMessageType {
    /// Converts from byte value
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Self::PrepareRequest),
            0x01 => Some(Self::PrepareResponse),
            0x02 => Some(Self::Commit),
            0x03 => Some(Self::ChangeView),
            0x04 => Some(Self::RecoveryRequest),
            0x05 => Some(Self::RecoveryResponse),
            _ => None,
        }
    }

    /// Converts to byte value
    pub fn to_byte(&self) -> u8 {
        *self as u8
    }
}

/// Main consensus message wrapper
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsensusMessage {
    /// Message type
    pub message_type: ConsensusMessageType,
    /// Message payload
    pub payload: ConsensusPayload,
    /// Message signature
    pub signature: ConsensusSignature,
    /// Message-specific data
    pub data: ConsensusMessageData,
}

/// Message-specific data enum
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsensusMessageData {
    /// Prepare request data
    PrepareRequest(PrepareRequest),
    /// Prepare response data
    PrepareResponse(PrepareResponse),
    /// Commit data
    Commit(Commit),
    /// Change view data
    ChangeView(ChangeView),
    /// Recovery request data
    RecoveryRequest(RecoveryRequest),
    /// Recovery response data
    RecoveryResponse(RecoveryResponse),
}

impl ConsensusMessageData {
    /// Gets the size of the message data in bytes
    pub fn size(&self) -> usize {
        match self {
            Self::PrepareRequest(data) => data.size(),
            Self::PrepareResponse(data) => data.size(),
            Self::Commit(data) => data.size(),
            Self::ChangeView(data) => data.size(),
            Self::RecoveryRequest(data) => data.size(),
            Self::RecoveryResponse(data) => data.size(),
        }
    }

    /// Serializes message data based on type
    pub fn serialize(&self, writer: &mut BinaryWriter) -> neo_io::IoResult<()> {
        match self {
            Self::PrepareRequest(data) => Serializable::serialize(data, writer),
            Self::PrepareResponse(data) => Serializable::serialize(data, writer),
            Self::Commit(data) => Serializable::serialize(data, writer),
            Self::ChangeView(data) => Serializable::serialize(data, writer),
            Self::RecoveryRequest(data) => Serializable::serialize(data, writer),
            Self::RecoveryResponse(data) => Serializable::serialize(data, writer),
        }
    }

    /// Deserializes message data based on type
    pub fn deserialize_with_type(
        reader: &mut MemoryReader,
        message_type: ConsensusMessageType,
    ) -> neo_io::IoResult<Self> {
        match message_type {
            ConsensusMessageType::PrepareRequest => Ok(Self::PrepareRequest(
                <PrepareRequest as Serializable>::deserialize(reader)?,
            )),
            ConsensusMessageType::PrepareResponse => Ok(Self::PrepareResponse(
                <PrepareResponse as Serializable>::deserialize(reader)?,
            )),
            ConsensusMessageType::Commit => {
                Ok(Self::Commit(<Commit as Serializable>::deserialize(reader)?))
            }
            ConsensusMessageType::ChangeView => Ok(Self::ChangeView(
                <ChangeView as Serializable>::deserialize(reader)?,
            )),
            ConsensusMessageType::RecoveryRequest => Ok(Self::RecoveryRequest(
                <RecoveryRequest as Serializable>::deserialize(reader)?,
            )),
            ConsensusMessageType::RecoveryResponse => Ok(Self::RecoveryResponse(
                <RecoveryResponse as Serializable>::deserialize(reader)?,
            )),
        }
    }
}

impl ConsensusMessage {
    /// Creates a new consensus message
    pub fn new(
        message_type: ConsensusMessageType,
        payload: ConsensusPayload,
        signature: ConsensusSignature,
        data: ConsensusMessageData,
    ) -> Self {
        Self {
            message_type,
            payload,
            signature,
            data,
        }
    }

    /// Gets the validator index
    pub fn validator_index(&self) -> u8 {
        self.payload.validator_index
    }

    /// Gets the block index
    pub fn block_index(&self) -> BlockIndex {
        self.payload.block_index
    }

    /// Gets the view number
    pub fn view_number(&self) -> ViewNumber {
        self.payload.view_number
    }

    /// Gets the timestamp
    pub fn timestamp(&self) -> u64 {
        self.payload.timestamp
    }

    /// Validates the message
    pub fn validate(&self) -> Result<()> {
        // Validate message type matches data
        match (&self.message_type, &self.data) {
            (ConsensusMessageType::PrepareRequest, ConsensusMessageData::PrepareRequest(_)) => {}
            (ConsensusMessageType::PrepareResponse, ConsensusMessageData::PrepareResponse(_)) => {}
            (ConsensusMessageType::Commit, ConsensusMessageData::Commit(_)) => {}
            (ConsensusMessageType::ChangeView, ConsensusMessageData::ChangeView(_)) => {}
            (ConsensusMessageType::RecoveryRequest, ConsensusMessageData::RecoveryRequest(_)) => {}
            (ConsensusMessageType::RecoveryResponse, ConsensusMessageData::RecoveryResponse(_)) => {
            }
            _ => {
                return Err(Error::InvalidMessage(
                    "Message type and data mismatch".to_string(),
                ));
            }
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        if self.timestamp() > now + 300 {
            return Err(Error::InvalidMessage(
                "Message timestamp too far in future".to_string(),
            ));
        }

        if now > self.timestamp() + 3600 {
            return Err(Error::InvalidMessage(
                "Message timestamp too old".to_string(),
            ));
        }

        Ok(())
    }

    /// Serializes the message to bytes (matches C# Neo implementation)
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut writer = BinaryWriter::new();
        <Self as Serializable>::serialize(self, &mut writer)
            .map_err(|e| Error::Generic(format!("Failed to serialize message: {}", e)))?;
        Ok(writer.to_bytes())
    }

    /// Deserializes message from bytes (matches C# Neo implementation)
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut reader = MemoryReader::new(bytes);
        <Self as Serializable>::deserialize(&mut reader)
            .map_err(|e| Error::Generic(format!("Failed to deserialize message: {}", e)))
    }
}

impl Serializable for ConsensusMessage {
    fn size(&self) -> usize {
        1 + // message_type
        1 + 4 + 1 + 8 + // payload: validator_index, block_index, view_number, timestamp
        neo_io::helper::get_var_size(self.payload.data.len() as u64) + self.payload.data.len() + // payload data
        20 + // validator (UInt160)
        neo_io::helper::get_var_size(self.signature.signature.len() as u64) + self.signature.signature.len() + // signature
        self.data.size() // message data
    }

    fn serialize(&self, writer: &mut BinaryWriter) -> neo_io::IoResult<()> {
        // Serialize message type
        let _ = writer.write_u8(self.message_type.to_byte())?;

        // Serialize payload
        let _ = writer.write_u8(self.payload.validator_index)?;
        let _ = writer.write_u32(self.payload.block_index.value())?;
        let _ = writer.write_u8(self.payload.view_number.value())?;
        let _ = writer.write_u64(self.payload.timestamp)?;
        let _ = writer.write_var_bytes(&self.payload.data)?;

        // Serialize signature
        let _ = writer.write_serializable(&self.signature.validator)?;
        let _ = writer.write_var_bytes(&self.signature.signature)?;

        // Serialize message-specific data
        self.data.serialize(writer)?;

        Ok(())
    }

    fn deserialize(reader: &mut MemoryReader) -> neo_io::IoResult<Self> {
        let message_type =
            ConsensusMessageType::from_byte(reader.read_byte()?).ok_or_else(|| {
                neo_io::IoError::InvalidFormat {
                    expected_format: "ConsensusMessageType".to_string(),
                    reason: "Unknown message type".to_string(),
                }
            })?;

        let validator_index = reader.read_byte()?;
        let block_index = BlockIndex::new(reader.read_u32()?);
        let view_number = ViewNumber::new(reader.read_byte()?);
        let timestamp = reader.read_u64()?;
        let payload_data = reader.read_var_bytes(1024)?; // Payload data

        let payload = ConsensusPayload {
            validator_index,
            block_index,
            view_number,
            timestamp,
            data: payload_data,
        };

        let validator = <UInt160 as Serializable>::deserialize(reader)?;
        let signature_data = reader.read_var_bytes(1024)?; // Max signature size
        let signature = ConsensusSignature {
            validator,
            signature: signature_data,
        };

        let data = ConsensusMessageData::deserialize_with_type(reader, message_type)?;

        Ok(ConsensusMessage {
            message_type,
            payload,
            signature,
            data,
        })
    }
}

/// Prepare request message (sent by primary)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrepareRequest {
    /// Proposed block hash
    pub block_hash: UInt256,
    /// Proposed block data
    pub block_data: Vec<u8>,
    /// Transaction hashes included in the block
    pub transaction_hashes: Vec<UInt256>,
    /// Nonce for uniqueness
    pub nonce: u64,
}

impl PrepareRequest {
    /// Creates a new prepare request
    pub fn new(block_hash: UInt256, block_data: Vec<u8>, transaction_hashes: Vec<UInt256>) -> Self {
        Self {
            block_hash,
            block_data,
            transaction_hashes,
            nonce: rand::random(),
        }
    }

    /// Validates the prepare request
    pub fn validate(&self) -> Result<()> {
        if self.block_data.is_empty() {
            return Err(Error::InvalidMessage(
                "Block data cannot be empty".to_string(),
            ));
        }

        // Validation logic would go here in full implementation
        Ok(())
    }

    /// Gets the number of transactions
    pub fn transaction_count(&self) -> usize {
        self.transaction_hashes.len()
    }
}

impl Serializable for PrepareRequest {
    fn size(&self) -> usize {
        32 + // block_hash
        neo_io::helper::get_var_size(self.block_data.len() as u64) + self.block_data.len() + // block_data
        neo_io::helper::get_var_size(self.transaction_hashes.len() as u64) + (self.transaction_hashes.len() * 32) + // transaction_hashes
        8 // nonce
    }

    fn serialize(&self, writer: &mut BinaryWriter) -> neo_io::IoResult<()> {
        writer.write_serializable(&self.block_hash)?;
        writer.write_var_bytes(&self.block_data)?;
        writer.write_var_int(self.transaction_hashes.len() as u64)?;
        for hash in &self.transaction_hashes {
            writer.write_serializable(hash)?;
        }
        writer.write_u64(self.nonce)?;
        Ok(())
    }

    fn deserialize(reader: &mut MemoryReader) -> neo_io::IoResult<Self> {
        let block_hash = <UInt256 as Serializable>::deserialize(reader)?;
        let block_data = reader.read_var_bytes(MAX_BLOCK_SIZE)?;
        let tx_count = reader.read_var_int(MAX_TRANSACTIONS_PER_BLOCK as u64)? as usize;
        let mut transaction_hashes = Vec::with_capacity(tx_count);
        for _ in 0..tx_count {
            transaction_hashes.push(<UInt256 as Serializable>::deserialize(reader)?);
        }
        let nonce = reader.read_u64()?;

        Ok(Self {
            block_hash,
            block_data,
            transaction_hashes,
            nonce,
        })
    }
}

// Simplified implementations for other message types
impl Serializable for PrepareResponse {
    fn size(&self) -> usize {
        32 + 1 + // preparation_hash + accepted
        self.rejection_reason.as_ref().map_or(1, |r| 1 + neo_io::helper::get_var_size(r.len() as u64) + r.len())
    }
    fn serialize(&self, writer: &mut BinaryWriter) -> neo_io::IoResult<()> {
        writer.write_serializable(&self.preparation_hash)?;
        let _ = writer.write_u8(if self.accepted { 1 } else { 0 })?;
        let _ = writer.write_u8(if self.rejection_reason.is_some() {
            1
        } else {
            0
        })?;
        if let Some(ref reason) = self.rejection_reason {
            let _ = writer.write_var_string(reason)?;
        }
        Ok(())
    }
    fn deserialize(reader: &mut MemoryReader) -> neo_io::IoResult<Self> {
        Ok(Self {
            preparation_hash: <UInt256 as Serializable>::deserialize(reader)?,
            accepted: reader.read_byte()? != 0,
            rejection_reason: if reader.read_byte()? != 0 {
                Some(reader.read_var_string(256)?)
            } else {
                None
            },
        })
    }
}

impl Serializable for Commit {
    fn size(&self) -> usize {
        32 + // block_hash
        neo_io::helper::get_var_size(self.commitment_signature.len() as u64) + self.commitment_signature.len()
    }
    fn serialize(&self, writer: &mut BinaryWriter) -> neo_io::IoResult<()> {
        writer.write_serializable(&self.block_hash)?;
        writer.write_var_bytes(&self.commitment_signature)?;
        Ok(())
    }
    fn deserialize(reader: &mut MemoryReader) -> neo_io::IoResult<Self> {
        Ok(Self {
            block_hash: <UInt256 as Serializable>::deserialize(reader)?,
            commitment_signature: reader.read_var_bytes(128)?,
        })
    }
}

impl Serializable for ChangeView {
    fn size(&self) -> usize {
        1 + 8 + 1
    } // new_view_number + timestamp + reason
    fn serialize(&self, writer: &mut BinaryWriter) -> neo_io::IoResult<()> {
        let _ = writer.write_u8(self.new_view_number.value())?;
        let _ = writer.write_u64(self.timestamp)?;
        let _ = writer.write_u8(self.reason as u8)?;
        Ok(())
    }
    fn deserialize(reader: &mut MemoryReader) -> neo_io::IoResult<Self> {
        Ok(Self {
            new_view_number: ViewNumber::new(reader.read_byte()?),
            timestamp: reader.read_u64()?,
            reason: ViewChangeReason::from_byte(reader.read_byte()?)
                .unwrap_or(ViewChangeReason::PrepareRequestTimeout),
        })
    }
}

impl Serializable for RecoveryRequest {
    fn size(&self) -> usize {
        4 + 1 + 8 // block_index + view_number + timestamp
    }
    fn serialize(&self, writer: &mut BinaryWriter) -> neo_io::IoResult<()> {
        let _ = writer.write_u32(self.block_index.value())?;
        let _ = writer.write_u8(self.view_number.value())?;
        let _ = writer.write_u64(self.timestamp)?;
        Ok(())
    }
    fn deserialize(reader: &mut MemoryReader) -> neo_io::IoResult<Self> {
        Ok(Self {
            block_index: BlockIndex::new(reader.read_u32()?),
            view_number: ViewNumber::new(reader.read_byte()?),
            timestamp: reader.read_u64()?,
        })
    }
}

impl Serializable for RecoveryResponse {
    fn size(&self) -> usize {
        4 + 1 + // block_index + view_number
        neo_io::helper::get_var_size(self.change_views.len() as u64) +
        neo_io::helper::get_var_size(self.prepare_responses.len() as u64) +
        neo_io::helper::get_var_size(self.commits.len() as u64) +
        self.prepare_request.as_ref().map_or(1, |m| 1 + m.size())
    }

    fn serialize(&self, writer: &mut BinaryWriter) -> neo_io::IoResult<()> {
        // Serialize block index and view number
        let _ = writer.write_u32(self.block_index.value())?;
        let _ = writer.write_u8(self.view_number.value())?;

        // Serialize change views
        let _ = writer.write_var_int(self.change_views.len() as u64)?;
        for (_, cv) in &self.change_views {
            Serializable::serialize(cv, writer)?;
        }

        // Serialize prepare responses
        let _ = writer.write_var_int(self.prepare_responses.len() as u64)?;
        for (_, pr) in &self.prepare_responses {
            Serializable::serialize(pr, writer)?;
        }

        // Serialize commits
        let _ = writer.write_var_int(self.commits.len() as u64)?;
        for (_, c) in &self.commits {
            Serializable::serialize(c, writer)?;
        }

        // Serialize prepare request
        let _ = writer.write_u8(if self.prepare_request.is_some() { 1 } else { 0 })?;
        if let Some(ref pr) = self.prepare_request {
            Serializable::serialize(pr, writer)?;
        }

        Ok(())
    }

    fn deserialize(reader: &mut MemoryReader) -> neo_io::IoResult<Self> {
        let block_index = BlockIndex::new(reader.read_u32()?);
        let view_number = ViewNumber::new(reader.read_byte()?);

        // Read change views
        let cv_count = reader.read_var_int(1000)? as usize;
        let mut change_views = HashMap::new();
        for i in 0..cv_count {
            let cv = <ChangeView as Serializable>::deserialize(reader)?;
            change_views.insert(i as u8, cv);
        }

        // Read prepare responses
        let pr_count = reader.read_var_int(1000)? as usize;
        let mut prepare_responses = HashMap::new();
        for i in 0..pr_count {
            let pr = <PrepareResponse as Serializable>::deserialize(reader)?;
            prepare_responses.insert(i as u8, pr);
        }

        // Read commits
        let c_count = reader.read_var_int(1000)? as usize;
        let mut commits = HashMap::new();
        for i in 0..c_count {
            let c = <Commit as Serializable>::deserialize(reader)?;
            commits.insert(i as u8, c);
        }

        // Read prepare request
        let has_prepare = reader.read_byte()? != 0;
        let prepare_request = if has_prepare {
            Some(<PrepareRequest as Serializable>::deserialize(reader)?)
        } else {
            None
        };

        Ok(Self {
            block_index,
            view_number,
            change_views,
            prepare_responses,
            commits,
            prepare_request,
        })
    }
}

/// Reason for change view request
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ChangeViewReason {
    Timeout = 0x00,
    ChangeAgreement = 0x01,
    TxNotFound = 0x02,
    TxRejectedByPolicy = 0x03,
    TxInvalid = 0x04,
    BlockRejectedByPolicy = 0x05,
}

impl ChangeViewReason {
    pub fn from_byte(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(Self::Timeout),
            0x01 => Some(Self::ChangeAgreement),
            0x02 => Some(Self::TxNotFound),
            0x03 => Some(Self::TxRejectedByPolicy),
            0x04 => Some(Self::TxInvalid),
            0x05 => Some(Self::BlockRejectedByPolicy),
            _ => None,
        }
    }
}

/// Prepare response message (sent by backup validators)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrepareResponse {
    /// Hash of the prepare request being responded to
    pub preparation_hash: UInt256,
    /// Whether the validator accepts the proposal
    pub accepted: bool,
    /// Reason for rejection (if not accepted)
    pub rejection_reason: Option<String>,
}

impl PrepareResponse {
    /// Creates a new prepare response (accepted)
    pub fn accept(preparation_hash: UInt256) -> Self {
        Self {
            preparation_hash,
            accepted: true,
            rejection_reason: None,
        }
    }

    /// Creates a new prepare response (rejected)
    pub fn reject(preparation_hash: UInt256, reason: String) -> Self {
        Self {
            preparation_hash,
            accepted: false,
            rejection_reason: Some(reason),
        }
    }

    /// Checks if the response is an acceptance
    pub fn is_accepted(&self) -> bool {
        self.accepted
    }

    /// Gets the rejection reason if rejected
    pub fn rejection_reason(&self) -> Option<&str> {
        self.rejection_reason.as_deref()
    }
}

/// Commit message (sent by all validators)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Commit {
    /// Hash of the block being committed
    pub block_hash: UInt256,
    /// Commitment signature
    pub commitment_signature: Vec<u8>,
}

impl Commit {
    /// Creates a new commit message
    pub fn new(block_hash: UInt256, commitment_signature: Vec<u8>) -> Self {
        Self {
            block_hash,
            commitment_signature,
        }
    }

    /// Validates the commit message
    pub fn validate(&self) -> Result<()> {
        if self.commitment_signature.is_empty() {
            return Err(Error::InvalidMessage(
                "Commitment signature cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

/// Change view message (request to change view)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeView {
    /// New view number being requested
    pub new_view_number: ViewNumber,
    /// Reason for view change
    pub reason: ViewChangeReason,
    /// Timestamp of the request
    pub timestamp: u64,
}

/// Reasons for view change
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum ViewChangeReason {
    /// Timeout waiting for prepare request
    PrepareRequestTimeout = 0,
    /// Timeout waiting for prepare responses
    PrepareResponseTimeout = 1,
    /// Timeout waiting for commits
    CommitTimeout = 2,
    /// Invalid prepare request received
    InvalidPrepareRequest = 3,
    /// Primary node failure detected
    PrimaryFailure = 4,
    /// Network partition detected
    NetworkPartition = 5,
    /// Manual view change requested
    Manual = 6,
}

impl ViewChangeReason {
    /// Converts from byte value
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(Self::PrepareRequestTimeout),
            1 => Some(Self::PrepareResponseTimeout),
            2 => Some(Self::CommitTimeout),
            3 => Some(Self::InvalidPrepareRequest),
            4 => Some(Self::PrimaryFailure),
            5 => Some(Self::NetworkPartition),
            6 => Some(Self::Manual),
            _ => None,
        }
    }
}

impl ChangeView {
    /// Creates a new change view message
    pub fn new(new_view_number: ViewNumber, reason: ViewChangeReason) -> Self {
        Self {
            new_view_number,
            reason,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Gets the reason as a string
    pub fn reason_string(&self) -> &'static str {
        match self.reason {
            ViewChangeReason::PrepareRequestTimeout => "Prepare request timeout",
            ViewChangeReason::PrepareResponseTimeout => "Prepare response timeout",
            ViewChangeReason::CommitTimeout => "Commit timeout",
            ViewChangeReason::InvalidPrepareRequest => "Invalid prepare request",
            ViewChangeReason::PrimaryFailure => "Primary failure",
            ViewChangeReason::NetworkPartition => "Network partition",
            ViewChangeReason::Manual => "Manual",
        }
    }
}

/// Recovery request message
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryRequest {
    /// Block index for recovery
    pub block_index: BlockIndex,
    /// View number for recovery
    pub view_number: ViewNumber,
    /// Timestamp of the request
    pub timestamp: u64,
}

impl RecoveryRequest {
    /// Creates a new recovery request
    pub fn new(block_index: BlockIndex, view_number: ViewNumber) -> Self {
        Self {
            block_index,
            view_number,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}

/// Recovery response message
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryResponse {
    /// Block index for recovery
    pub block_index: BlockIndex,
    /// View number for recovery
    pub view_number: ViewNumber,
    /// Prepare request (if available)
    pub prepare_request: Option<PrepareRequest>,
    /// Prepare responses collected
    pub prepare_responses: HashMap<u8, PrepareResponse>,
    /// Commits collected
    pub commits: HashMap<u8, Commit>,
    /// Change view messages collected
    pub change_views: HashMap<u8, ChangeView>,
}

impl RecoveryResponse {
    /// Creates a new recovery response
    pub fn new(block_index: BlockIndex, view_number: ViewNumber) -> Self {
        Self {
            block_index,
            view_number,
            prepare_request: None,
            prepare_responses: HashMap::new(),
            commits: HashMap::new(),
            change_views: HashMap::new(),
        }
    }

    /// Adds a prepare request
    pub fn set_prepare_request(&mut self, prepare_request: PrepareRequest) {
        self.prepare_request = Some(prepare_request);
    }

    /// Adds a prepare response
    pub fn add_prepare_response(&mut self, validator_index: u8, response: PrepareResponse) {
        self.prepare_responses.insert(validator_index, response);
    }

    /// Adds a commit
    pub fn add_commit(&mut self, validator_index: u8, commit: Commit) {
        self.commits.insert(validator_index, commit);
    }

    /// Adds a change view
    pub fn add_change_view(&mut self, validator_index: u8, change_view: ChangeView) {
        self.change_views.insert(validator_index, change_view);
    }

    /// Gets the number of prepare responses
    pub fn prepare_response_count(&self) -> usize {
        self.prepare_responses.len()
    }

    /// Gets the number of commits
    pub fn commit_count(&self) -> usize {
        self.commits.len()
    }

    /// Gets the number of change views
    pub fn change_view_count(&self) -> usize {
        self.change_views.len()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    #[test]
    fn test_consensus_message_type() {
        assert_eq!(ConsensusMessageType::PrepareRequest.to_byte(), 0x00);
        assert_eq!(
            ConsensusMessageType::from_byte(0x00),
            Some(ConsensusMessageType::PrepareRequest)
        );

        assert_eq!(ConsensusMessageType::Commit.to_byte(), 0x02);
        assert_eq!(
            ConsensusMessageType::from_byte(0x02),
            Some(ConsensusMessageType::Commit)
        );

        assert_eq!(ConsensusMessageType::from_byte(0xFF), None);
    }

    #[test]
    fn test_prepare_request() {
        let block_hash = UInt256::from_bytes(&[1; HASH_SIZE]).unwrap();
        let block_data = vec![1, 2, 3, 4];
        let tx_hashes = vec![UInt256::from_bytes(&[2; HASH_SIZE]).unwrap()];

        let prepare_request =
            PrepareRequest::new(block_hash, block_data.clone(), tx_hashes.clone());

        assert_eq!(prepare_request.block_hash, block_hash);
        assert_eq!(prepare_request.block_data, block_data);
        assert_eq!(prepare_request.transaction_hashes, tx_hashes);
        assert_eq!(prepare_request.transaction_count(), 1);

        assert!(prepare_request.validate().is_ok());
    }

    #[test]
    fn test_prepare_response() {
        let prep_hash = UInt256::zero();

        let accept = PrepareResponse::accept(prep_hash);
        assert!(accept.is_accepted());
        assert!(accept.rejection_reason().is_none());

        let reject = PrepareResponse::reject(prep_hash, "Invalid block".to_string());
        assert!(!reject.is_accepted());
        assert_eq!(reject.rejection_reason(), Some("Invalid block"));
    }

    #[test]
    fn test_commit() {
        let block_hash = UInt256::zero();
        let signature = vec![1, 2, 3, 4, 5];

        let commit = Commit::new(block_hash, signature.clone());
        assert_eq!(commit.block_hash, block_hash);
        assert_eq!(commit.commitment_signature, signature);

        assert!(commit.validate().is_ok());
    }

    #[test]
    fn test_change_view() {
        let new_view = ViewNumber::new(2);
        let reason = ViewChangeReason::PrepareRequestTimeout;

        let change_view = ChangeView::new(new_view, reason);
        assert_eq!(change_view.new_view_number, new_view);
        assert_eq!(change_view.reason, reason);
        assert_eq!(change_view.reason_string(), "Prepare request timeout");
    }

    #[test]
    fn test_recovery_response() {
        let block_index = BlockIndex::new(100);
        let view_number = ViewNumber::new(1);

        let mut recovery = RecoveryResponse::new(block_index, view_number);
        assert_eq!(recovery.block_index, block_index);
        assert_eq!(recovery.view_number, view_number);

        // Add some responses
        let prep_response = PrepareResponse::accept(UInt256::zero());
        recovery.add_prepare_response(0, prep_response);
        assert_eq!(recovery.prepare_response_count(), 1);

        let commit = Commit::new(UInt256::zero(), vec![1, 2, 3]);
        recovery.add_commit(0, commit);
        assert_eq!(recovery.commit_count(), 1);
    }
}
