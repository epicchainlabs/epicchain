// Copyright (C) 2021-2024 The EpicChain Labs.
//
// InventoryType.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


namespace Neo.Network.P2P.Payloads
{
    /// <summary>
    /// Represents the type of an inventory.
    /// </summary>
    public enum InventoryType : byte
    {
        /// <summary>
        /// Indicates that the inventory is a <see cref="Transaction"/>.
        /// </summary>
        TX = MessageCommand.Transaction,

        /// <summary>
        /// Indicates that the inventory is a <see cref="Block"/>.
        /// </summary>
        Block = MessageCommand.Block,

        /// <summary>
        /// Indicates that the inventory is an <see cref="ExtensiblePayload"/>.
        /// </summary>
        Extensible = MessageCommand.Extensible
    }
}
