// Copyright (C) 2021-2024 EpicChain Labs.

//
// TaskSession.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Network.P2P.Capabilities;
using EpicChain.Network.P2P.Payloads;
using System;
using System.Collections.Generic;
using System.Linq;

namespace EpicChain.Network.P2P
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
