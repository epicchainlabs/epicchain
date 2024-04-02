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
// JumpTable.Compound.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.VM.Types;
using System;
using System.Linq;
using System.Numerics;
using System.Runtime.CompilerServices;
using VMArray = Neo.VM.Types.Array;

namespace Neo.VM
{
    public partial class JumpTable
    {
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void PackMap(ExecutionEngine engine, Instruction instruction)
        {
            var size = (int)engine.Pop().GetInteger();
            if (size < 0 || size * 2 > engine.CurrentContext!.EvaluationStack.Count)
                throw new InvalidOperationException($"The value {size} is out of range.");
            Map map = new(engine.ReferenceCounter);
            for (var i = 0; i < size; i++)
            {
                var key = engine.Pop<PrimitiveType>();
                var value = engine.Pop();
                map[key] = value;
            }
            engine.Push(map);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void PackStruct(ExecutionEngine engine, Instruction instruction)
        {
            var size = (int)engine.Pop().GetInteger();
            if (size < 0 || size > engine.CurrentContext!.EvaluationStack.Count)
                throw new InvalidOperationException($"The value {size} is out of range.");
            Struct @struct = new(engine.ReferenceCounter);
            for (var i = 0; i < size; i++)
            {
                var item = engine.Pop();
                @struct.Add(item);
            }
            engine.Push(@struct);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Pack(ExecutionEngine engine, Instruction instruction)
        {
            var size = (int)engine.Pop().GetInteger();
            if (size < 0 || size > engine.CurrentContext!.EvaluationStack.Count)
                throw new InvalidOperationException($"The value {size} is out of range.");
            VMArray array = new(engine.ReferenceCounter);
            for (var i = 0; i < size; i++)
            {
                var item = engine.Pop();
                array.Add(item);
            }
            engine.Push(array);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Unpack(ExecutionEngine engine, Instruction instruction)
        {
            var compound = engine.Pop<CompoundType>();
            switch (compound)
            {
                case Map map:
                    foreach (var (key, value) in map.Reverse())
                    {
                        engine.Push(value);
                        engine.Push(key);
                    }
                    break;
                case VMArray array:
                    for (var i = array.Count - 1; i >= 0; i--)
                    {
                        engine.Push(array[i]);
                    }
                    break;
                default:
                    throw new InvalidOperationException($"Invalid type for {instruction.OpCode}: {compound.Type}");
            }
            engine.Push(compound.Count);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void NewArray0(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(new VMArray(engine.ReferenceCounter));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void NewArray(ExecutionEngine engine, Instruction instruction)
        {
            var n = (int)engine.Pop().GetInteger();
            if (n < 0 || n > engine.Limits.MaxStackSize)
                throw new InvalidOperationException($"MaxStackSize exceed: {n}");

            engine.Push(new VMArray(engine.ReferenceCounter, Enumerable.Repeat(StackItem.Null, n)));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void NewArray_T(ExecutionEngine engine, Instruction instruction)
        {
            var n = (int)engine.Pop().GetInteger();
            if (n < 0 || n > engine.Limits.MaxStackSize)
                throw new InvalidOperationException($"MaxStackSize exceed: {n}");

            var type = (StackItemType)instruction.TokenU8;
            if (!Enum.IsDefined(typeof(StackItemType), type))
                throw new InvalidOperationException($"Invalid type for {instruction.OpCode}: {instruction.TokenU8}");

            var item = instruction.TokenU8 switch
            {
                (byte)StackItemType.Boolean => StackItem.False,
                (byte)StackItemType.Integer => Integer.Zero,
                (byte)StackItemType.ByteString => ByteString.Empty,
                _ => StackItem.Null
            };

            engine.Push(new VMArray(engine.ReferenceCounter, Enumerable.Repeat(item, n)));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void NewStruct0(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(new Struct(engine.ReferenceCounter));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void NewStruct(ExecutionEngine engine, Instruction instruction)
        {
            var n = (int)engine.Pop().GetInteger();
            if (n < 0 || n > engine.Limits.MaxStackSize)
                throw new InvalidOperationException($"MaxStackSize exceed: {n}");
            Struct result = new(engine.ReferenceCounter);
            for (var i = 0; i < n; i++)
                result.Add(StackItem.Null);
            engine.Push(result);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void NewMap(ExecutionEngine engine, Instruction instruction)
        {
            engine.Push(new Map(engine.ReferenceCounter));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Size(ExecutionEngine engine, Instruction instruction)
        {
            var x = engine.Pop();
            switch (x)
            {
                case CompoundType compound:
                    engine.Push(compound.Count);
                    break;
                case PrimitiveType primitive:
                    engine.Push(primitive.Size);
                    break;
                case Types.Buffer buffer:
                    engine.Push(buffer.Size);
                    break;
                default:
                    throw new InvalidOperationException($"Invalid type for {instruction.OpCode}: {x.Type}");
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void HasKey(ExecutionEngine engine, Instruction instruction)
        {
            var key = engine.Pop<PrimitiveType>();
            var x = engine.Pop();
            switch (x)
            {
                case VMArray array:
                    {
                        var index = (int)key.GetInteger();
                        if (index < 0)
                            throw new InvalidOperationException($"The negative value {index} is invalid for OpCode.{instruction.OpCode}.");
                        engine.Push(index < array.Count);
                        break;
                    }
                case Map map:
                    {
                        engine.Push(map.ContainsKey(key));
                        break;
                    }
                case Types.Buffer buffer:
                    {
                        var index = (int)key.GetInteger();
                        if (index < 0)
                            throw new InvalidOperationException($"The negative value {index} is invalid for OpCode.{instruction.OpCode}.");
                        engine.Push(index < buffer.Size);
                        break;
                    }
                case ByteString array:
                    {
                        var index = (int)key.GetInteger();
                        if (index < 0)
                            throw new InvalidOperationException($"The negative value {index} is invalid for OpCode.{instruction.OpCode}.");
                        engine.Push(index < array.Size);
                        break;
                    }
                default:
                    throw new InvalidOperationException($"Invalid type for {instruction.OpCode}: {x.Type}");
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Keys(ExecutionEngine engine, Instruction instruction)
        {
            var map = engine.Pop<Map>();
            engine.Push(new VMArray(engine.ReferenceCounter, map.Keys));
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Values(ExecutionEngine engine, Instruction instruction)
        {
            var x = engine.Pop();
            var values = x switch
            {
                VMArray array => array,
                Map map => map.Values,
                _ => throw new InvalidOperationException($"Invalid type for {instruction.OpCode}: {x.Type}"),
            };
            VMArray newArray = new(engine.ReferenceCounter);
            foreach (var item in values)
                if (item is Struct s)
                    newArray.Add(s.Clone(engine.Limits));
                else
                    newArray.Add(item);
            engine.Push(newArray);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void PickItem(ExecutionEngine engine, Instruction instruction)
        {
            var key = engine.Pop<PrimitiveType>();
            var x = engine.Pop();
            switch (x)
            {
                case VMArray array:
                    {
                        var index = (int)key.GetInteger();
                        if (index < 0 || index >= array.Count)
                            throw new CatchableException($"The value {index} is out of range.");
                        engine.Push(array[index]);
                        break;
                    }
                case Map map:
                    {
                        if (!map.TryGetValue(key, out var value))
                            throw new CatchableException($"Key not found in {nameof(Map)}");
                        engine.Push(value);
                        break;
                    }
                case PrimitiveType primitive:
                    {
                        var byteArray = primitive.GetSpan();
                        var index = (int)key.GetInteger();
                        if (index < 0 || index >= byteArray.Length)
                            throw new CatchableException($"The value {index} is out of range.");
                        engine.Push((BigInteger)byteArray[index]);
                        break;
                    }
                case Types.Buffer buffer:
                    {
                        var index = (int)key.GetInteger();
                        if (index < 0 || index >= buffer.Size)
                            throw new CatchableException($"The value {index} is out of range.");
                        engine.Push((BigInteger)buffer.InnerBuffer.Span[index]);
                        break;
                    }
                default:
                    throw new InvalidOperationException($"Invalid type for {instruction.OpCode}: {x.Type}");
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Append(ExecutionEngine engine, Instruction instruction)
        {
            var newItem = engine.Pop();
            var array = engine.Pop<VMArray>();
            if (newItem is Struct s) newItem = s.Clone(engine.Limits);
            array.Add(newItem);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void SetItem(ExecutionEngine engine, Instruction instruction)
        {
            var value = engine.Pop();
            if (value is Struct s) value = s.Clone(engine.Limits);
            var key = engine.Pop<PrimitiveType>();
            var x = engine.Pop();
            switch (x)
            {
                case VMArray array:
                    {
                        var index = (int)key.GetInteger();
                        if (index < 0 || index >= array.Count)
                            throw new CatchableException($"The value {index} is out of range.");
                        array[index] = value;
                        break;
                    }
                case Map map:
                    {
                        map[key] = value;
                        break;
                    }
                case Types.Buffer buffer:
                    {
                        var index = (int)key.GetInteger();
                        if (index < 0 || index >= buffer.Size)
                            throw new CatchableException($"The value {index} is out of range.");
                        if (value is not PrimitiveType p)
                            throw new InvalidOperationException($"Value must be a primitive type in {instruction.OpCode}");
                        var b = (int)p.GetInteger();
                        if (b < sbyte.MinValue || b > byte.MaxValue)
                            throw new InvalidOperationException($"Overflow in {instruction.OpCode}, {b} is not a byte type.");
                        buffer.InnerBuffer.Span[index] = (byte)b;
                        break;
                    }
                default:
                    throw new InvalidOperationException($"Invalid type for {instruction.OpCode}: {x.Type}");
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void ReverseItems(ExecutionEngine engine, Instruction instruction)
        {
            var x = engine.Pop();
            switch (x)
            {
                case VMArray array:
                    array.Reverse();
                    break;
                case Types.Buffer buffer:
                    buffer.InnerBuffer.Span.Reverse();
                    break;
                default:
                    throw new InvalidOperationException($"Invalid type for {instruction.OpCode}: {x.Type}");
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void Remove(ExecutionEngine engine, Instruction instruction)
        {
            var key = engine.Pop<PrimitiveType>();
            var x = engine.Pop();
            switch (x)
            {
                case VMArray array:
                    var index = (int)key.GetInteger();
                    if (index < 0 || index >= array.Count)
                        throw new InvalidOperationException($"The value {index} is out of range.");
                    array.RemoveAt(index);
                    break;
                case Map map:
                    map.Remove(key);
                    break;
                default:
                    throw new InvalidOperationException($"Invalid type for {instruction.OpCode}: {x.Type}");
            }
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void ClearItems(ExecutionEngine engine, Instruction instruction)
        {
            var x = engine.Pop<CompoundType>();
            x.Clear();
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public virtual void PopItem(ExecutionEngine engine, Instruction instruction)
        {
            var x = engine.Pop<VMArray>();
            var index = x.Count - 1;
            engine.Push(x[index]);
            x.RemoveAt(index);
        }
    }
}
