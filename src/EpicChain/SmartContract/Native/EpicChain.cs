// Copyright (C) 2021-2024 EpicChain Labs.

//
// EpicChain.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
// distributed as free software under the MIT License, allowing for wide usage and modification
// with minimal restrictions. For comprehensive details regarding the license, please refer to
// the LICENSE file located in the root directory of the repository or visit
// http://www.opensource.org/licenses/mit-license.php.
//
// EpicChain Labs is dedicated to fostering innovation and development in the blockchain space,
// and we believe in the open-source philosophy as a way to drive progress and collaboration.
// This file, along with all associated code and documentation, is provided with the intention of
// supporting and enhancing the development community.
//
// Redistribution and use of this file in both source and binary forms, with or without
// modifications, are permitted. We encourage users to contribute to the project and respect the
// guidelines outlined in the LICENSE file. By using this software, you agree to the terms and
// conditions specified in the MIT License, ensuring the continuation of free and open software
// practices.


#pragma warning disable IDE0051

using EpicChain.Cryptography.ECC;
using EpicChain.IO;
using EpicChain.Persistence;
using EpicChain.SmartContract.Iterators;
using EpicChain.VM;
using EpicChain.VM.Types;
using System;
using System.Buffers.Binary;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;

namespace EpicChain.SmartContract.Native
{
    /// <summary>
    /// Represents the EpicChain token in the EpicChain system.
    /// </summary>
    public sealed class EpicChain : FungibleToken<EpicChain.EpicChainAccountState>
    {
        public override string Symbol => "XPR";
        public override byte Decimals => 0;

        /// <summary>
        /// Indicates the total amount of EpicChain.
        /// </summary>
        public BigInteger TotalAmount { get; }

        /// <summary>
        /// Indicates the effective voting turnout in EpicChain. The voted candidates will only be effective when the voting turnout exceeds this value.
        /// </summary>
        public const decimal EffectiveVoterTurnout = 0.2M;

        private const byte Prefix_VotersCount = 1;
        private const byte Prefix_Candidate = 33;
        private const byte Prefix_Committee = 14;
        private const byte Prefix_epicpulsePerBlock = 29;
        private const byte Prefix_RegisterPrice = 13;
        private const byte Prefix_VoterRewardPerCommittee = 23;

        private const byte epicchainHolderRewardRatio = 10;
        private const byte CommitteeRewardRatio = 10;
        private const byte VoterRewardRatio = 80;

        [ContractEvent(1, name: "CandidateStateChanged",
           "pubkey", ContractParameterType.PublicKey,
           "registered", ContractParameterType.Boolean,
           "votes", ContractParameterType.Integer)]
        [ContractEvent(2, name: "Vote",
           "account", ContractParameterType.Hash160,
           "from", ContractParameterType.PublicKey,
           "to", ContractParameterType.PublicKey,
           "amount", ContractParameterType.Integer)]
        [ContractEvent(Hardfork.HF_Cockatrice, 3, name: "CommitteeChanged",
           "old", ContractParameterType.Array,
           "new", ContractParameterType.Array)]
        internal EpicChain() : base()
        {
            TotalAmount = 1000000000 * Factor;
        }

        public override BigInteger TotalSupply(DataCache snapshot)
        {
            return TotalAmount;
        }

        internal override void OnBalanceChanging(ApplicationEngine engine, UInt160 account, EpicChainAccountState state, BigInteger amount)
        {
            EpicPulseDistribution distribution = DistributeEpicPulse(engine, account, state);
            if (distribution is not null)
            {
                var list = engine.CurrentContext.GetState<List<EpicPulseDistribution>>();
                list.Add(distribution);
            }
            if (amount.IsZero) return;
            if (state.VoteTo is null) return;
            engine.SnapshotCache.GetAndChange(CreateStorageKey(Prefix_VotersCount)).Add(amount);
            StorageKey key = CreateStorageKey(Prefix_Candidate).Add(state.VoteTo);
            CandidateState candidate = engine.SnapshotCache.GetAndChange(key).GetInteroperable<CandidateState>();
            candidate.Votes += amount;
            CheckCandidate(engine.SnapshotCache, state.VoteTo, candidate);
        }

