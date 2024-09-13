// Copyright (C) 2021-2024 EpicChain Labs.

//
// NamedCurveHash.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


namespace EpicChain.SmartContract.Native
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
