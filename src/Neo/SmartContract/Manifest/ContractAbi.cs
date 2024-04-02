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
// ContractAbi.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.Json;
using Neo.VM;
using Neo.VM.Types;
using System;
using System.Collections.Generic;
using System.Linq;
using Array = Neo.VM.Types.Array;

namespace Neo.SmartContract.Manifest
{
    /// <summary>
    /// Represents the ABI of a smart contract.
    /// </summary>
    /// <remarks>For more details, see NEP-14.</remarks>
    public class ContractAbi : IInteroperable
    {
        private IReadOnlyDictionary<(string, int), ContractMethodDescriptor> methodDictionary;

        /// <summary>
        /// Gets the methods in the ABI.
        /// </summary>
        public ContractMethodDescriptor[] Methods { get; set; }

        /// <summary>
        /// Gets the events in the ABI.
        /// </summary>
        public ContractEventDescriptor[] Events { get; set; }

        void IInteroperable.FromStackItem(StackItem stackItem)
        {
            Struct @struct = (Struct)stackItem;
            Methods = ((Array)@struct[0]).Select(p => p.ToInteroperable<ContractMethodDescriptor>()).ToArray();
            Events = ((Array)@struct[1]).Select(p => p.ToInteroperable<ContractEventDescriptor>()).ToArray();
        }

        public StackItem ToStackItem(ReferenceCounter referenceCounter)
        {
            return new Struct(referenceCounter)
            {
                new Array(referenceCounter, Methods.Select(p => p.ToStackItem(referenceCounter))),
                new Array(referenceCounter, Events.Select(p => p.ToStackItem(referenceCounter))),
            };
        }

        /// <summary>
        /// Converts the ABI from a JSON object.
        /// </summary>
        /// <param name="json">The ABI represented by a JSON object.</param>
        /// <returns>The converted ABI.</returns>
        public static ContractAbi FromJson(JObject json)
        {
            ContractAbi abi = new()
            {
                Methods = ((JArray)json["methods"]).Select(u => ContractMethodDescriptor.FromJson((JObject)u)).ToArray(),
                Events = ((JArray)json["events"]).Select(u => ContractEventDescriptor.FromJson((JObject)u)).ToArray()
            };
            if (abi.Methods.Length == 0) throw new FormatException();
            return abi;
        }

        /// <summary>
        /// Gets the method with the specified name.
        /// </summary>
        /// <param name="name">The name of the method.</param>
        /// <param name="pcount">The number of parameters of the method. It can be set to -1 to search for the method with the specified name and any number of parameters.</param>
        /// <returns>The method that matches the specified name and number of parameters. If <paramref name="pcount"/> is set to -1, the first method with the specified name will be returned.</returns>
        public ContractMethodDescriptor GetMethod(string name, int pcount)
        {
            if (pcount < -1 || pcount > ushort.MaxValue) throw new ArgumentOutOfRangeException(nameof(pcount));
            if (pcount >= 0)
            {
                methodDictionary ??= Methods.ToDictionary(p => (p.Name, p.Parameters.Length));
                methodDictionary.TryGetValue((name, pcount), out var method);
                return method;
            }
            else
            {
                return Methods.FirstOrDefault(p => p.Name == name);
            }
        }

        /// <summary>
        /// Converts the ABI to a JSON object.
        /// </summary>
        /// <returns>The ABI represented by a JSON object.</returns>
        public JObject ToJson()
        {
            var json = new JObject();
            json["methods"] = new JArray(Methods.Select(u => u.ToJson()).ToArray());
            json["events"] = new JArray(Events.Select(u => u.ToJson()).ToArray());
            return json;
        }
    }
}
