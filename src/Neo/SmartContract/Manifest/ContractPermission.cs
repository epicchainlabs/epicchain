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
// ContractPermission.cs file belongs to the neo project and is free
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
    /// Represents a permission of a contract. It describes which contracts may be
    /// invoked and which methods are called.
    /// If a contract invokes a contract or method that is not declared in the manifest
    /// at runtime, the invocation will fail.
    /// </summary>
    public class ContractPermission : IInteroperable
    {
        /// <summary>
        /// Indicates which contract to be invoked.
        /// It can be a hash of a contract, a public key of a group, or a wildcard *.
        /// If it specifies a hash of a contract, then the contract will be invoked;
        /// If it specifies a public key of a group, then any contract in this group
        /// may be invoked; If it specifies a wildcard *, then any contract may be invoked.
        /// </summary>
        public ContractPermissionDescriptor Contract { get; set; }

        /// <summary>
        /// Indicates which methods to be called.
        /// It can also be assigned with a wildcard *. If it is a wildcard *,
        /// then it means that any method can be called.
        /// </summary>
        public WildcardContainer<string> Methods { get; set; }

        /// <summary>
        /// A default permission that both <see cref="Contract"/> and <see cref="Methods"/> fields are set to wildcard *.
        /// </summary>
        public static readonly ContractPermission DefaultPermission = new()
        {
            Contract = ContractPermissionDescriptor.CreateWildcard(),
            Methods = WildcardContainer<string>.CreateWildcard()
        };

        void IInteroperable.FromStackItem(StackItem stackItem)
        {
            Struct @struct = (Struct)stackItem;
            Contract = @struct[0] switch
            {
                Null => ContractPermissionDescriptor.CreateWildcard(),
                StackItem item => new ContractPermissionDescriptor(item.GetSpan())
            };
            Methods = @struct[1] switch
            {
                Null => WildcardContainer<string>.CreateWildcard(),
                Array array => WildcardContainer<string>.Create(array.Select(p => p.GetString()).ToArray()),
                _ => throw new ArgumentException(null, nameof(stackItem))
            };
        }

        public StackItem ToStackItem(ReferenceCounter referenceCounter)
        {
            return new Struct(referenceCounter)
            {
                Contract.IsWildcard ? StackItem.Null : Contract.IsHash ? Contract.Hash.ToArray() : Contract.Group.ToArray(),
                Methods.IsWildcard ? StackItem.Null : new Array(referenceCounter, Methods.Select(p => (StackItem)p)),
            };
        }

        /// <summary>
        /// Converts the permission from a JSON object.
        /// </summary>
        /// <param name="json">The permission represented by a JSON object.</param>
        /// <returns>The converted permission.</returns>
        public static ContractPermission FromJson(JObject json)
        {
            ContractPermission permission = new()
            {
                Contract = ContractPermissionDescriptor.FromJson((JString)json["contract"]),
                Methods = WildcardContainer<string>.FromJson(json["methods"], u => u.GetString()),
            };
            if (permission.Methods.Any(p => string.IsNullOrEmpty(p)))
                throw new FormatException();
            _ = permission.Methods.ToDictionary(p => p);
            return permission;
        }

        /// <summary>
        /// Converts the permission to a JSON object.
        /// </summary>
        /// <returns>The permission represented by a JSON object.</returns>
        public JObject ToJson()
        {
            var json = new JObject();
            json["contract"] = Contract.ToJson();
            json["methods"] = Methods.ToJson(p => p);
            return json;
        }

        /// <summary>
        /// Determines whether the method of the specified contract can be called by this contract.
        /// </summary>
        /// <param name="targetContract">The contract being called.</param>
        /// <param name="targetMethod">The method of the specified contract.</param>
        /// <returns><see langword="true"/> if the contract allows to be called; otherwise, <see langword="false"/>.</returns>
        public bool IsAllowed(ContractState targetContract, string targetMethod)
        {
            if (Contract.IsHash)
            {
                if (!Contract.Hash.Equals(targetContract.Hash)) return false;
            }
            else if (Contract.IsGroup)
            {
                if (targetContract.Manifest.Groups.All(p => !p.PubKey.Equals(Contract.Group))) return false;
            }
            return Methods.IsWildcard || Methods.Contains(targetMethod);
        }
    }
}
