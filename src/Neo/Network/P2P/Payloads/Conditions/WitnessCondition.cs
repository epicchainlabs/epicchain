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
// WitnessCondition.cs file belongs to the neo project and is free
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
using Neo.SmartContract;
using Neo.VM;
using Neo.VM.Types;
using System;
using System.IO;

namespace Neo.Network.P2P.Payloads.Conditions
{
    public abstract class WitnessCondition : IInteroperable, ISerializable
    {
        internal const int MaxSubitems = 16;
        internal const int MaxNestingDepth = 2;

        /// <summary>
        /// The type of the <see cref="WitnessCondition"/>.
        /// </summary>
        public abstract WitnessConditionType Type { get; }

        public virtual int Size => sizeof(WitnessConditionType);

        void ISerializable.Deserialize(ref MemoryReader reader)
        {
            if (reader.ReadByte() != (byte)Type) throw new FormatException();
            DeserializeWithoutType(ref reader, MaxNestingDepth);
        }

        /// <summary>
        /// Deserializes an <see cref="WitnessCondition"/> array from a <see cref="MemoryReader"/>.
        /// </summary>
        /// <param name="reader">The <see cref="MemoryReader"/> for reading data.</param>
        /// <param name="maxNestDepth">The maximum nesting depth allowed during deserialization.</param>
        /// <returns>The deserialized <see cref="WitnessCondition"/> array.</returns>
        protected static WitnessCondition[] DeserializeConditions(ref MemoryReader reader, int maxNestDepth)
        {
            WitnessCondition[] conditions = new WitnessCondition[reader.ReadVarInt(MaxSubitems)];
            for (int i = 0; i < conditions.Length; i++)
                conditions[i] = DeserializeFrom(ref reader, maxNestDepth);
            return conditions;
        }

        /// <summary>
        /// Deserializes an <see cref="WitnessCondition"/> object from a <see cref="MemoryReader"/>.
        /// </summary>
        /// <param name="reader">The <see cref="MemoryReader"/> for reading data.</param>
        /// <param name="maxNestDepth">The maximum nesting depth allowed during deserialization.</param>
        /// <returns>The deserialized <see cref="WitnessCondition"/>.</returns>
        public static WitnessCondition DeserializeFrom(ref MemoryReader reader, int maxNestDepth)
        {
            WitnessConditionType type = (WitnessConditionType)reader.ReadByte();
            if (ReflectionCache<WitnessConditionType>.CreateInstance(type) is not WitnessCondition condition)
                throw new FormatException();
            condition.DeserializeWithoutType(ref reader, maxNestDepth);
            return condition;
        }

        /// <summary>
        /// Deserializes the <see cref="WitnessCondition"/> object from a <see cref="MemoryReader"/>.
        /// </summary>
        /// <param name="reader">The <see cref="MemoryReader"/> for reading data.</param>
        /// <param name="maxNestDepth">The maximum nesting depth allowed during deserialization.</param>
        protected abstract void DeserializeWithoutType(ref MemoryReader reader, int maxNestDepth);

        /// <summary>
        /// Checks whether the current context matches the condition.
        /// </summary>
        /// <param name="engine">The <see cref="ApplicationEngine"/> that is executing CheckWitness.</param>
        /// <returns><see langword="true"/> if the condition matches; otherwise, <see langword="false"/>.</returns>
        public abstract bool Match(ApplicationEngine engine);

        void ISerializable.Serialize(BinaryWriter writer)
        {
            writer.Write((byte)Type);
            SerializeWithoutType(writer);
        }

        /// <summary>
        /// Serializes the <see cref="WitnessCondition"/> object to a <see cref="BinaryWriter"/>.
        /// </summary>
        /// <param name="writer">The <see cref="BinaryWriter"/> for writing data.</param>
        protected abstract void SerializeWithoutType(BinaryWriter writer);

        private protected abstract void ParseJson(JObject json, int maxNestDepth);

        /// <summary>
        /// Converts the <see cref="WitnessCondition"/> from a JSON object.
        /// </summary>
        /// <param name="json">The <see cref="WitnessCondition"/> represented by a JSON object.</param>
        /// <param name="maxNestDepth">The maximum nesting depth allowed during deserialization.</param>
        /// <returns>The converted <see cref="WitnessCondition"/>.</returns>
        public static WitnessCondition FromJson(JObject json, int maxNestDepth)
        {
            WitnessConditionType type = Enum.Parse<WitnessConditionType>(json["type"].GetString());
            if (ReflectionCache<WitnessConditionType>.CreateInstance(type) is not WitnessCondition condition)
                throw new FormatException("Invalid WitnessConditionType.");
            condition.ParseJson(json, maxNestDepth);
            return condition;
        }

        /// <summary>
        /// Converts the condition to a JSON object.
        /// </summary>
        /// <returns>The condition represented by a JSON object.</returns>
        public virtual JObject ToJson()
        {
            return new JObject
            {
                ["type"] = Type
            };
        }

        void IInteroperable.FromStackItem(StackItem stackItem)
        {
            throw new NotSupportedException();
        }

        public virtual StackItem ToStackItem(ReferenceCounter referenceCounter)
        {
            return new VM.Types.Array(referenceCounter, new StackItem[] { (byte)Type });
        }
    }
}
