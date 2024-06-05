// Copyright (C) 2021-2024 The EpicChain Labs.
//
// VerifyResult.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.Network.P2P.Payloads;

namespace Neo.Ledger
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
