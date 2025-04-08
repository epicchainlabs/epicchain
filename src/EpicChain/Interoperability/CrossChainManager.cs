using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Cryptography;

namespace EpicChain.Interoperability
{
    /// <summary>
    /// Manages cross-chain communication and asset transfers between different blockchain networks.
    /// </summary>
    public class CrossChainManager
    {
        private readonly ICrossChainStorage _storage;
        private readonly ICrossChainVerifier _verifier;
        private readonly Dictionary<UInt256, ICrossChainClient> _clients;

        public CrossChainManager(ICrossChainStorage storage, ICrossChainVerifier verifier)
        {
            _storage = storage;
            _verifier = verifier;
            _clients = new Dictionary<UInt256, ICrossChainClient>();
        }

        /// <summary>
        /// Registers a new cross-chain client for a specific blockchain.
        /// </summary>
        /// <param name="chainId">The ID of the blockchain.</param>
        /// <param name="client">The cross-chain client implementation.</param>
        public void RegisterClient(UInt256 chainId, ICrossChainClient client)
        {
            _clients[chainId] = client;
        }

        /// <summary>
        /// Initiates a cross-chain asset transfer.
        /// </summary>
        /// <param name="sourceChain">The source blockchain ID.</param>
        /// <param name="targetChain">The target blockchain ID.</param>
        /// <param name="assetId">The ID of the asset to transfer.</param>
        /// <param name="amount">The amount to transfer.</param>
        /// <param name="recipient">The recipient's address on the target chain.</param>
        /// <returns>The cross-chain transfer transaction.</returns>
        public async Task<CrossChainTransfer> CreateTransferAsync(
            UInt256 sourceChain,
            UInt256 targetChain,
            UInt256 assetId,
            ulong amount,
            byte[] recipient)
        {
            if (!_clients.ContainsKey(sourceChain) || !_clients.ContainsKey(targetChain))
                throw new InvalidOperationException("Chain not supported");

            var transfer = new CrossChainTransfer
            {
                Id = Guid.NewGuid(),
                SourceChain = sourceChain,
                TargetChain = targetChain,
                AssetId = assetId,
                Amount = amount,
                Recipient = recipient,
                Status = CrossChainTransferStatus.Pending,
                CreationTime = DateTime.UtcNow
            };

            await _storage.StoreTransferAsync(transfer);
            return transfer;
        }

        /// <summary>
        /// Verifies and processes a cross-chain transfer.
        /// </summary>
        /// <param name="transferId">The ID of the transfer to process.</param>
        public async Task ProcessTransferAsync(Guid transferId)
        {
            var transfer = await _storage.GetTransferAsync(transferId);
            if (transfer == null || transfer.Status != CrossChainTransferStatus.Pending)
                throw new InvalidOperationException("Transfer not found or already processed");

            var sourceClient = _clients[transfer.SourceChain];
            var targetClient = _clients[transfer.TargetChain];

            // Verify the transfer on the source chain
            var verification = await _verifier.VerifyTransferAsync(transfer);
            if (!verification.IsValid)
            {
                transfer.Status = CrossChainTransferStatus.Failed;
                await _storage.StoreTransferAsync(transfer);
                return;
            }

            // Execute the transfer on the target chain
            try
            {
                await targetClient.ExecuteTransferAsync(transfer);
                transfer.Status = CrossChainTransferStatus.Completed;
            }
            catch (Exception)
            {
                transfer.Status = CrossChainTransferStatus.Failed;
            }

            await _storage.StoreTransferAsync(transfer);
        }

        /// <summary>
        /// Retrieves the status of a cross-chain transfer.
        /// </summary>
        /// <param name="transferId">The ID of the transfer.</param>
        /// <returns>The current status of the transfer.</returns>
        public async Task<CrossChainTransferStatus> GetTransferStatusAsync(Guid transferId)
        {
            var transfer = await _storage.GetTransferAsync(transferId);
            return transfer?.Status ?? CrossChainTransferStatus.Unknown;
        }
    }
} 