// Copyright (C) 2021-2024 The EpicChain Lab's.
//
// MessageFlags.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using System;

namespace Neo.Network.P2P
{
    /// <summary>
    /// Represents the flags of a message.
    /// </summary>
    [Flags]
    public enum MessageFlags : byte
    {
        /// <summary>
        /// No flag is set for the message.
        /// </summary>
        None = 0,

        /// <summary>
        /// Indicates that the message is compressed.
        /// </summary>
        Compressed = 1 << 0
    }
}