        private protected override async ContractTask PostTransferAsync(ApplicationEngine engine, UInt160 from, UInt160 to, BigInteger amount, StackItem data, bool callOnPayment)
        {
            await base.PostTransferAsync(engine, from, to, amount, data, callOnPayment);
            var list = engine.CurrentContext.GetState<List<EpicPulseDistribution>>();
            foreach (var distribution in list)
                await EpicPulse.Mint(engine, distribution.Account, distribution.Amount, callOnPayment);
        }

        private EpicPulseDistribution DistributeEpicPulse(ApplicationEngine engine, UInt160 account, EpicChainAccountState state)
        {
            // PersistingBlock is null when running under the debugger
            if (engine.PersistingBlock is null) return null;

            // In the unit of datoshi, 1 datoshi = 1e-8 EpicPulse
            BigInteger datoshi = CalculateBonus(engine.SnapshotCache, state, engine.PersistingBlock.Index);
            state.BalanceHeight = engine.PersistingBlock.Index;
            if (state.VoteTo is not null)
            {
                var keyLastest = CreateStorageKey(Prefix_VoterRewardPerCommittee).Add(state.VoteTo);
                var latestEpicPulsePerVote = engine.SnapshotCache.TryGet(keyLastest) ?? BigInteger.Zero;
                state.LastEpicPulsePerVote = latestEpicPulsePerVote;
            }
            if (datoshi == 0) return null;
            return new EpicPulseDistribution
            {
                Account = account,
                Amount = datoshi
            };
        }

        private BigInteger CalculateBonus(DataCache snapshot, EpicChainAccountState state, uint end)
        {
            if (state.Balance.IsZero) return BigInteger.Zero;
            if (state.Balance.Sign < 0) throw new ArgumentOutOfRangeException(nameof(state.Balance));

            var expectEnd = Ledger.CurrentIndex(snapshot) + 1;
            if (expectEnd != end) throw new ArgumentOutOfRangeException(nameof(end));
            if (state.BalanceHeight >= end) return BigInteger.Zero;
            // In the unit of datoshi, 1 datoshi = 1e-8 EpicPulse
            BigInteger epicchainHolderReward = CalculateEpicChainHolderReward(snapshot, state.Balance, state.BalanceHeight, end);
            if (state.VoteTo is null) return epicchainHolderReward;

            var keyLastest = CreateStorageKey(Prefix_VoterRewardPerCommittee).Add(state.VoteTo);
            var latestEpicPulsePerVote = snapshot.TryGet(keyLastest) ?? BigInteger.Zero;
            var voteReward = state.Balance * (latestEpicPulsePerVote - state.LastEpicPulsePerVote) / 100000000L;

            return epicchainHolderReward + voteReward;
        }

        private BigInteger CalculateEpicChainHolderReward(DataCache snapshot, BigInteger value, uint start, uint end)
        {
            // In the unit of datoshi, 1 EpicPulse = 10^8 datoshi
            BigInteger sum = 0;
            foreach (var (index, epicpulsePerBlock) in GetSortedEpicPulseRecords(snapshot, end - 1))
            {
                if (index > start)
                {
                    sum += epicpulsePerBlock * (end - index);
                    end = index;
                }
                else
                {
                    sum += epicpulsePerBlock * (end - start);
                    break;
                }
            }
            return value * sum * epicchainHolderRewardRatio / 100 / TotalAmount;
        }

