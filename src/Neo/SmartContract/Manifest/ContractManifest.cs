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
// ContractManifest.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.IO;
using Neo.Json;
using Neo.VM;
using Neo.VM.Types;
using System;
using System.Linq;
using Array = Neo.VM.Types.Array;

namespace Neo.SmartContract.Manifest
{
    /// <summary>
    /// Represents the manifest of a smart contract.
    /// When a smart contract is deployed, it must explicitly declare the features and permissions it will use.
    /// When it is running, it will be limited by its declared list of features and permissions, and cannot make any behavior beyond the scope of the list.
    /// </summary>
    /// <remarks>For more details, see NEP-15.</remarks>
    public class ContractManifest : IInteroperable
    {
        /// <summary>
        /// The maximum length of a manifest.
        /// </summary>
        public const int MaxLength = ushort.MaxValue;

        /// <summary>
        /// The name of the contract.
        /// </summary>
        public string Name { get; set; }

        /// <summary>
        /// The groups of the contract.
        /// </summary>
        public ContractGroup[] Groups { get; set; }

        /// <summary>
        /// Indicates which standards the contract supports. It can be a list of NEPs.
        /// </summary>
        public string[] SupportedStandards { get; set; }

        /// <summary>
        /// The ABI of the contract.
        /// </summary>
        public ContractAbi Abi { get; set; }

        /// <summary>
        /// The permissions of the contract.
        /// </summary>
        public ContractPermission[] Permissions { get; set; }

        /// <summary>
        /// The trusted contracts and groups of the contract.
        /// If a contract is trusted, the user interface will not give any warnings when called by the contract.
        /// </summary>
        public WildcardContainer<ContractPermissionDescriptor> Trusts { get; set; }

        /// <summary>
        /// Custom user data.
        /// </summary>
        public JObject Extra { get; set; }

        void IInteroperable.FromStackItem(StackItem stackItem)
        {
            Struct @struct = (Struct)stackItem;
            Name = @struct[0].GetString();
            Groups = ((Array)@struct[1]).Select(p => p.ToInteroperable<ContractGroup>()).ToArray();
            if (((Map)@struct[2]).Count != 0)
                throw new ArgumentException(null, nameof(stackItem));
            SupportedStandards = ((Array)@struct[3]).Select(p => p.GetString()).ToArray();
            Abi = @struct[4].ToInteroperable<ContractAbi>();
            Permissions = ((Array)@struct[5]).Select(p => p.ToInteroperable<ContractPermission>()).ToArray();
            Trusts = @struct[6] switch
            {
                Null _ => WildcardContainer<ContractPermissionDescriptor>.CreateWildcard(),
                // Array array when array.Any(p => ((ByteString)p).Size == 0) => WildcardContainer<ContractPermissionDescriptor>.CreateWildcard(),
                Array array => WildcardContainer<ContractPermissionDescriptor>.Create(array.Select(ContractPermissionDescriptor.Create).ToArray()),
                _ => throw new ArgumentException(null, nameof(stackItem))
            };
            Extra = (JObject)JToken.Parse(@struct[7].GetSpan());
        }

        public StackItem ToStackItem(ReferenceCounter referenceCounter)
        {
            return new Struct(referenceCounter)
            {
                Name,
                new Array(referenceCounter, Groups.Select(p => p.ToStackItem(referenceCounter))),
                new Map(referenceCounter),
                new Array(referenceCounter, SupportedStandards.Select(p => (StackItem)p)),
                Abi.ToStackItem(referenceCounter),
                new Array(referenceCounter, Permissions.Select(p => p.ToStackItem(referenceCounter))),
                Trusts.IsWildcard ? StackItem.Null : new Array(referenceCounter, Trusts.Select(p => p.ToArray()?? StackItem.Null)),
                Extra is null ? "null" : Extra.ToByteArray(false)
            };
        }

        /// <summary>
        /// Converts the manifest from a JSON object.
        /// </summary>
        /// <param name="json">The manifest represented by a JSON object.</param>
        /// <returns>The converted manifest.</returns>
        public static ContractManifest FromJson(JObject json)
        {
            ContractManifest manifest = new()
            {
                Name = json["name"].GetString(),
                Groups = ((JArray)json["groups"]).Select(u => ContractGroup.FromJson((JObject)u)).ToArray(),
                SupportedStandards = ((JArray)json["supportedstandards"]).Select(u => u.GetString()).ToArray(),
                Abi = ContractAbi.FromJson((JObject)json["abi"]),
                Permissions = ((JArray)json["permissions"]).Select(u => ContractPermission.FromJson((JObject)u)).ToArray(),
                Trusts = WildcardContainer<ContractPermissionDescriptor>.FromJson(json["trusts"], u => ContractPermissionDescriptor.FromJson((JString)u)),
                Extra = (JObject)json["extra"]
            };
            if (string.IsNullOrEmpty(manifest.Name))
                throw new FormatException();
            _ = manifest.Groups.ToDictionary(p => p.PubKey);
            if (json["features"] is not JObject features || features.Count != 0)
                throw new FormatException();
            if (manifest.SupportedStandards.Any(p => string.IsNullOrEmpty(p)))
                throw new FormatException();
            _ = manifest.SupportedStandards.ToDictionary(p => p);
            _ = manifest.Permissions.ToDictionary(p => p.Contract);
            _ = manifest.Trusts.ToDictionary(p => p);
            return manifest;
        }

        /// <summary>
        /// Parse the manifest from a byte array containing JSON data.
        /// </summary>
        /// <param name="json">The byte array containing JSON data.</param>
        /// <returns>The parsed manifest.</returns>
        public static ContractManifest Parse(ReadOnlySpan<byte> json)
        {
            if (json.Length > MaxLength) throw new ArgumentException(null, nameof(json));
            return FromJson((JObject)JToken.Parse(json));
        }

        /// <summary>
        /// Parse the manifest from a JSON <see cref="string"/>.
        /// </summary>
        /// <param name="json">The JSON <see cref="string"/>.</param>
        /// <returns>The parsed manifest.</returns>
        public static ContractManifest Parse(string json) => Parse(Utility.StrictUTF8.GetBytes(json));

        /// <summary>
        /// Converts the manifest to a JSON object.
        /// </summary>
        /// <returns>The manifest represented by a JSON object.</returns>
        public JObject ToJson()
        {
            return new JObject
            {
                ["name"] = Name,
                ["groups"] = Groups.Select(u => u.ToJson()).ToArray(),
                ["features"] = new JObject(),
                ["supportedstandards"] = SupportedStandards.Select(u => new JString(u)).ToArray(),
                ["abi"] = Abi.ToJson(),
                ["permissions"] = Permissions.Select(p => p.ToJson()).ToArray(),
                ["trusts"] = Trusts.ToJson(p => p.ToJson()),
                ["extra"] = Extra
            };
        }

        /// <summary>
        /// Determines whether the manifest is valid.
        /// </summary>
        /// <param name="limits">The <see cref="ExecutionEngineLimits"/> used for test serialization.</param>
        /// <param name="hash">The hash of the contract.</param>
        /// <returns><see langword="true"/> if the manifest is valid; otherwise, <see langword="false"/>.</returns>
        public bool IsValid(ExecutionEngineLimits limits, UInt160 hash)
        {
            // Ensure that is serializable
            try
            {
                _ = BinarySerializer.Serialize(ToStackItem(null), limits);
            }
            catch
            {
                return false;
            }
            // Check groups
            return Groups.All(u => u.IsValid(hash));
        }
    }
}
