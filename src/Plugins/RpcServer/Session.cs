// Copyright (C) 2021-2024 The EpicChain Labs.
//
// Session.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.Network.P2P.Payloads;
using Neo.Persistence;
using Neo.SmartContract;
using Neo.SmartContract.Iterators;
using Neo.SmartContract.Native;
using System;
using System.Collections.Generic;

namespace Neo.Plugins.RpcServer
{
    class Session : IDisposable
    {
        public readonly SnapshotCache Snapshot;
        public readonly ApplicationEngine Engine;
        public readonly Dictionary<Guid, IIterator> Iterators = new();
        public DateTime StartTime;

        public Session(NeoSystem system, byte[] script, Signer[] signers, Witness[] witnesses, long datoshi, Diagnostic diagnostic)
        {
            Random random = new();
            Snapshot = system.GetSnapshot();
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
            Engine = ApplicationEngine.Run(script, Snapshot, container: tx, settings: system.Settings, gas: datoshi, diagnostic: diagnostic);
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