        private void CheckCandidate(DataCache snapshot, ECPoint pubkey, CandidateState candidate)
        {
            if (!candidate.Registered && candidate.Votes.IsZero)
            {
                snapshot.Delete(CreateStorageKey(Prefix_VoterRewardPerCommittee).Add(pubkey));
                snapshot.Delete(CreateStorageKey(Prefix_Candidate).Add(pubkey));
            }
        }

        /// <summary>
        /// Determine whether the votes should be recounted at the specified height.
        /// </summary>
        /// <param name="height">The height to be checked.</param>
        /// <param name="committeeMembersCount">The number of committee members in the system.</param>
        /// <returns><see langword="true"/> if the votes should be recounted; otherwise, <see langword="false"/>.</returns>
        public static bool ShouldRefreshCommittee(uint height, int committeeMembersCount) => height % committeeMembersCount == 0;

        internal override ContractTask InitializeAsync(ApplicationEngine engine, Hardfork? hardfork)
        {
            if (hardfork == ActiveIn)
            {
                var cachedCommittee = new CachedCommittee(engine.ProtocolSettings.StandbyCommittee.Select(p => (p, BigInteger.Zero)));
                engine.SnapshotCache.Add(CreateStorageKey(Prefix_Committee), new StorageItem(cachedCommittee));
                engine.SnapshotCache.Add(CreateStorageKey(Prefix_VotersCount), new StorageItem(System.Array.Empty<byte>()));
                engine.SnapshotCache.Add(CreateStorageKey(Prefix_epicpulsePerBlock).AddBigEndian(0u), new StorageItem(5 * EpicPulse.Factor));
                engine.SnapshotCache.Add(CreateStorageKey(Prefix_RegisterPrice), new StorageItem(1000 * EpicPulse.Factor));
                return Mint(engine, Contract.GetBFTAddress(engine.ProtocolSettings.StandbyValidators), TotalAmount, false);
            }
            return ContractTask.CompletedTask;
        }

        internal override ContractTask OnPersistAsync(ApplicationEngine engine)
        {
            // Set next committee
            if (ShouldRefreshCommittee(engine.PersistingBlock.Index, engine.ProtocolSettings.CommitteeMembersCount))
            {
                var storageItem = engine.SnapshotCache.GetAndChange(CreateStorageKey(Prefix_Committee));
                var cachedCommittee = storageItem.GetInteroperable<CachedCommittee>();

                var prevCommittee = cachedCommittee.Select(u => u.PublicKey).ToArray();

                cachedCommittee.Clear();
                cachedCommittee.AddRange(ComputeCommitteeMembers(engine.SnapshotCache, engine.ProtocolSettings));

                // Hardfork check for https://github.com/epicchainlabs/epicchain/pull/3158
                // New notification will case 3.7.0 and 3.6.0 have different behavior
                var index = engine.PersistingBlock?.Index ?? Ledger.CurrentIndex(engine.SnapshotCache);
                if (engine.ProtocolSettings.IsHardforkEnabled(Hardfork.HF_Cockatrice, index))
                {
                    var newCommittee = cachedCommittee.Select(u => u.PublicKey).ToArray();

                    if (!newCommittee.SequenceEqual(prevCommittee))
                    {
                        engine.SendNotification(Hash, "CommitteeChanged", new VM.Types.Array(engine.ReferenceCounter) {
                            new VM.Types.Array(engine.ReferenceCounter, prevCommittee.Select(u => (ByteString)u.ToArray())) ,
                            new VM.Types.Array(engine.ReferenceCounter, newCommittee.Select(u => (ByteString)u.ToArray()))
                        });
                    }
                }
            }
            return ContractTask.CompletedTask;
        }

