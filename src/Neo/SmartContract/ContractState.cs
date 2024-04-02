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
// ContractState.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.IO;
using Neo.Json;
using Neo.SmartContract.Manifest;
using Neo.VM;
using Neo.VM.Types;
using System;
using System.Linq;
using Array = Neo.VM.Types.Array;

namespace Neo.SmartContract
{
    /// <summary>
    /// Represents a deployed contract.
    /// </summary>
    public class ContractState : IInteroperable
    {
        /// <summary>
        /// The id of the contract.
        /// </summary>
        public int Id;

        /// <summary>
        /// Indicates the number of times the contract has been updated.
        /// </summary>
        public ushort UpdateCounter;

        /// <summary>
        /// The hash of the contract.
        /// </summary>
        public UInt160 Hash;

        /// <summary>
        /// The nef of the contract.
        /// </summary>
        public NefFile Nef;

        /// <summary>
        /// The manifest of the contract.
        /// </summary>
        public ContractManifest Manifest;

        /// <summary>
        /// The script of the contract.
        /// </summary>
        public ReadOnlyMemory<byte> Script => Nef.Script;

        IInteroperable IInteroperable.Clone()
        {
            return new ContractState
            {
                Id = Id,
                UpdateCounter = UpdateCounter,
                Hash = Hash,
                Nef = Nef,
                Manifest = Manifest
            };
        }

        void IInteroperable.FromReplica(IInteroperable replica)
        {
            ContractState from = (ContractState)replica;
            Id = from.Id;
            UpdateCounter = from.UpdateCounter;
            Hash = from.Hash;
            Nef = from.Nef;
            Manifest = from.Manifest;
        }

        void IInteroperable.FromStackItem(StackItem stackItem)
        {
            Array array = (Array)stackItem;
            Id = (int)array[0].GetInteger();
            UpdateCounter = (ushort)array[1].GetInteger();
            Hash = new UInt160(array[2].GetSpan());
            Nef = ((ByteString)array[3]).Memory.AsSerializable<NefFile>();
            Manifest = array[4].ToInteroperable<ContractManifest>();
        }

        /// <summary>
        /// Determines whether the current contract has the permission to call the specified contract.
        /// </summary>
        /// <param name="targetContract">The contract to be called.</param>
        /// <param name="targetMethod">The method to be called.</param>
        /// <returns><see langword="true"/> if the contract allows to be called; otherwise, <see langword="false"/>.</returns>
        public bool CanCall(ContractState targetContract, string targetMethod)
        {
            return Manifest.Permissions.Any(u => u.IsAllowed(targetContract, targetMethod));
        }

        /// <summary>
        /// Converts the contract to a JSON object.
        /// </summary>
        /// <returns>The contract represented by a JSON object.</returns>
        public JObject ToJson()
        {
            return new JObject
            {
                ["id"] = Id,
                ["updatecounter"] = UpdateCounter,
                ["hash"] = Hash.ToString(),
                ["nef"] = Nef.ToJson(),
                ["manifest"] = Manifest.ToJson()
            };
        }

        public StackItem ToStackItem(ReferenceCounter referenceCounter)
        {
            return new Array(referenceCounter, new StackItem[] { Id, (int)UpdateCounter, Hash.ToArray(), Nef.ToArray(), Manifest.ToStackItem(referenceCounter) });
        }
    }
}
