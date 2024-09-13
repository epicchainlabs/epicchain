// Copyright (C) 2021-2024 EpicChain Labs.

//
// TokensTracker.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.IEventHandlers;
using EpicChain.Ledger;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Persistence;
using EpicChain.Plugins.RpcServer;
using EpicChain.Plugins.Trackers;
using System;
using System.Collections.Generic;
using System.Linq;
using static System.IO.Path;

namespace EpicChain.Plugins
{
    public class TokensTracker : Plugin, ICommittingHandler, ICommittedHandler
    {
        private string _dbPath;
        private bool _shouldTrackHistory;
        private uint _maxResults;
        private uint _network;
        private string[] _enabledTrackers;
        private IStore _db;
        private UnhandledExceptionPolicy _exceptionPolicy;
        private EpicChainSystem EpicChainSystem;
        private readonly List<TrackerBase> trackers = new();
        protected override UnhandledExceptionPolicy ExceptionPolicy => _exceptionPolicy;

        public override string Description => "Enquiries balances and transaction history of accounts through RPC";

        public override string ConfigFile => System.IO.Path.Combine(RootPath, "TokensTracker.json");

        public TokensTracker()
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
            IConfigurationSection config = GetConfiguration();
            _dbPath = config.GetValue("DBPath", "TokensBalanceData");
            _shouldTrackHistory = config.GetValue("TrackHistory", true);
            _maxResults = config.GetValue("MaxResults", 1000u);
            _network = config.GetValue("Network", 860833102u);
            _enabledTrackers = config.GetSection("EnabledTrackers").GetChildren().Select(p => p.Value).ToArray();
            var policyString = config.GetValue(nameof(UnhandledExceptionPolicy), nameof(UnhandledExceptionPolicy.StopNode));
            if (Enum.TryParse(policyString, true, out UnhandledExceptionPolicy policy))
            {
                _exceptionPolicy = policy;
            }
        }

        protected override void OnSystemLoaded(EpicChainSystem system)
        {
            if (system.Settings.Network != _network) return;
            EpicChainSystem = system;
            string path = string.Format(_dbPath, EpicChainSystem.Settings.Network.ToString("X8"));
            _db = EpicChainSystem.LoadStore(GetFullPath(path));
            if (_enabledTrackers.Contains("XEP-11"))
                trackers.Add(new Trackers.XEP_11.Xep11Tracker(_db, _maxResults, _shouldTrackHistory, EpicChainSystem));
            if (_enabledTrackers.Contains("XEP-17"))
                trackers.Add(new Trackers.XEP_17.Xep17Tracker(_db, _maxResults, _shouldTrackHistory, EpicChainSystem));
            foreach (TrackerBase tracker in trackers)
                RpcServerPlugin.RegisterMethods(tracker, _network);
        }

        private void ResetBatch()
        {
            foreach (var tracker in trackers)
            {
                tracker.ResetBatch();
            }
        }

        void ICommittingHandler.Blockchain_Committing_Handler(EpicChainSystem system, Block block, DataCache snapshot, IReadOnlyList<Blockchain.ApplicationExecuted> applicationExecutedList)
        {
            if (system.Settings.Network != _network) return;
            // Start freshly with a new DBCache for each block.
            ResetBatch();
            foreach (var tracker in trackers)
            {
                tracker.OnPersist(system, block, snapshot, applicationExecutedList);
            }
        }

        void ICommittedHandler.Blockchain_Committed_Handler(EpicChainSystem system, Block block)
        {
            if (system.Settings.Network != _network) return;
            foreach (var tracker in trackers)
            {
                tracker.Commit();
            }
        }
    }
}
