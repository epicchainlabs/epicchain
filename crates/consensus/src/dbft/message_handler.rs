//! dBFT message handling module.
//!
//! This module contains message processing logic for the dBFT consensus engine.

use crate::{
    context::{ConsensusContext, ConsensusPhase},
    messages::{
        ChangeView, Commit, ConsensusMessage, ConsensusMessageData, PrepareRequest, PrepareResponse,
    },
    ConsensusPayload, Error, Result,
};
use neo_config::{HASH_SIZE, MAX_BLOCK_SIZE};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn};
/// Message handler for dBFT consensus
pub struct MessageHandler {
    /// Consensus context
    context: Arc<ConsensusContext>,
    /// Buffered messages for future rounds/views
    message_buffer: HashMap<(u32, u8), Vec<ConsensusMessage>>,
}

impl MessageHandler {
    /// Creates a new message handler
    pub fn new(context: Arc<ConsensusContext>) -> Self {
        Self {
            context,
            message_buffer: HashMap::new(),
        }
    }

    /// Handles an incoming consensus message
    pub async fn handle_message(
        &mut self,
        message: ConsensusMessage,
    ) -> Result<MessageHandleResult> {
        // Validate message
        message.validate()?;

        let current_round = self.context.get_current_round();

        if message.block_index() != current_round.block_index {
            return self.handle_future_message(message);
        }

        if message.view_number() < current_round.view_number {
            debug!(
                "Ignoring message from old view: {} < {}",
                message.view_number().value(),
                current_round.view_number.value()
            );
            return Ok(MessageHandleResult::Ignored);
        }

        if message.view_number() > current_round.view_number {
            return self.handle_future_view_message(message);
        }

        // Process message based on type
        match &message.data {
            ConsensusMessageData::PrepareRequest(prepare_request) => {
                self.handle_prepare_request(message.payload.clone(), prepare_request.clone())
                    .await
            }
            ConsensusMessageData::PrepareResponse(prepare_response) => {
                self.handle_prepare_response(message.payload.clone(), prepare_response.clone())
                    .await
            }
            ConsensusMessageData::Commit(commit) => {
                self.handle_commit(message.payload.clone(), commit.clone())
                    .await
            }
            ConsensusMessageData::ChangeView(change_view) => {
                self.handle_change_view(message.payload.clone(), change_view.clone())
                    .await
            }
            ConsensusMessageData::RecoveryRequest(_) => {
                // Recovery messages handled separately
                Ok(MessageHandleResult::Processed)
            }
            ConsensusMessageData::RecoveryResponse(_) => {
                // Recovery messages handled separately
                Ok(MessageHandleResult::Processed)
            }
        }
    }

    /// Handles a prepare request message
    async fn handle_prepare_request(
        &mut self,
        payload: ConsensusPayload,
        prepare_request: PrepareRequest,
    ) -> Result<MessageHandleResult> {
        debug!(
            "Handling prepare request from validator {}",
            payload.validator_index
        );

        // Validate that this is from the primary validator
        let current_round = self.context.get_current_round();

        if let Some(validator_set) = self.context.get_validator_set() {
            if let Some(primary_validator) = validator_set.get_primary(current_round.view_number) {
                if payload.validator_index != primary_validator.index {
                    return Err(Error::InvalidMessage(
                        "Prepare request not from primary validator".to_string(),
                    ));
                }
            } else {
                return Err(Error::InvalidMessage(
                    "No primary validator found".to_string(),
                ));
            }
        } else {
            return Err(Error::InvalidMessage(
                "No validator set available".to_string(),
            ));
        }

        // Validate prepare request
        prepare_request.validate()?;

        // 1. Validate block header
        if prepare_request.block_hash == neo_core::UInt256::zero() {
            return Err(Error::InvalidBlock("Block hash cannot be zero".to_string()));
        }

        // 2. Validate transaction hashes
        if prepare_request.transaction_hashes.is_empty() {
            return Err(Error::InvalidBlock(
                "Block must contain at least one transaction".to_string(),
            ));
        }

        // 3. Validate block data size
        if prepare_request.block_data.len() > MAX_BLOCK_SIZE {
            // 1MB limit
            return Err(Error::InvalidBlock("Block data too large".to_string()));
        }

        // 4. Calculate and verify merkle root (matches C# Neo Block.MerkleRoot exactly)
        let calculated_merkle_root =
            self.calculate_merkle_root(&prepare_request.transaction_hashes);

        // Extract merkle root from block data and verify it matches calculated root
        if prepare_request.block_data.len() >= HASH_SIZE {
            // In Neo block format, merkle root is at a specific offset
            // This matches the C# Neo Block.MerkleRoot verification exactly
            let block_merkle_root_bytes = &prepare_request.block_data[36..68]; // Merkle root offset in block header
            if let Ok(block_merkle_root) = neo_core::UInt256::from_bytes(block_merkle_root_bytes) {
                if block_merkle_root != calculated_merkle_root {
                    return Err(Error::InvalidBlock(format!(
                        "Merkle root mismatch: expected {}, got {}",
                        calculated_merkle_root, block_merkle_root
                    )));
                }
            }
        }

        log::info!(
            "Block proposal validation passed for block with {} transactions",
            prepare_request.transaction_hashes.len()
        );

        // Update context
        self.context.update_round(|round| {
            round.set_prepare_request(prepare_request.clone());
            round.phase = ConsensusPhase::WaitingForPrepareResponses;
        })?;

        if !self.context.am_i_primary() {
            return Ok(MessageHandleResult::SendPrepareResponse);
        }

        Ok(MessageHandleResult::Processed)
    }

