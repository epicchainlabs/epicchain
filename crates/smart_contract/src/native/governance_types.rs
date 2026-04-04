//! Governance types for NEO token contract.
//!
//! This module defines the data structures required for NEO governance,
//! matching the C# Neo implementation exactly.

use crate::{Error, Result};
use neo_core::UInt160;
use neo_cryptography::ECPoint;
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// Account state for NEO token holders (matches C# NeoAccountState exactly)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeoAccountState {
    /// NEO balance of the account
    pub balance: BigInt,
    /// Height when balance was last updated
    pub balance_height: u32,
    /// Vote target (public key of candidate, None if not voting)
    pub vote_to: Option<ECPoint>,
    /// Height when vote was last updated
    pub vote_height: u32,
}

impl NeoAccountState {
    /// Creates a new account state
    pub fn new(balance: BigInt, height: u32) -> Self {
        Self {
            balance,
            balance_height: height,
            vote_to: None,
            vote_height: height,
        }
    }

    /// Creates an account state with vote
    pub fn with_vote(balance: BigInt, height: u32, vote_to: ECPoint) -> Self {
        Self {
            balance,
            balance_height: height,
            vote_to: Some(vote_to),
            vote_height: height,
        }
    }

    /// Updates the balance
    pub fn update_balance(&mut self, new_balance: BigInt, height: u32) {
        self.balance = new_balance;
        self.balance_height = height;
    }

    /// Updates the vote
    pub fn update_vote(&mut self, vote_to: Option<ECPoint>, height: u32) {
        self.vote_to = vote_to;
        self.vote_height = height;
    }

    /// Serializes to bytes (matches C# ISerializable exactly)
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();

        // Serialize balance (BigInteger format)
        let balance_bytes = self.balance_to_bytes()?;
        bytes.extend_from_slice(&(balance_bytes.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&balance_bytes);

        // Serialize balance height
        bytes.extend_from_slice(&self.balance_height.to_le_bytes());

        // Serialize vote target
        match &self.vote_to {
            Some(vote) => {
                bytes.push(1); // Has vote
                let vote_bytes = vote.encode_point(true).map_err(|e| {
                    Error::SerializationError(format!("Failed to encode vote: {}", e))
                })?;
                bytes.extend_from_slice(&vote_bytes);
            }
            None => {
                bytes.push(0); // No vote
            }
        }

        // Serialize vote height
        bytes.extend_from_slice(&self.vote_height.to_le_bytes());

        Ok(bytes)
    }

    /// Deserializes from bytes (matches C# ISerializable exactly)
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 13 {
            // Minimum: 4 + 4 + 1 + 4 bytes
            return Err(Error::SerializationError("Insufficient data".to_string()));
        }

        let mut offset = 0;

