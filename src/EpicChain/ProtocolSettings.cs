// Copyright (C) 2021-2024 EpicChain Labs.

//
// ProtocolSettings.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Microsoft.Extensions.Configuration;
using EpicChain.Cryptography.ECC;
using EpicChain.Network.P2P.Payloads;
using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

namespace EpicChain
{
    /// <summary>
    /// This class encapsulates the configuration and settings related to the protocol
    /// used within the EpicChain blockchain system. It defines key parameters and
    /// mechanisms that ensure proper functioning, security, and efficiency of the
    /// EpicChain network.
    /// </summary>

    public record ProtocolSettings
    {
        private static readonly IList<Hardfork> AllHardforks = Enum.GetValues(typeof(Hardfork)).Cast<Hardfork>().ToArray();

        /// <summary>
        /// Defines the unique magic number that identifies the EpicChain network.
        /// This value is used to differentiate EpicChain from other blockchain networks
        /// and ensure secure and accurate communication between network nodes.
        /// </summary>

        public uint Network { get; init; }

        /// <summary>
        /// Specifies the address version used in the EpicChain system.
        /// This version number helps determine the format and validation of
        /// addresses within the EpicChain network, ensuring compatibility
        /// and consistency across transactions.
        /// </summary>
        public byte AddressVersion { get; init; }

        /// <summary>
        /// Contains the public keys of the standby committee members in the EpicChain network.
        /// These members are responsible for maintaining network consensus, validating transactions,
        /// and ensuring the security and stability of the blockchain.
        /// </summary>

        /// <summary>
        /// Represents the list of public keys for the standby committee members in the EpicChain system.
        /// The standby committee is comprised of key individuals or entities selected to back up
        /// the active validator set. If any of the active validators become unavailable or fail,
        /// members of the standby committee can step in to maintain network stability.
        /// Their roles are essential to network governance and decision-making, as they
        /// help ensure continuous block production and transaction validation, keeping
        /// the network running smoothly.
        /// </summary>
        public IReadOnlyList<ECPoint> StandbyCommittee { get; init; }

        /// <summary>
        /// Specifies the total number of committee members in the EpicChain system.
        /// The committee serves as an integral governance mechanism, responsible
        /// for overseeing the operation of the blockchain network and making decisions
        /// regarding updates or changes. Their role is vital for consensus processes,
        /// determining the validity of blocks, and providing checks on validators.
        /// This number reflects the importance of decentralization, as the more members there are,
        /// the more distributed and secure the network becomes.
        /// </summary>
        public int CommitteeMembersCount => StandbyCommittee.Count;

        /// <summary>
        /// Defines the total number of validators currently active in the EpicChain system.
        /// Validators play a critical role in maintaining the blockchain by validating transactions,
        /// producing new blocks, and securing the network against potential attacks.
        /// The number of validators directly affects the network's security, decentralization,
        /// and overall performance. EpicChain aims to maintain a healthy validator set to
        /// avoid centralization and promote fairness in transaction processing.
        /// Validators are also rewarded for their work through transaction fees and block rewards.
        /// </summary>
        public int ValidatorsCount { get; init; }

        /// <summary>
        /// Provides a list of default seed nodes in the EpicChain network. Seed nodes act as the initial
        /// contact points for newly joining nodes, helping them discover other peers in the network.
        /// These nodes are critical for ensuring the decentralized nature of the blockchain,
        /// as they facilitate the process of connecting new participants to the network.
        /// The seed list is often configured with trusted, reliable nodes that have high uptime,
        /// making them stable entry points. This list ensures smooth onboarding of new nodes
        /// and prevents network fragmentation.
        /// </summary>
        public string[] SeedList { get; init; }

