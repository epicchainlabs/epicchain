// Copyright (C) 2021-2024 The EpicChain Labs.
//
// NodeCapabilityType.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using System;

namespace Neo.Network.P2P.Capabilities
{
    /// <summary>
    /// Represents the type of <see cref="NodeCapability"/>.
    /// </summary>
    public enum NodeCapabilityType : byte
    {
        #region Servers

        /// <summary>
        /// Indicates that the node is listening on a Tcp port.
        /// </summary>
        TcpServer = 0x01,

        /// <summary>
        /// Indicates that the node is listening on a WebSocket port.
        /// </summary>
        [Obsolete]
        WsServer = 0x02,

        #endregion

        #region Others

        /// <summary>
        /// Indicates that the node has complete block data.
        /// </summary>
        FullNode = 0x10

        #endregion
    }
}