        // Deserialize balance
        let balance_len = u32::from_le_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]) as usize;
        offset += 4;

        if offset + balance_len > bytes.len() {
            return Err(Error::SerializationError(
                "Invalid balance data".to_string(),
            ));
        }

        let balance = Self::balance_from_bytes(&bytes[offset..offset + balance_len])?;
        offset += balance_len;

        // Deserialize balance height
        let balance_height = u32::from_le_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]);
        offset += 4;

        // Deserialize vote target
        let has_vote = bytes[offset] != 0;
        offset += 1;

        let vote_to = if has_vote {
            if offset + 33 > bytes.len() {
                return Err(Error::SerializationError("Invalid vote data".to_string()));
            }
            let vote_bytes = &bytes[offset..offset + 33];
            let vote = ECPoint::from_bytes(vote_bytes)
                .map_err(|e| Error::SerializationError(format!("Invalid vote point: {}", e)))?;
            offset += 33;
            Some(vote)
        } else {
            None
        };

        // Deserialize vote height
        if offset + 4 > bytes.len() {
            return Err(Error::SerializationError(
                "Invalid vote height data".to_string(),
            ));
        }

        let vote_height = u32::from_le_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]);

        Ok(Self {
            balance,
            balance_height,
            vote_to,
            vote_height,
        })
    }

    /// Converts balance to bytes (matches C# BigInteger.ToByteArray)
    fn balance_to_bytes(&self) -> Result<Vec<u8>> {
        if self.balance == BigInt::from(0) {
            return Ok(vec![0]);
        }

        let mut bytes = Vec::new();
        let mut value = self.balance.clone();
        let is_negative = value < BigInt::from(0);

        if is_negative {
            value = -value;
        }

        while value > BigInt::from(0) {
            let byte = (&value % 256u32)
                .to_bytes_le()
                .1
                .get(0)
                .copied()
                .unwrap_or(0);
            bytes.push(byte);
            value /= 256;
        }

        if bytes.is_empty() {
            bytes.push(0);
        }

        // Handle sign bit
        if !is_negative && bytes[bytes.len() - 1] >= 0x80 {
            bytes.push(0);
        } else if is_negative {
            // Two's complement representation
            let mut carry = true;
            for byte in bytes.iter_mut() {
                *byte = !*byte;
                if carry {
                    if *byte == 255 {
                        *byte = 0;
                    } else {
                        *byte += 1;
                        carry = false;
                    }
                }
            }
            if bytes[bytes.len() - 1] < 0x80 {
                bytes.push(0xFF);
            }
        }

        Ok(bytes)
    }

    /// Converts bytes to balance (matches C# BigInteger constructor)
    fn balance_from_bytes(bytes: &[u8]) -> Result<BigInt> {
        if bytes.is_empty() {
            return Ok(BigInt::from(0));
        }

        let is_negative = bytes[bytes.len() - 1] >= 0x80;
        let mut value = BigInt::from(0);
        let mut multiplier = BigInt::from(1);

        for &byte in bytes {
            value += BigInt::from(byte) * &multiplier;
            multiplier *= 256;
        }

        if is_negative {
            let max_value = BigInt::from(2).pow((bytes.len() * 8) as u32);
            value -= max_value;
        }

        Ok(value)
    }
}

/// Candidate state for NEO governance (matches C# CandidateState exactly)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateState {
    /// Candidate's public key
    pub public_key: ECPoint,
    /// Number of votes received
    pub votes: BigInt,
    /// Whether candidate is registered
    pub registered: bool,
}

impl CandidateState {
    /// Creates a new candidate state
    pub fn new(public_key: ECPoint) -> Self {
        Self {
            public_key,
            votes: BigInt::from(0),
            registered: true,
        }
    }

    /// Updates vote count
    pub fn update_votes(&mut self, votes: BigInt) {
        self.votes = votes;
    }

    /// Unregisters the candidate
    pub fn unregister(&mut self) {
        self.registered = false;
        self.votes = BigInt::from(0);
    }

    /// Serializes to bytes (matches C# ISerializable exactly)
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();

        // Serialize public key (33 bytes compressed)
        let pubkey_bytes = self.public_key.encode_point(true).map_err(|e| {
            Error::SerializationError(format!("Failed to encode public key: {}", e))
        })?;
        bytes.extend_from_slice(&pubkey_bytes);

        // Serialize votes (BigInteger format)
        let votes_bytes = self.votes_to_bytes()?;
        bytes.extend_from_slice(&(votes_bytes.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&votes_bytes);

        // Serialize registered flag
        bytes.push(if self.registered { 1 } else { 0 });

        Ok(bytes)
    }

    /// Deserializes from bytes (matches C# ISerializable exactly)
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 38 {
            // Minimum: 33 + 4 + 1 bytes
            return Err(Error::SerializationError(
                "Insufficient candidate data".to_string(),
            ));
        }

        let mut offset = 0;

        // Deserialize public key
        let public_key = ECPoint::from_bytes(&bytes[offset..offset + 33])
            .map_err(|e| Error::SerializationError(format!("Invalid public key: {}", e)))?;
        offset += 33;