        internal override async ContractTask PostPersistAsync(ApplicationEngine engine)
        {
            // Distribute EpicPulse for committee

            int m = engine.ProtocolSettings.CommitteeMembersCount;
            int n = engine.ProtocolSettings.ValidatorsCount;
            int index = (int)(engine.PersistingBlock.Index % (uint)m);
            var epicpulsePerBlock = GetepicpulsePerBlock(engine.SnapshotCache);
            var committee = GetCommitteeFromCache(engine.SnapshotCache);
            var pubkey = committee[index].PublicKey;
            var account = Contract.CreateSignatureRedeemScript(pubkey).ToScriptHash();
            await EpicPulse.Mint(engine, account, epicpulsePerBlock * CommitteeRewardRatio / 100, false);

            // Record the cumulative reward of the voters of committee

            if (ShouldRefreshCommittee(engine.PersistingBlock.Index, m))
            {
                BigInteger voterRewardOfEachCommittee = epicpulsePerBlock * VoterRewardRatio * 100000000L * m / (m + n) / 100; // Zoom in 100000000 times, and the final calculation should be divided 100000000L
                for (index = 0; index < committee.Count; index++)
                {
                    var (PublicKey, Votes) = committee[index];
                    var factor = index < n ? 2 : 1; // The `voter` rewards of validator will double than other committee's
                    if (Votes > 0)
                    {
                        BigInteger voterSumRewardPerEpicChain = factor * voterRewardOfEachCommittee / Votes;
                        StorageKey voterRewardKey = CreateStorageKey(Prefix_VoterRewardPerCommittee).Add(PublicKey);
                        StorageItem lastRewardPerEpicChain = engine.SnapshotCache.GetAndChange(voterRewardKey, () => new StorageItem(BigInteger.Zero));
                        lastRewardPerEpicChain.Add(voterSumRewardPerEpicChain);
                    }
                }
            }
        }

        [ContractMethod(CpuFee = 1 << 15, RequiredCallFlags = CallFlags.States)]
        private void SetepicpulsePerBlock(ApplicationEngine engine, BigInteger epicpulsePerBlock)
        {
            if (epicpulsePerBlock < 0 || epicpulsePerBlock > 10 * EpicPulse.Factor)
                throw new ArgumentOutOfRangeException(nameof(epicpulsePerBlock));
            if (!CheckCommittee(engine)) throw new InvalidOperationException();

            uint index = engine.PersistingBlock.Index + 1;
            StorageItem entry = engine.SnapshotCache.GetAndChange(CreateStorageKey(Prefix_epicpulsePerBlock).AddBigEndian(index), () => new StorageItem(epicpulsePerBlock));
            entry.Set(epicpulsePerBlock);
        }

        /// <summary>
        /// Gets the amount of EpicPulse generated in each block.
        /// </summary>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <returns>The amount of EpicPulse generated.</returns>
        [ContractMethod(CpuFee = 1 << 15, RequiredCallFlags = CallFlags.ReadStates)]
        public BigInteger GetepicpulsePerBlock(DataCache snapshot)
        {
            return GetSortedEpicPulseRecords(snapshot, Ledger.CurrentIndex(snapshot) + 1).First().epicpulsePerBlock;
        }

        [ContractMethod(CpuFee = 1 << 15, RequiredCallFlags = CallFlags.States)]
        private void SetRegisterPrice(ApplicationEngine engine, long registerPrice)
        {
            if (registerPrice <= 0)
                throw new ArgumentOutOfRangeException(nameof(registerPrice));
            if (!CheckCommittee(engine)) throw new InvalidOperationException();
            engine.SnapshotCache.GetAndChange(CreateStorageKey(Prefix_RegisterPrice)).Set(registerPrice);
        }

        /// <summary>
        /// Gets the fees to be paid to register as a candidate.
        /// </summary>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <returns>The amount of the fees.</returns>
        [ContractMethod(CpuFee = 1 << 15, RequiredCallFlags = CallFlags.ReadStates)]
        public long GetRegisterPrice(DataCache snapshot)
        {
            // In the unit of datoshi, 1 datoshi = 1e-8 EpicPulse
            return (long)(BigInteger)snapshot[CreateStorageKey(Prefix_RegisterPrice)];
        }

