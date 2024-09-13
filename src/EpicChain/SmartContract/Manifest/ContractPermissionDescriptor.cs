// Copyright (C) 2021-2024 EpicChain Labs.

//
// ContractPermissionDescriptor.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Cryptography.ECC;
using EpicChain.IO;
using EpicChain.Json;
using EpicChain.VM.Types;
using System;

namespace EpicChain.SmartContract.Manifest
{
    /// <summary>
    /// Indicates which contracts are authorized to be called.
    /// </summary>
    public class ContractPermissionDescriptor : IEquatable<ContractPermissionDescriptor>
    {
        /// <summary>
        /// The hash of the contract. It can't be set with <see cref="Group"/>.
        /// </summary>
        public UInt160 Hash { get; }

        /// <summary>
        /// The group of the contracts. It can't be set with <see cref="Hash"/>.
        /// </summary>
        public ECPoint Group { get; }

        /// <summary>
        /// Indicates whether <see cref="Hash"/> is set.
        /// </summary>
        public bool IsHash => Hash != null;

        /// <summary>
        /// Indicates whether <see cref="Group"/> is set.
        /// </summary>
        public bool IsGroup => Group != null;

        /// <summary>
        /// Indicates whether it is a wildcard.
        /// </summary>
        public bool IsWildcard => Hash is null && Group is null;

        private ContractPermissionDescriptor(UInt160 hash, ECPoint group)
        {
            Hash = hash;
            Group = group;
        }

        internal ContractPermissionDescriptor(ReadOnlySpan<byte> span)
        {
            switch (span.Length)
            {
                case UInt160.Length:
                    Hash = new UInt160(span);
                    break;
                case 33:
                    Group = ECPoint.DecodePoint(span, ECCurve.Secp256r1);
                    break;
                default:
                    throw new ArgumentException(null, nameof(span));
            }
        }

        public static ContractPermissionDescriptor Create(StackItem item)
        {
            return item.Equals(StackItem.Null) ? CreateWildcard() : new ContractPermissionDescriptor(item.GetSpan());
        }

        /// <summary>
        /// Creates a new instance of the <see cref="ContractPermissionDescriptor"/> class with the specified contract hash.
        /// </summary>
        /// <param name="hash">The contract to be called.</param>
        /// <returns>The created permission descriptor.</returns>
        public static ContractPermissionDescriptor Create(UInt160 hash)
        {
            return new ContractPermissionDescriptor(hash, null);
        }

        /// <summary>
        /// Creates a new instance of the <see cref="ContractPermissionDescriptor"/> class with the specified group.
        /// </summary>
        /// <param name="group">The group of the contracts to be called.</param>
        /// <returns>The created permission descriptor.</returns>
        public static ContractPermissionDescriptor Create(ECPoint group)
        {
            return new ContractPermissionDescriptor(null, group);
        }

        /// <summary>
        /// Creates a new instance of the <see cref="ContractPermissionDescriptor"/> class with wildcard.
        /// </summary>
        /// <returns>The created permission descriptor.</returns>
        public static ContractPermissionDescriptor CreateWildcard()
        {
            return new ContractPermissionDescriptor(null, null);
        }

        public override bool Equals(object obj)
        {
            if (obj is not ContractPermissionDescriptor other) return false;
            return Equals(other);
        }

        public bool Equals(ContractPermissionDescriptor other)
        {
            if (other is null) return false;
            if (this == other) return true;
            if (IsWildcard == other.IsWildcard) return true;
            if (IsHash) return Hash.Equals(other.Hash);
            if (IsGroup) return Group.Equals(other.Group);
            return false;
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(Hash, Group);
        }

        /// <summary>
        /// Converts the permission descriptor from a JSON object.
        /// </summary>
        /// <param name="json">The permission descriptor represented by a JSON object.</param>
        /// <returns>The converted permission descriptor.</returns>
        public static ContractPermissionDescriptor FromJson(JString json)
        {
            string str = json.GetString();
            if (str.Length == 42)
                return Create(UInt160.Parse(str));
            if (str.Length == 66)
                return Create(ECPoint.Parse(str, ECCurve.Secp256r1));
            if (str == "*")
                return CreateWildcard();
            throw new FormatException();
        }

        /// <summary>
        /// Converts the permission descriptor to a JSON object.
        /// </summary>
        /// <returns>The permission descriptor represented by a JSON object.</returns>
        public JString ToJson()
        {
            if (IsHash) return Hash.ToString();
            if (IsGroup) return Group.ToString();
            return "*";
        }

        /// <summary>
        /// Converts the permission descriptor to byte array.
        /// </summary>
        /// <returns>The converted byte array. Or <see langword="null"/> if it is a wildcard.</returns>
        public byte[] ToArray()
        {
            return Hash?.ToArray() ?? Group?.EncodePoint(true);
        }
    }
}
