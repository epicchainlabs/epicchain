using System;
using System.Threading.Tasks;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Ledger;

namespace EpicChain.Consensus
{
    /// <summary>
    /// Represents a block producer in the consensus process.
    /// </summary>
    public interface IBlockProducer
    {
        /// <summary>
        /// Produces a new block for the current consensus round.
        /// </summary>
        /// <returns>The produced block, or null if no block could be produced.</returns>
        Task<Block?> ProduceBlockAsync();

        /// <summary>
        /// Gets the current block index.
        /// </summary>
        uint CurrentIndex { get; }

        /// <summary>
        /// Gets the timestamp of the last produced block.
        /// </summary>
        DateTime LastBlockTime { get; }

        /// <summary>
        /// Gets the current block time.
        /// </summary>
        DateTime CurrentBlockTime { get; }

        /// <summary>
        /// Initializes the block producer.
        /// </summary>
        Task InitializeAsync();

        /// <summary>
        /// Updates the block producer's state.
        /// </summary>
        /// <param name="index">The current block index.</param>
        /// <param name="timestamp">The current timestamp.</param>
        Task UpdateStateAsync(uint index, DateTime timestamp);

        /// <summary>
        /// Verifies a block produced by another validator.
        /// </summary>
        /// <param name="block">The block to verify.</param>
        /// <returns>True if the block is valid, false otherwise.</returns>
        Task<bool> VerifyBlockAsync(Block block);

        /// <summary>
        /// Gets the maximum block size.
        /// </summary>
        uint MaxBlockSize { get; }

        /// <summary>
        /// Gets the maximum block system fee.
        /// </summary>
        long MaxBlockSystemFee { get; }

        /// <summary>
        /// Gets the maximum block transactions.
        /// </summary>
        uint MaxBlockTransactions { get; }

        /// <summary>
        /// Gets the minimum block time.
        /// </summary>
        TimeSpan MinBlockTime { get; }

        /// <summary>
        /// Gets the maximum block time.
        /// </summary>
        TimeSpan MaxBlockTime { get; }
    }
} 