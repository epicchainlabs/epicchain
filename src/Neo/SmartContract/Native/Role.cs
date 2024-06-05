// Copyright (C) 2021-2024 The EpicChain Labs.
//
// Role.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


namespace Neo.SmartContract.Native
{
    /// <summary>
    /// Represents the roles in the NEO system.
    /// </summary>
    public enum Role : byte
    {
        /// <summary>
        /// The validators of state. Used to generate and sign the state root.
        /// </summary>
        StateValidator = 4,

        /// <summary>
        /// The nodes used to process Oracle requests.
        /// </summary>
        Oracle = 8,

        /// <summary>
        /// NeoFS Alphabet nodes.
        /// </summary>
        NeoFSAlphabetNode = 16,

        /// <summary>
        /// P2P Notary nodes used to process P2P notary requests.
        /// </summary>
        P2PNotary = 32
    }
}
