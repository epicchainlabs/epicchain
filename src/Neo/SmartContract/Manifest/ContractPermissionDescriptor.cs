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
// ContractPermissionDescriptor.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.Cryptography.ECC;
using Neo.IO;
using Neo.Json;
using Neo.VM.Types;
using System;

namespace Neo.SmartContract.Manifest
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
            this.Hash = hash;
            this.Group = group;
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
            else return Group.Equals(other.Group);
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
