// Copyright (C) 2021-2024 EpicChain Labs.

//
// Integer.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Diagnostics;
using System.Numerics;
using System.Runtime.CompilerServices;

namespace EpicChain.VM.Types
{
    /// <summary>
    /// Represents an integer value in the VM.
    /// </summary>
    [DebuggerDisplay("Type={GetType().Name}, Value={value}")]
    public class Integer : PrimitiveType
    {
        /// <summary>
        /// The maximum size of an integer in bytes.
        /// </summary>
        public const int MaxSize = 32;

        /// <summary>
        /// Represents the number 0.
        /// </summary>
        public static readonly Integer Zero = 0;
        private readonly BigInteger value;

        public override ReadOnlyMemory<byte> Memory => value.IsZero ? ReadOnlyMemory<byte>.Empty : value.ToByteArray();
        public override int Size { get; }
        public override StackItemType Type => StackItemType.Integer;

        /// <summary>
        /// Create an integer with the specified value.
        /// </summary>
        /// <param name="value">The value of the integer.</param>
        public Integer(BigInteger value)
        {
            if (value.IsZero)
            {
                Size = 0;
            }
            else
            {
                Size = value.GetByteCount();
                if (Size > MaxSize) throw new ArgumentException($"MaxSize exceed: {Size}");
            }
            this.value = value;
        }

        public override bool Equals(StackItem? other)
        {
            if (ReferenceEquals(this, other)) return true;
            if (other is Integer i) return value == i.value;
            return false;
        }

        public override bool GetBoolean()
        {
            return !value.IsZero;
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(value);
        }

        public override BigInteger GetInteger()
        {
            return value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator Integer(sbyte value)
        {
            return (BigInteger)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator Integer(byte value)
        {
            return (BigInteger)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator Integer(short value)
        {
            return (BigInteger)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator Integer(ushort value)
        {
            return (BigInteger)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator Integer(int value)
        {
            return (BigInteger)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator Integer(uint value)
        {
            return (BigInteger)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator Integer(long value)
        {
            return (BigInteger)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator Integer(ulong value)
        {
            return (BigInteger)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator Integer(BigInteger value)
        {
            return new Integer(value);
        }

        public override string ToString()
        {
            return value.ToString();
        }
    }
}