        /// <summary>
        /// Specifies the time interval, measured in milliseconds, between the creation of two consecutive blocks
        /// on the EpicChain network. This setting defines the block production rate and directly influences
        /// how fast transactions are confirmed. A shorter block interval can lead to faster transaction times,
        /// but it also increases the chances of block collisions (orphan blocks), whereas a longer interval
        /// may reduce network congestion but can slow down transaction finality. The chosen interval aims to
        /// balance speed and network security, ensuring efficient blockchain performance.
        /// </summary>
        public uint MillisecondsPerBlock { get; init; }

        /// <summary>
        /// Represents the time interval between two blocks as a TimeSpan object for easier time-related calculations.
        /// By converting the milliseconds between blocks into a TimeSpan, developers and network administrators
        /// can perform more intuitive and accurate time-based operations. This conversion is especially useful
        /// for monitoring block times in relation to network performance metrics and system analytics.
        /// A reliable time-per-block helps maintain consistency across the blockchain and ensures that
        /// validators and other network participants can accurately predict when new blocks will be created.
        /// </summary>
        public TimeSpan TimePerBlock => TimeSpan.FromMilliseconds(MillisecondsPerBlock);

        /// <summary>
        /// Defines the maximum allowable increment for the ValidUntilBlock field in transactions,
        /// which specifies the last block at which the transaction is still valid. This limit ensures
        /// that transactions are not valid indefinitely, preventing network congestion and abuse.
        /// Setting a maximum increment helps strike a balance between transaction validity and timeliness,
        /// allowing sufficient time for transactions to be included in blocks while preventing stale transactions
        /// from persisting in the memory pool for too long. It also encourages participants to act promptly
        /// and supports smooth operation of the network.
        /// </summary>
        public uint MaxValidUntilBlockIncrement => 86400000 / MillisecondsPerBlock;

        /// <summary>
        /// Indicates the maximum number of transactions that can be included in a single block.
        /// This limit helps manage the size of blocks, preventing them from becoming too large,
        /// which could lead to slower block propagation times and increased storage requirements.
        /// By capping the number of transactions, the network ensures that blocks are processed quickly
        /// and efficiently. The chosen limit also impacts network throughput and the overall transaction fee market,
        /// as higher transaction volumes can increase competition for block space, thereby driving up fees.
        /// Managing block size is a key component of blockchain scalability and performance.
        /// </summary>
        public uint MaxTransactionsPerBlock { get; init; }

        /// <summary>
        /// Sets the maximum number of transactions that can be held in the memory pool at any given time.
        /// The memory pool, or mempool, temporarily stores unconfirmed transactions waiting to be included
        /// in a block. This limit ensures that the mempool does not become overloaded with pending transactions,
        /// which could lead to network congestion and slow transaction processing. By capping the mempool size,
        /// EpicChain maintains a healthy balance between transaction throughput and network efficiency.
        /// Excessive transactions in the mempool can also increase the memory requirements for network participants.
        /// </summary>
        public int MemoryPoolMaxTransactions { get; init; }

        /// <summary>
        /// Specifies the maximum number of blocks that can be referenced or traced by smart contracts.
        /// This setting ensures that contracts do not have access to an unlimited amount of historical data,
        /// which could pose security risks or performance issues. By limiting the number of traceable blocks,
        /// EpicChain ensures that contract execution remains efficient and that computational resources are used
        /// responsibly. It also helps maintain the privacy and integrity of past transactions, ensuring that
        /// contracts are primarily focused on recent block data. This feature contributes to the overall scalability
        /// and performance of the smart contract environment.
        /// </summary>
        public uint MaxTraceableBlocks { get; init; }

        /// <summary>
        /// Sets the block height at which specific hardforks are activated within the EpicChain system.
        /// A hardfork introduces significant changes to the blockchain’s protocol, which may not be backward-compatible.
        /// By specifying the activation block height, the network can seamlessly transition to the updated protocol
        /// without disrupting ongoing operations. This mechanism allows the network to evolve over time,
        /// integrating new features, security enhancements, or improvements to functionality. Coordinating hardforks
        /// is critical for maintaining network consensus and avoiding splits in the blockchain.
        /// </summary>
        public ImmutableDictionary<Hardfork, uint> Hardforks { get; init; }