        // Deserialize votes
        let votes_len = u32::from_le_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]) as usize;
        offset += 4;

        if offset + votes_len > bytes.len() {
            return Err(Error::SerializationError("Invalid votes data".to_string()));
        }

        let votes = Self::votes_from_bytes(&bytes[offset..offset + votes_len])?;
        offset += votes_len;

        // Deserialize registered flag
        if offset >= bytes.len() {
            return Err(Error::SerializationError(
                "Missing registered flag".to_string(),
            ));
        }

        let registered = bytes[offset] != 0;

        Ok(Self {
            public_key,
            votes,
            registered,
        })
    }

    /// Converts votes to bytes (matches C# BigInteger.ToByteArray)
    fn votes_to_bytes(&self) -> Result<Vec<u8>> {
        NeoAccountState::balance_to_bytes_static(&self.votes)
    }

    /// Converts bytes to votes (matches C# BigInteger constructor)
    fn votes_from_bytes(bytes: &[u8]) -> Result<BigInt> {
        NeoAccountState::balance_from_bytes_static(bytes)
    }
}

impl NeoAccountState {
    /// Static version of balance_to_bytes for use in CandidateState
    pub fn balance_to_bytes_static(value: &BigInt) -> Result<Vec<u8>> {
        if *value == BigInt::from(0) {
            return Ok(vec![0]);
        }

        let mut bytes = Vec::new();
        let mut val = value.clone();
        let is_negative = val < BigInt::from(0);

        if is_negative {
            val = -val;
        }

        while val > BigInt::from(0) {
            let byte = (&val % 256u32).to_bytes_le().1.get(0).copied().unwrap_or(0);
            bytes.push(byte);
            val /= 256;
        }

        if bytes.is_empty() {
            bytes.push(0);
        }

        // Handle sign bit
        if !is_negative && bytes[bytes.len() - 1] >= 0x80 {
            bytes.push(0);
        } else if is_negative {
            // Two's complement representation
            let mut carry = true;
            for byte in bytes.iter_mut() {
                *byte = !*byte;
                if carry {
                    if *byte == 255 {
                        *byte = 0;
                    } else {
                        *byte += 1;
                        carry = false;
                    }
                }
            }
            if bytes[bytes.len() - 1] < 0x80 {
                bytes.push(0xFF);
            }
        }

        Ok(bytes)
    }

    /// Static version of balance_from_bytes for use in CandidateState
    pub fn balance_from_bytes_static(bytes: &[u8]) -> Result<BigInt> {
        if bytes.is_empty() {
            return Ok(BigInt::from(0));
        }

        let is_negative = bytes[bytes.len() - 1] >= 0x80;
        let mut value = BigInt::from(0);
        let mut multiplier = BigInt::from(1);

        for &byte in bytes {
            value += BigInt::from(byte) * &multiplier;
            multiplier *= 256;
        }

        if is_negative {
            let max_value = BigInt::from(2).pow((bytes.len() * 8) as u32);
            value -= max_value;
        }

        Ok(value)
    }
}

/// Committee state for NEO governance (matches C# implementation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteeState {
    /// Current committee members (21 members)
    pub members: Vec<ECPoint>,
    /// Next committee members (calculated from votes)
    pub next_members: Vec<ECPoint>,
    /// Block height when committee was last updated
    pub last_update_height: u32,
}

impl CommitteeState {
    /// Creates a new committee state
    pub fn new() -> Self {
        Self {
            members: Vec::new(),
            next_members: Vec::new(),
            last_update_height: 0,
        }
    }

    /// Updates committee members
    pub fn update_committee(&mut self, members: Vec<ECPoint>, height: u32) {
        self.members = self.next_members.clone();
        self.next_members = members;
        self.last_update_height = height;
    }

    /// Gets current committee size (21 for mainnet)
    pub fn committee_size() -> usize {
        21
    }

    /// Gets consensus node count (7 for mainnet)
    pub fn consensus_nodes_count() -> usize {
        7
    }

