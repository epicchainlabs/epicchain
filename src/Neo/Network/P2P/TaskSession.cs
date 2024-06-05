// Copyright (C) 2021-2024 The EpicChain Labs.
//
// TaskSession.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.Network.P2P.Capabilities;
using Neo.Network.P2P.Payloads;
using System;
using System.Collections.Generic;
using System.Linq;

namespace Neo.Network.P2P
{
    internal class TaskSession
    {
        public Dictionary<UInt256, DateTime> InvTasks { get; } = new Dictionary<UInt256, DateTime>();
        public Dictionary<uint, DateTime> IndexTasks { get; } = new Dictionary<uint, DateTime>();
        public HashSet<UInt256> AvailableTasks { get; } = new HashSet<UInt256>();
        public Dictionary<uint, Block> ReceivedBlock { get; } = new Dictionary<uint, Block>();
        public bool HasTooManyTasks => InvTasks.Count + IndexTasks.Count >= 100;
        public bool IsFullNode { get; }
        public uint LastBlockIndex { get; set; }
        public bool MempoolSent { get; set; }

        public TaskSession(VersionPayload version)
        {
            var fullNode = version.Capabilities.OfType<FullNodeCapability>().FirstOrDefault();
            IsFullNode = fullNode != null;
            LastBlockIndex = fullNode?.StartHeight ?? 0;
        }
    }
}
