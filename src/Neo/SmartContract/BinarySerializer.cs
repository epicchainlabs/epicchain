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
// BinarySerializer.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.IO;
using Neo.VM;
using Neo.VM.Types;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Numerics;
using Array = Neo.VM.Types.Array;
using Boolean = Neo.VM.Types.Boolean;
using Buffer = Neo.VM.Types.Buffer;

namespace Neo.SmartContract
{
    /// <summary>
    /// A binary serializer for <see cref="StackItem"/>.
    /// </summary>
    public static class BinarySerializer
    {
        private class ContainerPlaceholder : StackItem
        {
            public override StackItemType Type { get; }
            public int ElementCount { get; }

            public ContainerPlaceholder(StackItemType type, int count)
            {
                Type = type;
                ElementCount = count;
            }

            public override bool Equals(StackItem other) => throw new NotSupportedException();

            public override int GetHashCode() => throw new NotSupportedException();

            public override bool GetBoolean() => throw new NotSupportedException();
        }

        /// <summary>
        /// Deserializes a <see cref="StackItem"/> from byte array.
        /// </summary>
        /// <param name="data">The byte array to parse.</param>
        /// <param name="limits">The limits for the deserialization.</param>
        /// <param name="referenceCounter">The <see cref="ReferenceCounter"/> used by the <see cref="StackItem"/>.</param>
        /// <returns>The deserialized <see cref="StackItem"/>.</returns>
        public static StackItem Deserialize(ReadOnlyMemory<byte> data, ExecutionEngineLimits limits, ReferenceCounter referenceCounter = null)
        {
            MemoryReader reader = new(data);
            return Deserialize(ref reader, (uint)Math.Min(data.Length, limits.MaxItemSize), limits.MaxStackSize, referenceCounter);
        }

        /// <summary>
        /// Deserializes a <see cref="StackItem"/> from <see cref="MemoryReader"/>.
        /// </summary>
        /// <param name="reader">The <see cref="MemoryReader"/> for reading data.</param>
        /// <param name="limits">The limits for the deserialization.</param>
        /// <param name="referenceCounter">The <see cref="ReferenceCounter"/> used by the <see cref="StackItem"/>.</param>
        /// <returns>The deserialized <see cref="StackItem"/>.</returns>
        public static StackItem Deserialize(ref MemoryReader reader, ExecutionEngineLimits limits, ReferenceCounter referenceCounter = null)
        {
            return Deserialize(ref reader, limits.MaxItemSize, limits.MaxStackSize, referenceCounter);
        }

        /// <summary>
        /// Deserializes a <see cref="StackItem"/> from <see cref="MemoryReader"/>.
        /// </summary>
        /// <param name="reader">The <see cref="MemoryReader"/> for reading data.</param>
        /// <param name="maxSize">The maximum size of the result.</param>
        /// <param name="maxItems">The max of items to serialize</param>
        /// <param name="referenceCounter">The <see cref="ReferenceCounter"/> used by the <see cref="StackItem"/>.</param>
        /// <returns>The deserialized <see cref="StackItem"/>.</returns>
        public static StackItem Deserialize(ref MemoryReader reader, uint maxSize, uint maxItems, ReferenceCounter referenceCounter = null)
        {
            Stack<StackItem> deserialized = new();
            int undeserialized = 1;
            while (undeserialized-- > 0)
            {
                StackItemType type = (StackItemType)reader.ReadByte();
                switch (type)
                {
                    case StackItemType.Any:
                        deserialized.Push(StackItem.Null);
                        break;
                    case StackItemType.Boolean:
                        deserialized.Push(reader.ReadBoolean());
                        break;
                    case StackItemType.Integer:
                        deserialized.Push(new BigInteger(reader.ReadVarMemory(Integer.MaxSize).Span));
                        break;
                    case StackItemType.ByteString:
                        deserialized.Push(reader.ReadVarMemory((int)maxSize));
                        break;
                    case StackItemType.Buffer:
                        ReadOnlyMemory<byte> memory = reader.ReadVarMemory((int)maxSize);
                        deserialized.Push(new Buffer(memory.Span));
                        break;
                    case StackItemType.Array:
                    case StackItemType.Struct:
                        {
                            int count = (int)reader.ReadVarInt(maxItems);
                            deserialized.Push(new ContainerPlaceholder(type, count));
                            undeserialized += count;
                        }
                        break;
                    case StackItemType.Map:
                        {
                            int count = (int)reader.ReadVarInt(maxItems);
                            deserialized.Push(new ContainerPlaceholder(type, count));
                            undeserialized += count * 2;
                        }
                        break;
                    default:
                        throw new FormatException();
                }
                if (deserialized.Count > maxItems)
                    throw new FormatException();
            }
            Stack<StackItem> stack_temp = new();
            while (deserialized.Count > 0)
            {
                StackItem item = deserialized.Pop();
                if (item is ContainerPlaceholder placeholder)
                {
                    switch (placeholder.Type)
                    {
                        case StackItemType.Array:
                            Array array = new(referenceCounter);
                            for (int i = 0; i < placeholder.ElementCount; i++)
                                array.Add(stack_temp.Pop());
                            item = array;
                            break;
                        case StackItemType.Struct:
                            Struct @struct = new(referenceCounter);
                            for (int i = 0; i < placeholder.ElementCount; i++)
                                @struct.Add(stack_temp.Pop());
                            item = @struct;
                            break;
                        case StackItemType.Map:
                            Map map = new(referenceCounter);
                            for (int i = 0; i < placeholder.ElementCount; i++)
                            {
                                StackItem key = stack_temp.Pop();
                                StackItem value = stack_temp.Pop();
                                map[(PrimitiveType)key] = value;
                            }
                            item = map;
                            break;
                    }
                }
                stack_temp.Push(item);
            }
            return stack_temp.Peek();
        }