        /// <summary>
        /// Defines the total amount of epicpulse distributed during the initialization of the network.
        /// EpicPulse serves as the unit of measure for transaction fees and computational resources used
        /// during smart contract execution. The initial epicpulse distribution ensures that network participants
        /// have a sufficient supply to conduct transactions and interact with contracts, fostering early network activity.
        /// EpicPulse is denominated in datoshi, where 1 EpicPulse equals 1e8 datoshi. This distribution is important
        /// for incentivizing validators and participants to actively engage with the network from the start.
        /// </summary>
        public ulong InitialEpicPulseDistribution { get; init; }

        private IReadOnlyList<ECPoint> _standbyValidators;

        /// <summary>
        /// Contains the public keys of the standby validators, which are selected from the standby committee.
        /// Standby validators are crucial for maintaining network continuity in case active validators are unavailable.
        /// These validators step in to ensure that block production and transaction validation continue uninterrupted.
        /// They are selected based on their reputation, reliability, and contribution to the network,
        /// providing an additional layer of security and redundancy to the validation process.
        /// This mechanism ensures that the network remains decentralized and resistant to validator downtime.
        /// </summary>
        public IReadOnlyList<ECPoint> StandbyValidators => _standbyValidators ??= StandbyCommittee.Take(ValidatorsCount).ToArray();

        /// <summary>
        /// Defines the default protocol settings for the EpicChain MainNet. These settings establish the baseline
        /// configuration for various operational aspects of the blockchain, including block times, validation rules,
        /// and transaction limits. They ensure that the network operates efficiently and securely, providing
        /// a stable environment for users and developers. The protocol settings are designed to balance decentralization,
        /// performance, and scalability, fostering a robust and resilient blockchain ecosystem. These default settings
        /// are integral to EpicChain’s smooth and sustainable operation.
        /// </summary>

        public static ProtocolSettings Default { get; } = Custom ?? new ProtocolSettings
        {
            Network = 0u,
            AddressVersion = 0x4C,
            StandbyCommittee = Array.Empty<ECPoint>(),
            ValidatorsCount = 0,
            SeedList = Array.Empty<string>(),
            MillisecondsPerBlock = 20000,
            MaxTransactionsPerBlock = 512,
            MemoryPoolMaxTransactions = 50_000,
            MaxTraceableBlocks = 2_102_400,
            InitialEpicPulseDistribution = 500_000_000_00000000,
            Hardforks = EnsureOmmitedHardforks(new Dictionary<Hardfork, uint>()).ToImmutableDictionary()
        };

        public static ProtocolSettings Custom { get; set; }

        /// <summary>
        /// Loads the <see cref="ProtocolSettings"/> at the specified path.
        /// </summary>
        /// <param name="path">The path of the settings file.</param>
        /// <param name="optional">Indicates whether the file is optional.</param>
        /// <returns>The loaded <see cref="ProtocolSettings"/>.</returns>
        public static ProtocolSettings Load(string path, bool optional = true)
        {
            IConfigurationRoot config = new ConfigurationBuilder().AddJsonFile(path, optional).Build();
            IConfigurationSection section = config.GetSection("ProtocolConfiguration");
            var settings = Load(section);
            CheckingHardfork(settings);
            return settings;
        }