        private IEnumerable<(uint Index, BigInteger epicpulsePerBlock)> GetSortedEpicPulseRecords(DataCache snapshot, uint end)
        {
            byte[] key = CreateStorageKey(Prefix_epicpulsePerBlock).AddBigEndian(end).ToArray();
            byte[] boundary = CreateStorageKey(Prefix_epicpulsePerBlock).ToArray();
            return snapshot.FindRange(key, boundary, SeekDirection.Backward)
                .Select(u => (BinaryPrimitives.ReadUInt32BigEndian(u.Key.Key.Span[^sizeof(uint)..]), (BigInteger)u.Value));
        }

        /// <summary>
        /// Get the amount of unclaimed EpicPulse in the specified account.
        /// </summary>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <param name="account">The account to check.</param>
        /// <param name="end">The block index used when calculating EpicPulse.</param>
        /// <returns>The amount of unclaimed EpicPulse.</returns>
        [ContractMethod(CpuFee = 1 << 17, RequiredCallFlags = CallFlags.ReadStates)]
        public BigInteger UnclaimedEpicPulse(DataCache snapshot, UInt160 account, uint end)
        {
            StorageItem storage = snapshot.TryGet(CreateStorageKey(Prefix_Account).Add(account));
            if (storage is null) return BigInteger.Zero;
            EpicChainAccountState state = storage.GetInteroperable<EpicChainAccountState>();
            return CalculateBonus(snapshot, state, end);
        }

        [ContractMethod(RequiredCallFlags = CallFlags.States)]
        private bool RegisterCandidate(ApplicationEngine engine, ECPoint pubkey)
        {
            if (!engine.CheckWitnessInternal(Contract.CreateSignatureRedeemScript(pubkey).ToScriptHash()))
                return false;
            // In the unit of datoshi, 1 datoshi = 1e-8 EpicPulse
            engine.AddFee(GetRegisterPrice(engine.SnapshotCache));
            StorageKey key = CreateStorageKey(Prefix_Candidate).Add(pubkey);
            StorageItem item = engine.SnapshotCache.GetAndChange(key, () => new StorageItem(new CandidateState()));
            CandidateState state = item.GetInteroperable<CandidateState>();
            if (state.Registered) return true;
            state.Registered = true;
            engine.SendNotification(Hash, "CandidateStateChanged",
                new VM.Types.Array(engine.ReferenceCounter) { pubkey.ToArray(), true, state.Votes });
            return true;
        }

        [ContractMethod(CpuFee = 1 << 16, RequiredCallFlags = CallFlags.States)]
        private bool UnregisterCandidate(ApplicationEngine engine, ECPoint pubkey)
        {
            if (!engine.CheckWitnessInternal(Contract.CreateSignatureRedeemScript(pubkey).ToScriptHash()))
                return false;
            StorageKey key = CreateStorageKey(Prefix_Candidate).Add(pubkey);
            if (engine.SnapshotCache.TryGet(key) is null) return true;
            StorageItem item = engine.SnapshotCache.GetAndChange(key);
            CandidateState state = item.GetInteroperable<CandidateState>();
            if (!state.Registered) return true;
            state.Registered = false;
            CheckCandidate(engine.SnapshotCache, pubkey, state);
            engine.SendNotification(Hash, "CandidateStateChanged",
                new VM.Types.Array(engine.ReferenceCounter) { pubkey.ToArray(), false, state.Votes });
            return true;
        }

