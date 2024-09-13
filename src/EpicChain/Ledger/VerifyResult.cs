// Copyright (C) 2021-2024 EpicChain Labs.

//
// VerifyResult.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Network.P2P.Payloads;

namespace EpicChain.Ledger
{
    /// <summary>
    /// Represents a verifying result of <see cref="IInventory"/>.
    /// </summary>
    public enum VerifyResult : byte
    {
        /// <summary>
        /// Indicates that the verification was successful.
        /// </summary>
        Succeed,

        /// <summary>
        /// Indicates that an <see cref="IInventory"/> with the same hash already exists.
        /// </summary>
        AlreadyExists,

        /// <summary>
        /// Indicates that an <see cref="IInventory"/> with the same hash already exists in the memory pool.
        /// </summary>
        AlreadyInPool,

        /// <summary>
        /// Indicates that the <see cref="MemoryPool"/> is full and the transaction cannot be verified.
        /// </summary>
        OutOfMemory,

        /// <summary>
        /// Indicates that the previous block of the current block has not been received, so the block cannot be verified.
        /// </summary>
        UnableToVerify,

        /// <summary>
        /// Indicates that the <see cref="IInventory"/> is invalid.
        /// </summary>
        Invalid,

        /// <summary>
        /// Indicates that the <see cref="Transaction"/> has an invalid script.
        /// </summary>
        InvalidScript,

        /// <summary>
        /// Indicates that the <see cref="Transaction"/> has an invalid attribute.
        /// </summary>
        InvalidAttribute,

        /// <summary>
        /// Indicates that the <see cref="IInventory"/> has an invalid signature.
        /// </summary>
        InvalidSignature,

        /// <summary>
        /// Indicates that the size of the <see cref="IInventory"/> is not allowed.
        /// </summary>
        OverSize,

        /// <summary>
        /// Indicates that the <see cref="Transaction"/> has expired.
        /// </summary>
        Expired,

        /// <summary>
        /// Indicates that the <see cref="Transaction"/> failed to verify due to insufficient fees.
        /// </summary>
        InsufficientFunds,

        /// <summary>
        /// Indicates that the <see cref="Transaction"/> failed to verify because it didn't comply with the policy.
        /// </summary>
        PolicyFail,

        /// <summary>
        /// Indicates that the <see cref="Transaction"/> failed to verify because it conflicts with on-chain or mempooled transactions.
        /// </summary>
        HasConflicts,

        /// <summary>
        /// Indicates that the <see cref="IInventory"/> failed to verify due to other reasons.
        /// </summary>
        Unknown
    }
}
