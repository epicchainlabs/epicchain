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
// TransactionAttribute.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.IO;
using Neo.IO.Caching;
using Neo.Json;
using Neo.Persistence;
using Neo.SmartContract.Native;
using System;
using System.IO;

namespace Neo.Network.P2P.Payloads
{
    /// <summary>
    /// Represents an attribute of a transaction.
    /// </summary>
    public abstract class TransactionAttribute : ISerializable
    {
        /// <summary>
        /// The type of the attribute.
        /// </summary>
        public abstract TransactionAttributeType Type { get; }

        /// <summary>
        /// Indicates whether multiple instances of this attribute are allowed.
        /// </summary>
        public abstract bool AllowMultiple { get; }

        public virtual int Size => sizeof(TransactionAttributeType);

        public void Deserialize(ref MemoryReader reader)
        {
            if (reader.ReadByte() != (byte)Type)
                throw new FormatException();
            DeserializeWithoutType(ref reader);
        }

        /// <summary>
        /// Deserializes an <see cref="TransactionAttribute"/> object from a <see cref="MemoryReader"/>.
        /// </summary>
        /// <param name="reader">The <see cref="MemoryReader"/> for reading data.</param>
        /// <returns>The deserialized attribute.</returns>
        public static TransactionAttribute DeserializeFrom(ref MemoryReader reader)
        {
            TransactionAttributeType type = (TransactionAttributeType)reader.ReadByte();
            if (ReflectionCache<TransactionAttributeType>.CreateInstance(type) is not TransactionAttribute attribute)
                throw new FormatException();
            attribute.DeserializeWithoutType(ref reader);
            return attribute;
        }

        /// <summary>
        /// Deserializes the <see cref="TransactionAttribute"/> object from a <see cref="MemoryReader"/>.
        /// </summary>
        /// <param name="reader">The <see cref="MemoryReader"/> for reading data.</param>
        protected abstract void DeserializeWithoutType(ref MemoryReader reader);

        /// <summary>
        /// Converts the attribute to a JSON object.
        /// </summary>
        /// <returns>The attribute represented by a JSON object.</returns>
        public virtual JObject ToJson()
        {
            return new JObject
            {
                ["type"] = Type
            };
        }

        public void Serialize(BinaryWriter writer)
        {
            writer.Write((byte)Type);
            SerializeWithoutType(writer);
        }

        /// <summary>
        /// Serializes the <see cref="TransactionAttribute"/> object to a <see cref="BinaryWriter"/>.
        /// </summary>
        /// <param name="writer">The <see cref="BinaryWriter"/> for writing data.</param>
        protected abstract void SerializeWithoutType(BinaryWriter writer);

        /// <summary>
        /// Verifies the attribute with the transaction.
        /// </summary>
        /// <param name="snapshot">The snapshot used to verify the attribute.</param>
        /// <param name="tx">The <see cref="Transaction"/> that contains the attribute.</param>
        /// <returns><see langword="true"/> if the verification passes; otherwise, <see langword="false"/>.</returns>
        public virtual bool Verify(DataCache snapshot, Transaction tx) => true;

        public virtual long CalculateNetworkFee(DataCache snapshot, Transaction tx) => NativeContract.Policy.GetAttributeFee(snapshot, (byte)Type);
    }
}
