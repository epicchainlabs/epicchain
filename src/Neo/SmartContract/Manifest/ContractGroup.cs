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
// ContractGroup.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.Cryptography;
using Neo.Cryptography.ECC;
using Neo.IO;
using Neo.Json;
using Neo.VM;
using Neo.VM.Types;
using System;

namespace Neo.SmartContract.Manifest
{
    /// <summary>
    /// Represents a set of mutually trusted contracts.
    /// A contract will trust and allow any contract in the same group to invoke it, and the user interface will not give any warnings.
    /// A group is identified by a public key and must be accompanied by a signature for the contract hash to prove that the contract is indeed included in the group.
    /// </summary>
    public class ContractGroup : IInteroperable
    {
        /// <summary>
        /// The public key of the group.
        /// </summary>
        public ECPoint PubKey { get; set; }

        /// <summary>
        /// The signature of the contract hash which can be verified by <see cref="PubKey"/>.
        /// </summary>
        public byte[] Signature { get; set; }

        void IInteroperable.FromStackItem(StackItem stackItem)
        {
            Struct @struct = (Struct)stackItem;
            PubKey = ECPoint.DecodePoint(@struct[0].GetSpan(), ECCurve.Secp256r1);
            Signature = @struct[1].GetSpan().ToArray();
        }

        public StackItem ToStackItem(ReferenceCounter referenceCounter)
        {
            return new Struct(referenceCounter) { PubKey.ToArray(), Signature };
        }

        /// <summary>
        /// Converts the group from a JSON object.
        /// </summary>
        /// <param name="json">The group represented by a JSON object.</param>
        /// <returns>The converted group.</returns>
        public static ContractGroup FromJson(JObject json)
        {
            ContractGroup group = new()
            {
                PubKey = ECPoint.Parse(json["pubkey"].GetString(), ECCurve.Secp256r1),
                Signature = Convert.FromBase64String(json["signature"].GetString()),
            };
            if (group.Signature.Length != 64) throw new FormatException();
            return group;
        }

        /// <summary>
        /// Determines whether the signature in the group is valid.
        /// </summary>
        /// <param name="hash">The hash of the contract.</param>
        /// <returns><see langword="true"/> if the signature is valid; otherwise, <see langword="false"/>.</returns>
        public bool IsValid(UInt160 hash)
        {
            return Crypto.VerifySignature(hash.ToArray(), Signature, PubKey);
        }

        /// <summary>
        /// Converts the group to a JSON object.
        /// </summary>
        /// <returns>The group represented by a JSON object.</returns>
        public JObject ToJson()
        {
            var json = new JObject();
            json["pubkey"] = PubKey.ToString();
            json["signature"] = Convert.ToBase64String(Signature);
            return json;
        }
    }
}