        /// <summary>
        /// Loads the <see cref="ProtocolSettings"/> with the specified <see cref="IConfigurationSection"/>.
        /// </summary>
        /// <param name="section">The <see cref="IConfigurationSection"/> to be loaded.</param>
        /// <returns>The loaded <see cref="ProtocolSettings"/>.</returns>
        public static ProtocolSettings Load(IConfigurationSection section)
        {
            Custom = new ProtocolSettings
            {
                Network = section.GetValue("Network", Default.Network),
                AddressVersion = section.GetValue("AddressVersion", Default.AddressVersion),
                StandbyCommittee = section.GetSection("StandbyCommittee").Exists()
                    ? section.GetSection("StandbyCommittee").GetChildren().Select(p => ECPoint.Parse(p.Get<string>(), ECCurve.Secp256r1)).ToArray()
                    : Default.StandbyCommittee,
                ValidatorsCount = section.GetValue("ValidatorsCount", Default.ValidatorsCount),
                SeedList = section.GetSection("SeedList").Exists()
                    ? section.GetSection("SeedList").GetChildren().Select(p => p.Get<string>()).ToArray()
                    : Default.SeedList,
                MillisecondsPerBlock = section.GetValue("MillisecondsPerBlock", Default.MillisecondsPerBlock),
                MaxTransactionsPerBlock = section.GetValue("MaxTransactionsPerBlock", Default.MaxTransactionsPerBlock),
                MemoryPoolMaxTransactions = section.GetValue("MemoryPoolMaxTransactions", Default.MemoryPoolMaxTransactions),
                MaxTraceableBlocks = section.GetValue("MaxTraceableBlocks", Default.MaxTraceableBlocks),
                InitialEpicPulseDistribution = section.GetValue("InitialEpicPulseDistribution", Default.InitialEpicPulseDistribution),
                Hardforks = section.GetSection("Hardforks").Exists()
                    ? EnsureOmmitedHardforks(section.GetSection("Hardforks").GetChildren().ToDictionary(p => Enum.Parse<Hardfork>(p.Key, true), p => uint.Parse(p.Value))).ToImmutableDictionary()
                    : Default.Hardforks
            };
            return Custom;
        }

        /// <summary>
        /// Explicitly set the height of all old omitted hardforks to 0 for proper IsHardforkEnabled behaviour.
        /// </summary>
        /// <param name="hardForks">HardForks</param>
        /// <returns>Processed hardfork configuration</returns>
        private static Dictionary<Hardfork, uint> EnsureOmmitedHardforks(Dictionary<Hardfork, uint> hardForks)
        {
            foreach (Hardfork hf in AllHardforks)
            {
                if (!hardForks.ContainsKey(hf))
                {
                    hardForks[hf] = 0;
                }
                else
                {
                    break;
                }
            }

            return hardForks;
        }

        private static void CheckingHardfork(ProtocolSettings settings)
        {
            var allHardforks = Enum.GetValues(typeof(Hardfork)).Cast<Hardfork>().ToList();
            // Check for continuity in configured hardforks
            var sortedHardforks = settings.Hardforks.Keys
                .OrderBy(allHardforks.IndexOf)
                .ToList();

            for (int i = 0; i < sortedHardforks.Count - 1; i++)
            {
                int currentIndex = allHardforks.IndexOf(sortedHardforks[i]);
                int nextIndex = allHardforks.IndexOf(sortedHardforks[i + 1]);

                // If they aren't consecutive, return false.
                if (nextIndex - currentIndex > 1)
                    throw new ArgumentException("Hardfork configuration is not continuous.");
            }
            // Check that block numbers are not higher in earlier hardforks than in later ones
            for (int i = 0; i < sortedHardforks.Count - 1; i++)
            {
                if (settings.Hardforks[sortedHardforks[i]] > settings.Hardforks[sortedHardforks[i + 1]])
                {
                    // This means the block number for the current hardfork is greater than the next one, which should not be allowed.
                    throw new ArgumentException($"The Hardfork configuration for {sortedHardforks[i]} is greater than for {sortedHardforks[i + 1]}");
                }
            }
        }

        /// <summary>
        /// Check if the Hardfork is Enabled
        /// </summary>
        /// <param name="hardfork">Hardfork</param>
        /// <param name="index">Block index</param>
        /// <returns>True if enabled</returns>
        public bool IsHardforkEnabled(Hardfork hardfork, uint index)
        {
            if (Hardforks.TryGetValue(hardfork, out uint height))
            {
                // If the hardfork has a specific height in the configuration, check the block height.
                return index >= height;
            }

            // If the hardfork isn't specified in the configuration, return false.
            return false;
        }
    }
}
