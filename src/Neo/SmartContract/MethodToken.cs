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
// MethodToken.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.IO;
using Neo.Json;
using System;
using System.IO;

namespace Neo.SmartContract
{
    /// <summary>
    /// Represents the methods that a contract will call statically.
    /// </summary>
    public class MethodToken : ISerializable
    {
        /// <summary>
        /// The hash of the contract to be called.
        /// </summary>
        public UInt160 Hash;

        /// <summary>
        /// The name of the method to be called.
        /// </summary>
        public string Method;

        /// <summary>
        /// The number of parameters of the method to be called.
        /// </summary>
        public ushort ParametersCount;

        /// <summary>
        /// Indicates whether the method to be called has a return value.
        /// </summary>
        public bool HasReturnValue;

        /// <summary>
        /// The <see cref="CallFlags"/> to be used to call the contract.
        /// </summary>
        public CallFlags CallFlags;

        public int Size =>
            UInt160.Length +        // Hash
            Method.GetVarSize() +   // Method
            sizeof(ushort) +        // ParametersCount
            sizeof(bool) +          // HasReturnValue
            sizeof(CallFlags);      // CallFlags

        void ISerializable.Deserialize(ref MemoryReader reader)
        {
            Hash = reader.ReadSerializable<UInt160>();
            Method = reader.ReadVarString(32);
            if (Method.StartsWith('_')) throw new FormatException();
            ParametersCount = reader.ReadUInt16();
            HasReturnValue = reader.ReadBoolean();
            CallFlags = (CallFlags)reader.ReadByte();
            if ((CallFlags & ~CallFlags.All) != 0) throw new FormatException();
        }

        void ISerializable.Serialize(BinaryWriter writer)
        {
            writer.Write(Hash);
            writer.WriteVarString(Method);
            writer.Write(ParametersCount);
            writer.Write(HasReturnValue);
            writer.Write((byte)CallFlags);
        }

        /// <summary>
        /// Converts the token to a JSON object.
        /// </summary>
        /// <returns>The token represented by a JSON object.</returns>
        public JObject ToJson()
        {
            return new JObject
            {
                ["hash"] = Hash.ToString(),
                ["method"] = Method,
                ["paramcount"] = ParametersCount,
                ["hasreturnvalue"] = HasReturnValue,
                ["callflags"] = CallFlags
            };
        }
    }
}
