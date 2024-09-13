// Copyright (C) 2021-2024 EpicChain Labs.

//
// ByteString.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.VM.Cryptography;
using System;
using System.Buffers.Binary;
using System.Diagnostics;
using System.Numerics;
using System.Runtime.CompilerServices;

namespace EpicChain.VM.Types
{
    /// <summary>
    /// Represents an immutable memory block in the VM.
    /// </summary>
    [DebuggerDisplay("Type={GetType().Name}, Value={System.Convert.ToHexString(GetSpan())}")]
    public class ByteString : PrimitiveType
    {
        /// <summary>
        /// An empty <see cref="ByteString"/>.
        /// </summary>
        public static readonly ByteString Empty = ReadOnlyMemory<byte>.Empty;

        private static readonly uint s_seed = unchecked((uint)new Random().Next());
        private int _hashCode = 0;

        public override ReadOnlyMemory<byte> Memory { get; }
        public override StackItemType Type => StackItemType.ByteString;

        /// <summary>
        /// Create a new <see cref="ByteString"/> with the specified data.
        /// </summary>
        /// <param name="data">The data to be contained in this <see cref="ByteString"/>.</param>
        public ByteString(ReadOnlyMemory<byte> data)
        {
            Memory = data;
        }

        private bool Equals(ByteString other)
        {
            return GetSpan().SequenceEqual(other.GetSpan());
        }

        public override bool Equals(StackItem? other)
        {
            if (ReferenceEquals(this, other)) return true;
            if (other is not ByteString b) return false;
            return Equals(b);
        }

        internal override bool Equals(StackItem? other, ExecutionEngineLimits limits)
        {
            uint maxComparableSize = limits.MaxComparableSize;
            return Equals(other, ref maxComparableSize);
        }

        internal bool Equals(StackItem? other, ref uint limits)
        {
            if (Size > limits || limits == 0)
                throw new InvalidOperationException("The operand exceeds the maximum comparable size.");
            uint comparedSize = 1;
            try
            {
                if (other is not ByteString b) return false;
                comparedSize = Math.Max((uint)Math.Max(Size, b.Size), comparedSize);
                if (ReferenceEquals(this, b)) return true;
                if (b.Size > limits)
                    throw new InvalidOperationException("The operand exceeds the maximum comparable size.");
                return Equals(b);
            }
            finally
            {
                limits -= comparedSize;
            }
        }

        public override bool GetBoolean()
        {
            if (Size > Integer.MaxSize) throw new InvalidCastException();
            return Unsafe.NotZero(GetSpan());
        }

        public override int GetHashCode()
        {
            if (_hashCode == 0)
            {
                using Murmur32 murmur = new(s_seed);
                _hashCode = BinaryPrimitives.ReadInt32LittleEndian(murmur.ComputeHash(GetSpan().ToArray()));
            }
            return _hashCode;
        }

        public override BigInteger GetInteger()
        {
            if (Size > Integer.MaxSize) throw new InvalidCastException($"MaxSize exceed: {Size}");
            return new BigInteger(GetSpan());
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator ReadOnlyMemory<byte>(ByteString value)
        {
            return value.Memory;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator ReadOnlySpan<byte>(ByteString value)
        {
            return value.Memory.Span;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator ByteString(byte[] value)
        {
            return new ByteString(value);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator ByteString(ReadOnlyMemory<byte> value)
        {
            return new ByteString(value);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator ByteString(string value)
        {
            return new ByteString(Utility.StrictUTF8.GetBytes(value));
        }

        public override string ToString()
        {
            return GetSpan().TryGetString(out var str) ? $"\"{str}\"" : $"\"Base64: {Convert.ToBase64String(GetSpan())}\"";
        }
    }
}
