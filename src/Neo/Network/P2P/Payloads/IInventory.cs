// Copyright (C) 2021-2024 The EpicChain Labs.
//
// IInventory.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
    /// Represents a message that can be relayed on the NEO network.
    /// </summary>
    public interface IInventory : IVerifiable
    {
        /// <summary>
        /// The type of the inventory.
        /// </summary>
        InventoryType InventoryType { get; }
    }
}
