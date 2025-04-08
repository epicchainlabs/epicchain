using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using EpicChain.Network.P2P.Payloads;

namespace EpicChain.Consensus
{
    /// <summary>
    /// Represents the state of the consensus process, including validators, votes, and block proposals.
    /// </summary>
    public interface IConsensusState
    {
        /// <summary>
        /// Gets the current view number of the consensus process.
        /// </summary>
        uint ViewNumber { get; }

        /// <summary>
        /// Gets the current primary validator.
        /// </summary>
        UInt160 PrimaryValidator { get; }

        /// <summary>
        /// Gets the list of validators for the current view.
        /// </summary>
        IReadOnlyList<UInt160> Validators { get; }

        /// <summary>
        /// Gets the current block being proposed.
        /// </summary>
        Block? CurrentBlock { get; }

        /// <summary>
        /// Initializes the consensus state.
        /// </summary>
        Task InitializeAsync();

        /// <summary>
        /// Processes a new block proposal.
        /// </summary>
        /// <param name="block">The proposed block.</param>
        /// <returns>True if the block is accepted, false otherwise.</returns>
        Task<bool> ProcessBlockAsync(Block block);

        /// <summary>
        /// Processes a vote from a validator.
        /// </summary>
        /// <param name="vote">The vote to process.</param>
        /// <returns>True if the vote is accepted, false otherwise.</returns>
        Task<bool> ProcessVoteAsync(ConsensusVote vote);

        /// <summary>
        /// Changes the view to the next view number.
        /// </summary>
        /// <returns>True if the view change is successful, false otherwise.</returns>
        Task<bool> ChangeViewAsync();

        /// <summary>
        /// Gets the number of votes received for the current block.
        /// </summary>
        /// <returns>The number of votes.</returns>
        int GetVoteCount();

        /// <summary>
        /// Checks if the current block has received enough votes to be committed.
        /// </summary>
        /// <returns>True if the block can be committed, false otherwise.</returns>
        bool CanCommit();

        /// <summary>
        /// Commits the current block to the blockchain.
        /// </summary>
        /// <returns>True if the commit is successful, false otherwise.</returns>
        Task<bool> CommitAsync();

        /// <summary>
        /// Resets the consensus state for a new round.
        /// </summary>
        Task ResetAsync();

        /// <summary>
        /// Gets the timestamp of the last block.
        /// </summary>
        /// <returns>The timestamp.</returns>
        DateTime GetLastBlockTimestamp();

        /// <summary>
        /// Gets the current consensus timeout.
        /// </summary>
        /// <returns>The timeout duration.</returns>
        TimeSpan GetTimeout();

        /// <summary>
        /// Updates the list of validators for the next view.
        /// </summary>
        /// <param name="validators">The new list of validators.</param>
        Task UpdateValidatorsAsync(IReadOnlyList<UInt160> validators);
    }
} 