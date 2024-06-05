// Copyright (C) 2021-2024 The EpicChain Labs.
//
// ChannelsConfig.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using System.Net;

namespace Neo.Network.P2P
{
    /// <summary>
    /// Represents the settings to start <see cref="LocalNode"/>.
    /// </summary>
    public class ChannelsConfig
    {
        /// <summary>
        /// Tcp configuration.
        /// </summary>
        public IPEndPoint Tcp { get; set; }

        /// <summary>
        /// Minimum desired connections.
        /// </summary>
        public int MinDesiredConnections { get; set; } = Peer.DefaultMinDesiredConnections;

        /// <summary>
        /// Max allowed connections.
        /// </summary>
        public int MaxConnections { get; set; } = Peer.DefaultMaxConnections;

        /// <summary>
        /// Max allowed connections per address.
        /// </summary>
        public int MaxConnectionsPerAddress { get; set; } = 3;
    }
}
