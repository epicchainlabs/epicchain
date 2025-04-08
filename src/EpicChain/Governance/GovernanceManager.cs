using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using EpicChain.SmartContract;
using EpicChain.Network.P2P.Payloads;

namespace EpicChain.Governance
{
    /// <summary>
    /// Manages on-chain governance operations including proposal creation, voting, and execution.
    /// </summary>
    public class GovernanceManager
    {
        private readonly ProtocolSettings _settings;
        private readonly ISmartContractService _contractService;
        private readonly IGovernanceStorage _storage;

        public GovernanceManager(ProtocolSettings settings, ISmartContractService contractService, IGovernanceStorage storage)
        {
            _settings = settings;
            _contractService = contractService;
            _storage = storage;
        }

        /// <summary>
        /// Creates a new governance proposal.
        /// </summary>
        /// <param name="proposer">The address of the proposal creator.</param>
        /// <param name="title">The title of the proposal.</param>
        /// <param name="description">The detailed description of the proposal.</param>
        /// <param name="executionScript">The script to execute if the proposal is approved.</param>
        /// <returns>The created proposal.</returns>
        public async Task<GovernanceProposal> CreateProposalAsync(UInt160 proposer, string title, string description, byte[] executionScript)
        {
            var proposal = new GovernanceProposal
            {
                Id = Guid.NewGuid(),
                Proposer = proposer,
                Title = title,
                Description = description,
                ExecutionScript = executionScript,
                CreationTime = DateTime.UtcNow,
                Status = ProposalStatus.Active
            };

            await _storage.StoreProposalAsync(proposal);
            return proposal;
        }

        /// <summary>
        /// Casts a vote on a governance proposal.
        /// </summary>
        /// <param name="voter">The address of the voter.</param>
        /// <param name="proposalId">The ID of the proposal.</param>
        /// <param name="vote">The vote (Yes/No/Abstain).</param>
        public async Task CastVoteAsync(UInt160 voter, Guid proposalId, VoteType vote)
        {
            var proposal = await _storage.GetProposalAsync(proposalId);
            if (proposal == null || proposal.Status != ProposalStatus.Active)
                throw new InvalidOperationException("Proposal not found or not active");

            var voteRecord = new GovernanceVote
            {
                ProposalId = proposalId,
                Voter = voter,
                Vote = vote,
                Timestamp = DateTime.UtcNow
            };

            await _storage.StoreVoteAsync(voteRecord);
            await UpdateProposalStatusAsync(proposal);
        }

        /// <summary>
        /// Executes a proposal that has been approved.
        /// </summary>
        /// <param name="proposalId">The ID of the proposal to execute.</param>
        public async Task ExecuteProposalAsync(Guid proposalId)
        {
            var proposal = await _storage.GetProposalAsync(proposalId);
            if (proposal == null || proposal.Status != ProposalStatus.Approved)
                throw new InvalidOperationException("Proposal not found or not approved");

            await _contractService.ExecuteScriptAsync(proposal.ExecutionScript);
            proposal.Status = ProposalStatus.Executed;
            await _storage.StoreProposalAsync(proposal);
        }

        private async Task UpdateProposalStatusAsync(GovernanceProposal proposal)
        {
            var votes = await _storage.GetVotesAsync(proposal.Id);
            var votingPower = await CalculateVotingPowerAsync(votes);

            if (votingPower.YesVotes >= _settings.GovernanceApprovalThreshold)
            {
                proposal.Status = ProposalStatus.Approved;
                await _storage.StoreProposalAsync(proposal);
            }
            else if (votingPower.NoVotes >= _settings.GovernanceRejectionThreshold)
            {
                proposal.Status = ProposalStatus.Rejected;
                await _storage.StoreProposalAsync(proposal);
            }
        }

        private async Task<VotingPower> CalculateVotingPowerAsync(IEnumerable<GovernanceVote> votes)
        {
            var result = new VotingPower();
            foreach (var vote in votes)
            {
                var power = await _contractService.GetVotingPowerAsync(vote.Voter);
                switch (vote.Vote)
                {
                    case VoteType.Yes:
                        result.YesVotes += power;
                        break;
                    case VoteType.No:
                        result.NoVotes += power;
                        break;
                }
            }
            return result;
        }
    }
} 