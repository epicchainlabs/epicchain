//! ExtensiblePayload wrapper for consensus messages.
//!
//! This module provides functionality to wrap consensus messages in ExtensiblePayload
//! format for Neo N3 network compatibility.

use crate::{ConsensusMessage, Error, Result};
use neo_core::{UInt160, Witness};
use neo_network::messages::ExtensiblePayload;

/// Wraps a consensus message in an ExtensiblePayload for network transmission
pub fn wrap_consensus_message(
    message: &ConsensusMessage,
    valid_block_start: u32,
    valid_block_end: u32,
    sender: UInt160,
    witness: Witness,
) -> Result<ExtensiblePayload> {
    // Serialize the consensus message
    let message_bytes = message.to_bytes()?;

    // Create ExtensiblePayload with "dBFT" category
    Ok(ExtensiblePayload::consensus(
        valid_block_start,
        valid_block_end,
        sender,
        message_bytes,
        witness,
    ))
}

/// Unwraps a consensus message from an ExtensiblePayload
pub fn unwrap_consensus_message(payload: &ExtensiblePayload) -> Result<ConsensusMessage> {
    // Check if it's a consensus payload
    if !payload.is_consensus() {
        return Err(Error::Generic(format!(
            "ExtensiblePayload is not a consensus message, category: {}",
            payload.category
        )));
    }

    // Deserialize the consensus message from the payload data
    ConsensusMessage::from_bytes(&payload.data)
}

/// Creates an ExtensiblePayload for broadcasting a consensus message
pub fn create_broadcast_payload(
    message: &ConsensusMessage,
    current_height: u32,
    sender: UInt160,
    witness: Witness,
) -> Result<ExtensiblePayload> {
    // Consensus messages are typically valid for a range of blocks
    // Starting from current height and valid for next 100 blocks
    let valid_block_start = current_height;
    let valid_block_end = current_height + 100;

    wrap_consensus_message(message, valid_block_start, valid_block_end, sender, witness)
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::messages::ChangeView;
    use crate::{ConsensusMessageData, ConsensusMessageType, ConsensusPayload, ConsensusSignature};

    #[test]
    fn test_wrap_unwrap_consensus_message() {
        // Create a test consensus message
        let payload = ConsensusPayload {
            version: 0,
            prev_hash: neo_core::UInt256::zero(),
            block_index: 100,
            validator_index: 1,
            timestamp: 1234567890,
        };

        let signature = ConsensusSignature {
            data: vec![1, 2, 3, 4],
        };

        let data = ConsensusMessageData::ChangeView(ChangeView {
            new_view_number: 2,
            timestamp: 1234567890,
            reason: crate::messages::ChangeViewReason::Timeout,
        });

        let message =
            ConsensusMessage::new(ConsensusMessageType::ChangeView, payload, signature, data);

        // Wrap the message
        let sender = UInt160::zero();
        let witness = Witness::new(vec![5, 6, 7], vec![8, 9, 10]);
        let extensible = wrap_consensus_message(&message, 100, 200, sender, witness).unwrap();

        // Check the payload
        assert_eq!(extensible.category, "dBFT");
        assert!(extensible.is_consensus());
        assert_eq!(extensible.valid_block_start, 100);
        assert_eq!(extensible.valid_block_end, 200);

        // Unwrap the message
        let unwrapped = unwrap_consensus_message(&extensible).unwrap();
        assert_eq!(unwrapped, message);
    }

    #[test]
    fn test_unwrap_non_consensus_payload() {
        // Create a non-consensus ExtensiblePayload
        let payload = ExtensiblePayload::new(
            "other".to_string(),
            100,
            200,
            UInt160::zero(),
            vec![1, 2, 3],
            Witness::new(vec![], vec![]),
        );

        // Try to unwrap as consensus message
        let result = unwrap_consensus_message(&payload);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("not a consensus message"));
    }
}
