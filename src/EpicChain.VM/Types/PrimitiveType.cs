// Copyright (C) 2021-2024 EpicChain Labs.

//
// PrimitiveType.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
// distributed as free software under the MIT License, allowing for wide usage and modification
// with minimal restrictions. For comprehensive details regarding the license, please refer to
// the LICENSE file located in the root directory of the repository or visit
// http://www.opensource.org/licenses/mit-license.php.
//
// EpicChain Labs is dedicated to fostering innovation and development in the blockchain space,
// and we believe in the open-source philosophy as a way to drive progress and collaboration.
// This file, along with all associated code and documentation, is provided with the intention of
// supporting and enhancing the development community.
//
// Redistribution and use of this file in both source and binary forms, with or without
// modifications, are permitted. We encourage users to contribute to the project and respect the
// guidelines outlined in the LICENSE file. By using this software, you agree to the terms and
// conditions specified in the MIT License, ensuring the continuation of free and open software
// practices.


using System;
using System.Collections.Generic;
using System.Numerics;
using System.Runtime.CompilerServices;

namespace EpicChain.VM.Types
{
    /// <summary>
    /// The base class for primitive types in the VM.
    /// </summary>
    public abstract class PrimitiveType : StackItem
    {
        public abstract ReadOnlyMemory<byte> Memory { get; }

        /// <summary>
        /// The size of the VM object in bytes.
        /// </summary>
        public virtual int Size => Memory.Length;

        public override StackItem ConvertTo(StackItemType type)
        {
            if (type == Type) return this;
            return type switch
            {
                StackItemType.Integer => GetInteger(),
                StackItemType.ByteString => Memory,
                StackItemType.Buffer => new Buffer(GetSpan()),
                _ => base.ConvertTo(type)
            };
        }

        internal sealed override StackItem DeepCopy(Dictionary<StackItem, StackItem> refMap, bool asImmutable)
        {
            return this;
        }

        public abstract override bool Equals(StackItem? other);

        /// <summary>
        /// Get the hash code of the VM object, which is used for key comparison in the <see cref="Map"/>.
        /// </summary>
        /// <returns>The hash code of this VM object.</returns>
        public abstract override int GetHashCode();

        public sealed override ReadOnlySpan<byte> GetSpan()
        {
            return Memory.Span;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator PrimitiveType(sbyte value)
        {
            return (Integer)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator PrimitiveType(byte value)
        {
            return (Integer)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator PrimitiveType(short value)
        {
            return (Integer)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator PrimitiveType(ushort value)
        {
            return (Integer)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator PrimitiveType(int value)
        {
            return (Integer)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator PrimitiveType(uint value)
        {
            return (Integer)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator PrimitiveType(long value)
        {
            return (Integer)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator PrimitiveType(ulong value)
        {
            return (Integer)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator PrimitiveType(BigInteger value)
        {
            return (Integer)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator PrimitiveType(bool value)
        {
            return (Boolean)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator PrimitiveType(byte[] value)
        {
            return (ByteString)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator PrimitiveType(ReadOnlyMemory<byte> value)
        {
            return (ByteString)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator PrimitiveType(string value)
        {
            return (ByteString)value;
        }
    }
}
