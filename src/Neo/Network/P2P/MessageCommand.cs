// EpicChain Copyright Project (2021-2024)
// 
// Copyright (c) 2021-2024 EpicChain
// 
// EpicChain is an innovative blockchain network developed and maintained by xmoohad. This copyright project outlines the rights and responsibilities associated with the EpicChain software and its related components.
// 
// 1. Copyright Holder:
//    - xmoohad
// 
// 2. Project Name:
//    - EpicChain
// 
// 3. Project Description:
//    - EpicChain is a decentralized blockchain network that aims to revolutionize the way digital assets are managed, traded, and secured. With its innovative features and robust architecture, EpicChain provides a secure and efficient platform for various decentralized applications (dApps) and digital asset management.
// 
// 4. Copyright Period:
//    - The copyright for the EpicChain software and its related components is valid from 2021 to 2024.
// 
// 5. Copyright Statement:
//    - All rights reserved. No part of the EpicChain software or its related components may be reproduced, distributed, or transmitted in any form or by any means, without the prior written permission of the copyright holder, except in the case of brief quotations embodied in critical reviews and certain other noncommercial uses permitted by copyright law.
// 
// 6. License:
//    - The EpicChain software is licensed under the EpicChain Software License, a custom license that governs the use, distribution, and modification of the software. The EpicChain Software License is designed to promote the free and open development of the EpicChain network while protecting the interests of the copyright holder.
// 
// 7. Open Source:
//    - EpicChain is an open-source project, and its source code is available to the public under the terms of the EpicChain Software License. Developers are encouraged to contribute to the development of EpicChain and create innovative applications on top of the EpicChain network.
// 
// 8. Disclaimer:
//    - The EpicChain software and its related components are provided "as is," without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall the copyright holder or contributors be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the EpicChain software or its related components.
// 
// 9. Contact Information:
//    - For inquiries regarding the EpicChain copyright project, please contact xmoohad at [email address].
// 
// 10. Updates:
//     - This copyright project may be updated or modified from time to time to reflect changes in the EpicChain project or to address new legal or regulatory requirements. Users and developers are encouraged to check the latest version of the copyright project periodically.

//
// MessageCommand.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.Cryptography;
using Neo.IO.Caching;
using Neo.Network.P2P.Payloads;

namespace Neo.Network.P2P
{
    /// <summary>
    /// Represents the command of a message.
    /// </summary>
    public enum MessageCommand : byte
    {
        #region handshaking

        /// <summary>
        /// Sent when a connection is established.
        /// </summary>
        [ReflectionCache(typeof(VersionPayload))]
        Version = 0x00,

        /// <summary>
        /// Sent to respond to <see cref="Version"/> messages.
        /// </summary>
        Verack = 0x01,

        #endregion

        #region connectivity

        /// <summary>
        /// Sent to request for remote nodes.
        /// </summary>
        GetAddr = 0x10,

        /// <summary>
        /// Sent to respond to <see cref="GetAddr"/> messages.
        /// </summary>
        [ReflectionCache(typeof(AddrPayload))]
        Addr = 0x11,

        /// <summary>
        /// Sent to detect whether the connection has been disconnected.
        /// </summary>
        [ReflectionCache(typeof(PingPayload))]
        Ping = 0x18,

        /// <summary>
        /// Sent to respond to <see cref="Ping"/> messages.
        /// </summary>
        [ReflectionCache(typeof(PingPayload))]
        Pong = 0x19,

        #endregion

        #region synchronization

        /// <summary>
        /// Sent to request for headers.
        /// </summary>
        [ReflectionCache(typeof(GetBlockByIndexPayload))]
        GetHeaders = 0x20,

        /// <summary>
        /// Sent to respond to <see cref="GetHeaders"/> messages.
        /// </summary>
        [ReflectionCache(typeof(HeadersPayload))]
        Headers = 0x21,

        /// <summary>
        /// Sent to request for blocks.
        /// </summary>
        [ReflectionCache(typeof(GetBlocksPayload))]
        GetBlocks = 0x24,

        /// <summary>
        /// Sent to request for memory pool.
        /// </summary>
        Mempool = 0x25,

        /// <summary>
        /// Sent to relay inventories.
        /// </summary>
        [ReflectionCache(typeof(InvPayload))]
        Inv = 0x27,

        /// <summary>
        /// Sent to request for inventories.
        /// </summary>
        [ReflectionCache(typeof(InvPayload))]
        GetData = 0x28,

        /// <summary>
        /// Sent to request for blocks.
        /// </summary>
        [ReflectionCache(typeof(GetBlockByIndexPayload))]
        GetBlockByIndex = 0x29,

        /// <summary>
        /// Sent to respond to <see cref="GetData"/> messages when the inventories are not found.
        /// </summary>
        [ReflectionCache(typeof(InvPayload))]
        NotFound = 0x2a,

        /// <summary>
        /// Sent to send a transaction.
        /// </summary>
        [ReflectionCache(typeof(Transaction))]
        Transaction = 0x2b,

        /// <summary>
        /// Sent to send a block.
        /// </summary>
        [ReflectionCache(typeof(Block))]
        Block = 0x2c,

        /// <summary>
        /// Sent to send an <see cref="ExtensiblePayload"/>.
        /// </summary>
        [ReflectionCache(typeof(ExtensiblePayload))]
        Extensible = 0x2e,

        /// <summary>
        /// Sent to reject an inventory.
        /// </summary>
        Reject = 0x2f,

        #endregion

        #region SPV protocol

        /// <summary>
        /// Sent to load the <see cref="BloomFilter"/>.
        /// </summary>
        [ReflectionCache(typeof(FilterLoadPayload))]
        FilterLoad = 0x30,

        /// <summary>
        /// Sent to update the items for the <see cref="BloomFilter"/>.
        /// </summary>
        [ReflectionCache(typeof(FilterAddPayload))]
        FilterAdd = 0x31,

        /// <summary>
        /// Sent to clear the <see cref="BloomFilter"/>.
        /// </summary>
        FilterClear = 0x32,

        /// <summary>
        /// Sent to send a filtered block.
        /// </summary>
        [ReflectionCache(typeof(MerkleBlockPayload))]
        MerkleBlock = 0x38,

        #endregion

        #region others

        /// <summary>
        /// Sent to send an alert.
        /// </summary>
        Alert = 0x40,

        #endregion
    }
}