        [ContractMethod(CpuFee = 1 << 16, RequiredCallFlags = CallFlags.States)]
        private async ContractTask<bool> Vote(ApplicationEngine engine, UInt160 account, ECPoint voteTo)
        {
            if (!engine.CheckWitnessInternal(account)) return false;
            EpicChainAccountState state_account = engine.SnapshotCache.GetAndChange(CreateStorageKey(Prefix_Account).Add(account))?.GetInteroperable<EpicChainAccountState>();
            if (state_account is null) return false;
            if (state_account.Balance == 0) return false;
            CandidateState validator_new = null;
            if (voteTo != null)
            {
                validator_new = engine.SnapshotCache.GetAndChange(CreateStorageKey(Prefix_Candidate).Add(voteTo))?.GetInteroperable<CandidateState>();
                if (validator_new is null) return false;
                if (!validator_new.Registered) return false;
            }
            if (state_account.VoteTo is null ^ voteTo is null)
            {
                StorageItem item = engine.SnapshotCache.GetAndChange(CreateStorageKey(Prefix_VotersCount));
                if (state_account.VoteTo is null)
                    item.Add(state_account.Balance);
                else
                    item.Add(-state_account.Balance);
            }
            EpicPulseDistribution EpicPulseDistribution = DistributeEpicPulse(engine, account, state_account);
            if (state_account.VoteTo != null)
            {
                StorageKey key = CreateStorageKey(Prefix_Candidate).Add(state_account.VoteTo);
                StorageItem storage_validator = engine.SnapshotCache.GetAndChange(key);
                CandidateState state_validator = storage_validator.GetInteroperable<CandidateState>();
                state_validator.Votes -= state_account.Balance;
                CheckCandidate(engine.SnapshotCache, state_account.VoteTo, state_validator);
            }
            if (voteTo != null && voteTo != state_account.VoteTo)
            {
                StorageKey voterRewardKey = CreateStorageKey(Prefix_VoterRewardPerCommittee).Add(voteTo);
                var latestEpicPulsePerVote = engine.SnapshotCache.TryGet(voterRewardKey) ?? BigInteger.Zero;
                state_account.LastEpicPulsePerVote = latestEpicPulsePerVote;
            }
            ECPoint from = state_account.VoteTo;
            state_account.VoteTo = voteTo;

            if (validator_new != null)
            {
                validator_new.Votes += state_account.Balance;
            }
            else
            {
                state_account.LastEpicPulsePerVote = 0;
            }
            engine.SendNotification(Hash, "Vote",
                new VM.Types.Array(engine.ReferenceCounter) { account.ToArray(), from?.ToArray() ?? StackItem.Null, voteTo?.ToArray() ?? StackItem.Null, state_account.Balance });
            if (EpicPulseDistribution is not null)
                await EpicPulse.Mint(engine, EpicPulseDistribution.Account, EpicPulseDistribution.Amount, true);
            return true;
        }

        /// <summary>
        /// Gets the first 256 registered candidates.
        /// </summary>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <returns>All the registered candidates.</returns>
        [ContractMethod(CpuFee = 1 << 22, RequiredCallFlags = CallFlags.ReadStates)]
        internal (ECPoint PublicKey, BigInteger Votes)[] GetCandidates(DataCache snapshot)
        {
            return GetCandidatesInternal(snapshot)
                .Select(p => (p.PublicKey, p.State.Votes))
                .Take(256)
                .ToArray();
        }

        /// <summary>
        /// Gets the registered candidates iterator.
        /// </summary>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <returns>All the registered candidates.</returns>
        [ContractMethod(CpuFee = 1 << 22, RequiredCallFlags = CallFlags.ReadStates)]
        private IIterator GetAllCandidates(DataCache snapshot)
        {
            const FindOptions options = FindOptions.RemovePrefix | FindOptions.DeserializeValues | FindOptions.PickField1;
            var enumerator = GetCandidatesInternal(snapshot)
                .Select(p => (p.Key, p.Value))
                .GetEnumerator();
            return new StorageIterator(enumerator, 1, options);
        }

