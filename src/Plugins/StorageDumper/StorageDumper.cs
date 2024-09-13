// Copyright (C) 2021-2024 EpicChain Labs.

//
// StorageDumper.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.ConsoleService;
using EpicChain.IEventHandlers;
using EpicChain.IO;
using EpicChain.Json;
using EpicChain.Ledger;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Persistence;
using EpicChain.SmartContract.Native;

namespace EpicChain.Plugins.StorageDumper
{
    public class StorageDumper : Plugin, ICommittingHandler, ICommittedHandler
    {
        private readonly Dictionary<uint, EpicChainSystem> systems = new Dictionary<uint, EpicChainSystem>();

        private StreamWriter? _writer;
        /// <summary>
        /// _currentBlock stores the last cached item
        /// </summary>
        private JObject? _currentBlock;
        private string? _lastCreateDirectory;
        protected override UnhandledExceptionPolicy ExceptionPolicy => Settings.Default?.ExceptionPolicy ?? UnhandledExceptionPolicy.Ignore;

        public override string Description => "Exports EpicChain-CLI status data";

        public override string ConfigFile => System.IO.Path.Combine(RootPath, "StorageDumper.json");

        public StorageDumper()
        {
            Blockchain.Committing += ((ICommittingHandler)this).Blockchain_Committing_Handler;
            Blockchain.Committed += ((ICommittedHandler)this).Blockchain_Committed_Handler;
        }

        public override void Dispose()
        {
            Blockchain.Committing -= ((ICommittingHandler)this).Blockchain_Committing_Handler;
            Blockchain.Committed -= ((ICommittedHandler)this).Blockchain_Committed_Handler;
        }

        protected override void Configure()
        {
            Settings.Load(GetConfiguration());
        }

        protected override void OnSystemLoaded(EpicChainSystem system)
        {
            systems.Add(system.Settings.Network, system);
        }

        /// <summary>
        /// Process "dump contract-storage" command
        /// </summary>
        [ConsoleCommand("dump contract-storage", Category = "Storage", Description = "You can specify the contract script hash or use null to get the corresponding information from the storage")]
        private void OnDumpStorage(uint network, UInt160? contractHash = null)
        {
            if (!systems.ContainsKey(network)) throw new InvalidOperationException("invalid network");
            string path = $"dump_{network}.json";
            byte[]? prefix = null;
            if (contractHash is not null)
            {
                var contract = NativeContract.ContractManagement.GetContract(systems[network].StoreView, contractHash);
                if (contract is null) throw new InvalidOperationException("contract not found");
                prefix = BitConverter.GetBytes(contract.Id);
            }
            var states = systems[network].StoreView.Find(prefix);
            JArray array = new JArray(states.Where(p => !Settings.Default!.Exclude.Contains(p.Key.Id)).Select(p => new JObject
            {
                ["key"] = Convert.ToBase64String(p.Key.ToArray()),
                ["value"] = Convert.ToBase64String(p.Value.ToArray())
            }));
            File.WriteAllText(path, array.ToString());
            ConsoleHelper.Info("States",
                $"({array.Count})",
                " have been dumped into file ",
                $"{path}");
        }

        void ICommittingHandler.Blockchain_Committing_Handler(EpicChainSystem system, Block block, DataCache snapshot, IReadOnlyList<Blockchain.ApplicationExecuted> applicationExecutedList)
        {
            InitFileWriter(system.Settings.Network, snapshot);
            OnPersistStorage(system.Settings.Network, snapshot);
        }

        private void OnPersistStorage(uint network, DataCache snapshot)
        {
            uint blockIndex = NativeContract.Ledger.CurrentIndex(snapshot);
            if (blockIndex >= Settings.Default!.HeightToBegin)
            {
                JArray stateChangeArray = new JArray();

                foreach (var trackable in snapshot.GetChangeSet())
                {
                    if (Settings.Default.Exclude.Contains(trackable.Key.Id))
                        continue;
                    JObject state = new JObject();
                    switch (trackable.State)
                    {
                        case TrackState.Added:
                            state["state"] = "Added";
                            state["key"] = Convert.ToBase64String(trackable.Key.ToArray());
                            state["value"] = Convert.ToBase64String(trackable.Item.ToArray());
                            break;
                        case TrackState.Changed:
                            state["state"] = "Changed";
                            state["key"] = Convert.ToBase64String(trackable.Key.ToArray());
                            state["value"] = Convert.ToBase64String(trackable.Item.ToArray());
                            break;
                        case TrackState.Deleted:
                            state["state"] = "Deleted";
                            state["key"] = Convert.ToBase64String(trackable.Key.ToArray());
                            break;
                    }
                    stateChangeArray.Add(state);
                }

                JObject bs_item = new JObject();
                bs_item["block"] = blockIndex;
                bs_item["size"] = stateChangeArray.Count;
                bs_item["storage"] = stateChangeArray;
                _currentBlock = bs_item;
            }
        }


        void ICommittedHandler.Blockchain_Committed_Handler(EpicChainSystem system, Block block)
        {
            OnCommitStorage(system.Settings.Network, system.StoreView);
        }

        void OnCommitStorage(uint network, DataCache snapshot)
        {
            if (_currentBlock != null && _writer != null)
            {
                _writer.WriteLine(_currentBlock.ToString());
                _writer.Flush();
            }
        }

        private void InitFileWriter(uint network, DataCache snapshot)
        {
            uint blockIndex = NativeContract.Ledger.CurrentIndex(snapshot);
            if (_writer == null
                || blockIndex % Settings.Default!.BlockCacheSize == 0)
            {
                string path = GetOrCreateDirectory(network, blockIndex);
                var filepart = (blockIndex / Settings.Default!.BlockCacheSize) * Settings.Default.BlockCacheSize;
                path = $"{path}/dump-block-{filepart}.dump";
                if (_writer != null)
                {
                    _writer.Dispose();
                }
                _writer = new StreamWriter(new FileStream(path, FileMode.Append));
            }
        }

        private string GetOrCreateDirectory(uint network, uint blockIndex)
        {
            string dirPathWithBlock = GetDirectoryPath(network, blockIndex);
            if (_lastCreateDirectory != dirPathWithBlock)
            {
                Directory.CreateDirectory(dirPathWithBlock);
                _lastCreateDirectory = dirPathWithBlock;
            }
            return dirPathWithBlock;
        }

        private string GetDirectoryPath(uint network, uint blockIndex)
        {
            uint folder = (blockIndex / Settings.Default!.StoragePerFolder) * Settings.Default.StoragePerFolder;
            return $"./StorageDumper_{network}/BlockStorage_{folder}";
        }

    }
}