    /// Handles a prepare response message
    async fn handle_prepare_response(
        &mut self,
        payload: ConsensusPayload,
        prepare_response: PrepareResponse,
    ) -> Result<MessageHandleResult> {
        debug!(
            "Handling prepare response from validator {}",
            payload.validator_index
        );

        // Basic validation
        if prepare_response.preparation_hash == neo_core::UInt256::zero() {
            return Err(Error::InvalidMessage(
                "Invalid preparation hash".to_string(),
            ));
        }

        let current_round = self.context.get_current_round();
        if current_round.prepare_request.is_none() {
            warn!("Received prepare response without prepare request");
            return Ok(MessageHandleResult::Buffered);
        }

        // Record the prepare response
        self.context.update_round(|round| {
            round.add_prepare_response(payload.validator_index, prepare_response);
        })?;

        let required_responses = self.context.get_required_signatures() - 1; // Exclude primary
        let response_count = current_round.prepare_responses.len();

        if response_count >= required_responses {
            info!(
                "Received enough prepare responses ({}/{})",
                response_count, required_responses
            );
            return Ok(MessageHandleResult::SendCommit);
        }

        Ok(MessageHandleResult::Processed)
    }

    /// Handles a commit message
    async fn handle_commit(
        &mut self,
        payload: ConsensusPayload,
        commit: Commit,
    ) -> Result<MessageHandleResult> {
        debug!("Handling commit from validator {}", payload.validator_index);

        // Validate commit
        commit.validate()?;

        // Record the commit
        self.context.update_round(|round| {
            round.add_commit(payload.validator_index, commit);
        })?;

        let required_commits = self.context.get_required_signatures();
        let commit_count = self.context.get_current_round().commits.len();

        if commit_count >= required_commits {
            info!(
                "Received enough commits ({}/{})",
                commit_count, required_commits
            );
            return Ok(MessageHandleResult::CommitBlock);
        }

        Ok(MessageHandleResult::Processed)
    }

    /// Handles a change view message
    async fn handle_change_view(
        &mut self,
        payload: ConsensusPayload,
        change_view: ChangeView,
    ) -> Result<MessageHandleResult> {
        debug!(
            "Handling change view from validator {} to view {}",
            payload.validator_index,
            change_view.new_view_number.value()
        );

        // Basic validation
        if change_view.new_view_number <= self.context.get_current_round().view_number {
            return Err(Error::InvalidMessage("Invalid view number".to_string()));
        }

        // Add change view vote
        self.context.update_round(|round| {
            round.add_change_view(payload.validator_index, change_view.clone());
        })?;

        let required_votes = self.context.get_required_signatures();
        let vote_count = self.context.get_current_round().change_views.len();

        if vote_count >= required_votes {
            info!(
                "Received enough change view votes ({}/{})",
                vote_count, required_votes
            );
            return Ok(MessageHandleResult::ChangeView(change_view.new_view_number));
        }

        Ok(MessageHandleResult::Processed)
    }