        internal IEnumerable<(StorageKey Key, StorageItem Value, ECPoint PublicKey, CandidateState State)> GetCandidatesInternal(DataCache snapshot)
        {
            byte[] prefix_key = CreateStorageKey(Prefix_Candidate).ToArray();
            return snapshot.Find(prefix_key)
                .Select(p => (p.Key, p.Value, PublicKey: p.Key.Key[1..].AsSerializable<ECPoint>(), State: p.Value.GetInteroperable<CandidateState>()))
                .Where(p => p.State.Registered)
                .Where(p => !Policy.IsBlocked(snapshot, Contract.CreateSignatureRedeemScript(p.PublicKey).ToScriptHash()));
        }

        /// <summary>
        /// Gets votes from specific candidate.
        /// </summary>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <param name="pubKey">Specific public key</param>
        /// <returns>Votes or -1 if it was not found.</returns>
        [ContractMethod(CpuFee = 1 << 15, RequiredCallFlags = CallFlags.ReadStates)]
        public BigInteger GetCandidateVote(DataCache snapshot, ECPoint pubKey)
        {
            StorageItem storage = snapshot.TryGet(CreateStorageKey(Prefix_Candidate).Add(pubKey));
            CandidateState state = storage?.GetInteroperable<CandidateState>();
            return state?.Registered == true ? state.Votes : -1;
        }

        /// <summary>
        /// Gets all the members of the committee.
        /// </summary>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <returns>The public keys of the members.</returns>
        [ContractMethod(CpuFee = 1 << 16, RequiredCallFlags = CallFlags.ReadStates)]
        public ECPoint[] GetCommittee(DataCache snapshot)
        {
            return GetCommitteeFromCache(snapshot).Select(p => p.PublicKey).OrderBy(p => p).ToArray();
        }

        /// <summary>
        /// Get account state.
        /// </summary>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <param name="account">account</param>
        /// <returns>The state of the account.</returns>
        [ContractMethod(CpuFee = 1 << 15, RequiredCallFlags = CallFlags.ReadStates)]
        public EpicChainAccountState GetAccountState(DataCache snapshot, UInt160 account)
        {
            return snapshot.TryGet(CreateStorageKey(Prefix_Account).Add(account))?.GetInteroperable<EpicChainAccountState>();
        }

        /// <summary>
        /// Gets the address of the committee.
        /// </summary>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <returns>The address of the committee.</returns>
        [ContractMethod(Hardfork.HF_Cockatrice, CpuFee = 1 << 16, RequiredCallFlags = CallFlags.ReadStates)]
        public UInt160 GetCommitteeAddress(DataCache snapshot)
        {
            ECPoint[] committees = GetCommittee(snapshot);
            return Contract.CreateMultiSigRedeemScript(committees.Length - (committees.Length - 1) / 2, committees).ToScriptHash();
        }

        private CachedCommittee GetCommitteeFromCache(DataCache snapshot)
        {
            return snapshot[CreateStorageKey(Prefix_Committee)].GetInteroperable<CachedCommittee>();
        }

        /// <summary>
        /// Computes the validators of the next block.
        /// </summary>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <param name="settings">The <see cref="ProtocolSettings"/> used during computing.</param>
        /// <returns>The public keys of the validators.</returns>
        public ECPoint[] ComputeNextBlockValidators(DataCache snapshot, ProtocolSettings settings)
        {
            return ComputeCommitteeMembers(snapshot, settings).Select(p => p.PublicKey).Take(settings.ValidatorsCount).OrderBy(p => p).ToArray();
        }

        private IEnumerable<(ECPoint PublicKey, BigInteger Votes)> ComputeCommitteeMembers(DataCache snapshot, ProtocolSettings settings)
        {
            decimal votersCount = (decimal)(BigInteger)snapshot[CreateStorageKey(Prefix_VotersCount)];
            decimal voterTurnout = votersCount / (decimal)TotalAmount;
            var candidates = GetCandidatesInternal(snapshot)
                .Select(p => (p.PublicKey, p.State.Votes))
                .ToArray();
            if (voterTurnout < EffectiveVoterTurnout || candidates.Length < settings.CommitteeMembersCount)
                return settings.StandbyCommittee.Select(p => (p, candidates.FirstOrDefault(k => k.PublicKey.Equals(p)).Votes));
            return candidates
                .OrderByDescending(p => p.Votes)
                .ThenBy(p => p.PublicKey)
                .Take(settings.CommitteeMembersCount);
        }

