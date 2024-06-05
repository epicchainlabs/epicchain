// Copyright (C) 2021-2024 The EpicChain Labs.
//
// NamedCurveHash.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
    /// Represents a pair of the named curve used in ECDSA and a hash algorithm used to hash message.
    /// </summary>
    public enum NamedCurveHash : byte
    {
        /// <summary>
        /// The secp256k1 curve and SHA256 hash algorithm.
        /// </summary>
        secp256k1SHA256 = 22,

        /// <summary>
        /// The secp256r1 curve, which known as prime256v1 or nistP-256, and SHA256 hash algorithm.
        /// </summary>
        secp256r1SHA256 = 23,

        /// <summary>
        /// The secp256k1 curve and Keccak256 hash algorithm.
        /// </summary>
        secp256k1Keccak256 = 122,

        /// <summary>
        /// The secp256r1 curve, which known as prime256v1 or nistP-256, and Keccak256 hash algorithm.
        /// </summary>
        secp256r1Keccak256 = 123
    }
}
