using System;
using System.Threading.Tasks;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Cryptography;

namespace EpicChain.Consensus
{
    /// <summary>
    /// Represents a validator in the consensus process.
    /// </summary>
    public interface IValidator
    {
        /// <summary>
        /// Gets a value indicating whether this instance is a validator.
        /// </summary>
        bool IsValidator { get; }

        /// <summary>
        /// Gets the public key of the validator.
        /// </summary>
        ECPoint PublicKey { get; }

        /// <summary>
        /// Gets the address of the validator.
        /// </summary>
        UInt160 Address { get; }

        /// <summary>
        /// Gets the voting power of the validator.
        /// </summary>
        long VotingPower { get; }

        /// <summary>
        /// Validates a block proposal.
        /// </summary>
        /// <param name="block">The block to validate.</param>
        /// <returns>True if the block is valid, false otherwise.</returns>
        Task<bool> ValidateBlockAsync(Block block);

        /// <summary>
        /// Validates a vote from another validator.
        /// </summary>
        /// <param name="vote">The vote to validate.</param>
        /// <returns>True if the vote is valid, false otherwise.</returns>
        Task<bool> ValidateVoteAsync(ConsensusVote vote);

        /// <summary>
        /// Creates a vote for a block proposal.
        /// </summary>
        /// <param name="block">The block to vote for.</param>
        /// <returns>The created vote.</returns>
        Task<ConsensusVote> CreateVoteAsync(Block block);

        /// <summary>
        /// Signs a message with the validator's private key.
        /// </summary>
        /// <param name="message">The message to sign.</param>
        /// <returns>The signature.</returns>
        byte[] Sign(byte[] message);

        /// <summary>
        /// Verifies a signature from another validator.
        /// </summary>
        /// <param name="message">The message that was signed.</param>
        /// <param name="signature">The signature to verify.</param>
        /// <param name="publicKey">The public key of the validator that signed the message.</param>
        /// <returns>True if the signature is valid, false otherwise.</returns>
        bool VerifySignature(byte[] message, byte[] signature, ECPoint publicKey);

        /// <summary>
        /// Updates the validator's voting power.
        /// </summary>
        /// <param name="newPower">The new voting power.</param>
        Task UpdateVotingPowerAsync(long newPower);

        /// <summary>
        /// Checks if the validator is eligible to propose a block in the current view.
        /// </summary>
        /// <param name="viewNumber">The current view number.</param>
        /// <returns>True if the validator is eligible, false otherwise.</returns>
        bool IsEligibleForProposal(uint viewNumber);
    }
} 