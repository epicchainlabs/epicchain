using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Cryptography;

namespace EpicChain.Consensus
{
    /// <summary>
    /// Represents the consensus engine that manages the blockchain's consensus mechanism.
    /// This class handles block validation, voting, and consensus state management.
    /// </summary>
    public class ConsensusEngine
    {
        private readonly ProtocolSettings _settings;
        private readonly IConsensusState _state;
        private readonly IValidator _validator;
        private readonly IBlockProducer _blockProducer;
        private readonly Dictionary<UInt160, IValidator> _validators;
        private bool _isRunning;
        private DateTime _lastBlockTime;

        public ConsensusEngine(ProtocolSettings settings, IConsensusState state, IValidator validator, IBlockProducer blockProducer)
        {
            _settings = settings;
            _state = state;
            _validator = validator;
            _blockProducer = blockProducer;
            _validators = new Dictionary<UInt160, IValidator>();
            _isRunning = false;
            _lastBlockTime = DateTime.UtcNow;
        }

        /// <summary>
        /// Initializes the consensus engine with the current blockchain state.
        /// </summary>
        public async Task InitializeAsync()
        {
            await _state.InitializeAsync();
            _lastBlockTime = _state.GetLastBlockTimestamp();
            _isRunning = true;
        }

        /// <summary>
        /// Starts the consensus process.
        /// </summary>
        public async Task StartAsync()
        {
            if (!_validator.IsValidator)
                return;

            while (_isRunning)
            {
                try
                {
                    await ProcessConsensusRoundAsync();
                }
                catch (Exception ex)
                {
                    // Log the exception and continue
                    Console.WriteLine($"Consensus error: {ex.Message}");
                }

                await Task.Delay(_settings.ConsensusInterval);
            }
        }

        /// <summary>
        /// Stops the consensus process.
        /// </summary>
        public void Stop()
        {
            _isRunning = false;
        }

        /// <summary>
        /// Processes a single consensus round.
        /// </summary>
        private async Task ProcessConsensusRoundAsync()
        {
            if (_validator.IsEligibleForProposal(_state.ViewNumber))
            {
                var block = await _blockProducer.ProduceBlockAsync();
                if (block != null)
                {
                    await ProcessBlockProposalAsync(block);
                }
            }

            if (_state.CanCommit())
            {
                await _state.CommitAsync();
                await _state.ResetAsync();
            }
            else if (DateTime.UtcNow - _lastBlockTime > _state.GetTimeout())
            {
                await _state.ChangeViewAsync();
            }
        }

        /// <summary>
        /// Processes a new block proposal from a validator.
        /// </summary>
        /// <param name="block">The proposed block.</param>
        /// <returns>True if the block is valid and accepted, false otherwise.</returns>
        public async Task<bool> ProcessBlockProposalAsync(Block block)
        {
            if (!await _validator.ValidateBlockAsync(block))
                return false;

            if (!await _state.ProcessBlockAsync(block))
                return false;

            if (_validator.IsValidator)
            {
                var vote = await _validator.CreateVoteAsync(block);
                await ProcessVoteAsync(vote);
            }

            return true;
        }

        /// <summary>
        /// Handles a vote from a validator.
        /// </summary>
        /// <param name="vote">The vote to process.</param>
        public async Task ProcessVoteAsync(ConsensusVote vote)
        {
            if (!await _validator.ValidateVoteAsync(vote))
                return;

            await _state.ProcessVoteAsync(vote);
        }

        /// <summary>
        /// Registers a new validator.
        /// </summary>
        /// <param name="validator">The validator to register.</param>
        public void RegisterValidator(IValidator validator)
        {
            _validators[validator.Address] = validator;
            _state.UpdateValidatorsAsync(_validators.Keys.ToList());
        }

        /// <summary>
        /// Unregisters a validator.
        /// </summary>
        /// <param name="address">The address of the validator to unregister.</param>
        public void UnregisterValidator(UInt160 address)
        {
            _validators.Remove(address);
            _state.UpdateValidatorsAsync(_validators.Keys.ToList());
        }

        /// <summary>
        /// Updates the voting power of a validator.
        /// </summary>
        /// <param name="address">The address of the validator.</param>
        /// <param name="newPower">The new voting power.</param>
        public async Task UpdateValidatorPowerAsync(UInt160 address, long newPower)
        {
            if (_validators.TryGetValue(address, out var validator))
            {
                await validator.UpdateVotingPowerAsync(newPower);
            }
        }
    }
} 