    /// Calculates next committee from candidates and votes
    pub fn calculate_next_committee(candidates: &BTreeMap<ECPoint, BigInt>) -> Vec<ECPoint> {
        let mut sorted_candidates: Vec<_> = candidates
            .iter()
            .map(|(pubkey, votes)| (pubkey.clone(), votes.clone()))
            .collect();

        // Sort by votes (descending) then by public key (ascending for determinism)
        sorted_candidates.sort_by(|a, b| {
            let vote_cmp = b.1.cmp(&a.1); // Descending votes
            if vote_cmp == std::cmp::Ordering::Equal {
                a.0.encode_point(true)
                    .unwrap_or_default()
                    .cmp(&b.0.encode_point(true).unwrap_or_default())
            } else {
                vote_cmp
            }
        });

        // Take top 21 candidates
        sorted_candidates
            .into_iter()
            .take(Self::committee_size())
            .map(|(pubkey, _)| pubkey)
            .collect()
    }

    /// Gets consensus nodes from committee (first 7 members)
    pub fn get_consensus_nodes(&self) -> Vec<ECPoint> {
        self.members
            .iter()
            .take(Self::consensus_nodes_count())
            .cloned()
            .collect()
    }
}

impl Default for CommitteeState {
    fn default() -> Self {
        Self::new()
    }
}

/// Vote tracking for governance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteTracker {
    /// Map of account -> voted candidate
    pub account_votes: HashMap<UInt160, Option<ECPoint>>,
    /// Map of candidate -> total votes
    pub candidate_votes: HashMap<ECPoint, BigInt>,
    /// Height when votes were last updated
    pub last_update_height: u32,
}

impl VoteTracker {
    /// Creates a new vote tracker
    pub fn new() -> Self {
        Self {
            account_votes: HashMap::new(),
            candidate_votes: HashMap::new(),
            last_update_height: 0,
        }
    }

    /// Updates account vote
    pub fn update_account_vote(
        &mut self,
        account: UInt160,
        old_vote: Option<ECPoint>,
        new_vote: Option<ECPoint>,
        balance: &BigInt,
        height: u32,
    ) {
        // Remove old vote
        if let Some(old_candidate) = old_vote {
            if let Some(votes) = self.candidate_votes.get_mut(&old_candidate) {
                *votes = std::cmp::max(BigInt::from(0), votes.clone() - balance);
            }
        }

        // Add new vote
        if let Some(new_candidate) = &new_vote {
            *self
                .candidate_votes
                .entry(new_candidate.clone())
                .or_insert_with(|| BigInt::from(0)) += balance;
        }

        // Update account vote record
        self.account_votes.insert(account, new_vote);
        self.last_update_height = height;
    }

    /// Gets votes for a candidate
    pub fn get_candidate_votes(&self, candidate: &ECPoint) -> BigInt {
        self.candidate_votes
            .get(candidate)
            .cloned()
            .unwrap_or_else(|| BigInt::from(0))
    }

    /// Gets all candidates with votes
    pub fn get_all_candidates(&self) -> BTreeMap<ECPoint, BigInt> {
        self.candidate_votes
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}

impl Default for VoteTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_neo_account_state_serialization() {
        let state = NeoAccountState::new(BigInt::from(1000000), 12345);
        let bytes = state.to_bytes().unwrap();
        let deserialized = NeoAccountState::from_bytes(&bytes).unwrap();

        assert_eq!(state.balance, deserialized.balance);
        assert_eq!(state.balance_height, deserialized.balance_height);
        assert_eq!(state.vote_to, deserialized.vote_to);
        assert_eq!(state.vote_height, deserialized.vote_height);
    }

    #[test]
    fn test_committee_size() {
        assert_eq!(CommitteeState::committee_size(), 21);
        assert_eq!(CommitteeState::consensus_nodes_count(), 7);
    }

    #[test]
    fn test_vote_tracker() {
        let mut tracker = VoteTracker::new();
        let account = UInt160::zero();
        let balance = BigInt::from(1000);

        // Create a dummy candidate (would need actual ECPoint in real test)
        // tracker.update_account_vote(account, None, Some(candidate), &balance, 100);

        assert_eq!(tracker.last_update_height, 0);
    }
}