        /// <summary>
        /// Serializes a <see cref="StackItem"/> to byte array.
        /// </summary>
        /// <param name="item">The <see cref="StackItem"/> to be serialized.</param>
        /// <param name="limits">The <see cref="ExecutionEngineLimits"/> used to ensure the limits.</param>
        /// <returns>The serialized byte array.</returns>
        public static byte[] Serialize(StackItem item, ExecutionEngineLimits limits)
        {
            return Serialize(item, limits.MaxItemSize, limits.MaxStackSize);
        }

        /// <summary>
        /// Serializes a <see cref="StackItem"/> to byte array.
        /// </summary>
        /// <param name="item">The <see cref="StackItem"/> to be serialized.</param>
        /// <param name="maxSize">The maximum size of the result.</param>
        /// <param name="maxItems">The max of items to serialize</param>
        /// <returns>The serialized byte array.</returns>
        public static byte[] Serialize(StackItem item, long maxSize, long maxItems)
        {
            using MemoryStream ms = new();
            using BinaryWriter writer = new(ms, Utility.StrictUTF8, true);
            Serialize(writer, item, maxSize, maxItems);
            writer.Flush();
            return ms.ToArray();
        }

        /// <summary>
        /// Serializes a <see cref="StackItem"/> into <see cref="BinaryWriter"/>.
        /// </summary>
        /// <param name="writer">The <see cref="BinaryWriter"/> for writing data.</param>
        /// <param name="item">The <see cref="StackItem"/> to be serialized.</param>
        /// <param name="maxSize">The maximum size of the result.</param>
        /// <param name="maxItems">The max of items to serialize</param>
        public static void Serialize(BinaryWriter writer, StackItem item, long maxSize, long maxItems)
        {
            HashSet<CompoundType> serialized = new(ReferenceEqualityComparer.Instance);
            Stack<StackItem> unserialized = new();
            unserialized.Push(item);
            while (unserialized.Count > 0)
            {
                if (--maxItems < 0)
                    throw new FormatException();
                item = unserialized.Pop();
                writer.Write((byte)item.Type);
                switch (item)
                {
                    case Null _:
                        break;
                    case Boolean _:
                        writer.Write(item.GetBoolean());
                        break;
                    case Integer _:
                    case ByteString _:
                    case Buffer _:
                        writer.WriteVarBytes(item.GetSpan());
                        break;
                    case Array array:
                        if (!serialized.Add(array))
                            throw new NotSupportedException();
                        writer.WriteVarInt(array.Count);
                        for (int i = array.Count - 1; i >= 0; i--)
                            unserialized.Push(array[i]);
                        break;
                    case Map map:
                        if (!serialized.Add(map))
                            throw new NotSupportedException();
                        writer.WriteVarInt(map.Count);
                        foreach (var pair in map.Reverse())
                        {
                            unserialized.Push(pair.Value);
                            unserialized.Push(pair.Key);
                        }
                        break;
                    default:
                        throw new NotSupportedException();
                }
                writer.Flush();
                if (writer.BaseStream.Position > maxSize)
                    throw new InvalidOperationException();
            }
        }
    }
}