    /// Handles a message for a future round
    fn handle_future_message(&mut self, message: ConsensusMessage) -> Result<MessageHandleResult> {
        if message.block_index() > self.context.get_current_round().block_index {
            debug!(
                "Buffering message for future block {}",
                message.block_index().value()
            );
            let key = (
                message.block_index().value(),
                message.view_number().value() as u8,
            );
            self.message_buffer
                .entry(key)
                .or_insert_with(Vec::new)
                .push(message);
            return Ok(MessageHandleResult::Buffered);
        }
        Ok(MessageHandleResult::Ignored)
    }

    /// Handles a message for a future view
    fn handle_future_view_message(
        &mut self,
        message: ConsensusMessage,
    ) -> Result<MessageHandleResult> {
        debug!(
            "Buffering message for future view {}",
            message.view_number().value()
        );
        let key = (
            message.block_index().value(),
            message.view_number().value() as u8,
        );
        self.message_buffer
            .entry(key)
            .or_insert_with(Vec::new)
            .push(message);
        Ok(MessageHandleResult::Buffered)
    }

    /// Gets buffered messages for a specific round and view
    pub fn get_buffered_messages(&mut self, block_index: u32, view: u8) -> Vec<ConsensusMessage> {
        let key = (block_index, view);
        self.message_buffer.remove(&key).unwrap_or_default()
    }

    /// Clears old buffered messages
    pub fn cleanup_old_messages(&mut self, current_block_index: u32) {
        self.message_buffer
            .retain(|(block_index, _), _| *block_index >= current_block_index);
    }

    /// Calculates merkle root from transaction hashes (matches C# Neo MerkleTree.ComputeRoot exactly)
    fn calculate_merkle_root(&self, transaction_hashes: &[neo_core::UInt256]) -> neo_core::UInt256 {
        // This implementation exactly matches C# Neo's MerkleTree.ComputeRoot method
        if transaction_hashes.is_empty() {
            return neo_core::UInt256::zero();
        }

        if transaction_hashes.len() == 1 {
            return transaction_hashes[0];
        }

        let mut current_level: Vec<neo_core::UInt256> = transaction_hashes.to_vec();

        while current_level.len() > 1 {
            let mut next_level = Vec::new();

            // Process pairs of hashes
            for chunk in current_level.chunks(2) {
                let combined_hash = if chunk.len() == 2 {
                    // Hash the concatenation of two hashes
                    self.hash_pair(chunk[0], chunk[1])
                } else {
                    self.hash_pair(chunk[0], chunk[0])
                };
                next_level.push(combined_hash);
            }

            current_level = next_level;
        }

        current_level[0]
    }

    /// Hashes a pair of UInt256 values (matches C# Neo Hash256 exactly)
    fn hash_pair(&self, left: neo_core::UInt256, right: neo_core::UInt256) -> neo_core::UInt256 {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(&left.as_bytes());
        hasher.update(&right.as_bytes());
        let first_hash = hasher.finalize();

        let mut hasher = Sha256::new();
        hasher.update(&first_hash);
        let final_hash = hasher.finalize();

        neo_core::UInt256::from_bytes(&final_hash[..]).unwrap_or(neo_core::UInt256::zero())
    }
}

/// Result of message handling
#[derive(Debug, Clone, PartialEq)]
pub enum MessageHandleResult {
    /// Message was processed successfully
    Processed,
    /// Message was ignored (old view/round)
    Ignored,
    /// Message was buffered for future processing
    Buffered,
    /// Should send prepare response
    SendPrepareResponse,
    /// Should send commit
    SendCommit,
    /// Should commit block
    CommitBlock,
    /// Should change view
    ChangeView(crate::ViewNumber),
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::{Error, Result};

    #[test]
    fn test_message_handle_result() {
        assert_eq!(
            MessageHandleResult::Processed,
            MessageHandleResult::Processed
        );
        assert_ne!(MessageHandleResult::Processed, MessageHandleResult::Ignored);
    }
}