        [ContractMethod(CpuFee = 1 << 16, RequiredCallFlags = CallFlags.ReadStates)]
        private ECPoint[] GetNextBlockValidators(ApplicationEngine engine)
        {
            return GetNextBlockValidators(engine.SnapshotCache, engine.ProtocolSettings.ValidatorsCount);
        }

        /// <summary>
        /// Gets the validators of the next block.
        /// </summary>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <param name="validatorsCount">The number of validators in the system.</param>
        /// <returns>The public keys of the validators.</returns>
        public ECPoint[] GetNextBlockValidators(DataCache snapshot, int validatorsCount)
        {
            return GetCommitteeFromCache(snapshot)
                .Take(validatorsCount)
                .Select(p => p.PublicKey)
                .OrderBy(p => p)
                .ToArray();
        }

        /// <summary>
        /// Represents the account state of <see cref="EpicChain"/>.
        /// </summary>
        public class EpicChainAccountState : AccountState
        {
            /// <summary>
            /// The height of the block where the balance changed last time.
            /// </summary>
            public uint BalanceHeight;

            /// <summary>
            /// The voting target of the account. This field can be <see langword="null"/>.
            /// </summary>
            public ECPoint VoteTo;

            public BigInteger LastEpicPulsePerVote;

            public override void FromStackItem(StackItem stackItem)
            {
                base.FromStackItem(stackItem);
                Struct @struct = (Struct)stackItem;
                BalanceHeight = (uint)@struct[1].GetInteger();
                VoteTo = @struct[2].IsNull ? null : ECPoint.DecodePoint(@struct[2].GetSpan(), ECCurve.Secp256r1);
                LastEpicPulsePerVote = @struct[3].GetInteger();
            }

            public override StackItem ToStackItem(ReferenceCounter referenceCounter)
            {
                Struct @struct = (Struct)base.ToStackItem(referenceCounter);
                @struct.Add(BalanceHeight);
                @struct.Add(VoteTo?.ToArray() ?? StackItem.Null);
                @struct.Add(LastEpicPulsePerVote);
                return @struct;
            }
        }

        internal class CandidateState : IInteroperable
        {
            public bool Registered;
            public BigInteger Votes;

            public void FromStackItem(StackItem stackItem)
            {
                Struct @struct = (Struct)stackItem;
                Registered = @struct[0].GetBoolean();
                Votes = @struct[1].GetInteger();
            }

            public StackItem ToStackItem(ReferenceCounter referenceCounter)
            {
                return new Struct(referenceCounter) { Registered, Votes };
            }
        }

        internal class CachedCommittee : InteroperableList<(ECPoint PublicKey, BigInteger Votes)>
        {
            public CachedCommittee() { }
            public CachedCommittee(IEnumerable<(ECPoint, BigInteger)> collection) => AddRange(collection);

            protected override (ECPoint, BigInteger) ElementFromStackItem(StackItem item)
            {
                Struct @struct = (Struct)item;
                return (ECPoint.DecodePoint(@struct[0].GetSpan(), ECCurve.Secp256r1), @struct[1].GetInteger());
            }

            protected override StackItem ElementToStackItem((ECPoint PublicKey, BigInteger Votes) element, ReferenceCounter referenceCounter)
            {
                return new Struct(referenceCounter) { element.PublicKey.ToArray(), element.Votes };
            }
        }

        private record EpicPulseDistribution
        {
            public UInt160 Account { get; init; }
            public BigInteger Amount { get; init; }
        }
    }
}
