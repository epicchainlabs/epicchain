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
// ByteString.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.VM.Cryptography;
using System;
using System.Buffers.Binary;
using System.Diagnostics;
using System.Numerics;
using System.Runtime.CompilerServices;

namespace Neo.VM.Types
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
            this.Memory = data;
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
