// Copyright (C) 2021-2024 EpicChain Labs.

//
// Session.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Network.P2P.Payloads;
using EpicChain.Persistence;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Iterators;
using EpicChain.SmartContract.Native;
using System;
using System.Collections.Generic;

namespace EpicChain.Plugins.RpcServer
{
    class Session : IDisposable
    {
        public readonly SnapshotCache Snapshot;
        public readonly ApplicationEngine Engine;
        public readonly Dictionary<Guid, IIterator> Iterators = new();
        public DateTime StartTime;

        public Session(EpicChainSystem system, byte[] script, Signer[] signers, Witness[] witnesses, long datoshi, Diagnostic diagnostic)
        {
            Random random = new();
            Snapshot = system.GetSnapshotCache();
            Transaction tx = signers == null ? null : new Transaction
            {
                Version = 0,
                Nonce = (uint)random.Next(),
                ValidUntilBlock = NativeContract.Ledger.CurrentIndex(Snapshot) + system.Settings.MaxValidUntilBlockIncrement,
                Signers = signers,
                Attributes = Array.Empty<TransactionAttribute>(),
                Script = script,
                Witnesses = witnesses
            };
            Engine = ApplicationEngine.Run(script, Snapshot, container: tx, settings: system.Settings, epicpulse: datoshi, diagnostic: diagnostic);
            ResetExpiration();
        }

        public void ResetExpiration()
        {
            StartTime = DateTime.UtcNow;
        }

        public void Dispose()
        {
            Engine.Dispose();
            Snapshot.Dispose();
        }
    }
}
