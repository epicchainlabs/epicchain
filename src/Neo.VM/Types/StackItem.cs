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
// StackItem.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

#pragma warning disable CS0659

using System;
using System.Collections.Generic;
using System.Diagnostics.CodeAnalysis;
using System.Numerics;
using System.Runtime.CompilerServices;

namespace Neo.VM.Types
{
    /// <summary>
    /// The base class for all types in the VM.
    /// </summary>
    public abstract partial class StackItem : IEquatable<StackItem>
    {
        [ThreadStatic]
        private static Boolean? tls_true = null;

        /// <summary>
        /// Represents <see langword="true"/> in the VM.
        /// </summary>
        public static Boolean True
        {
            get
            {
                tls_true ??= new(true);
                return tls_true;
            }
        }

        [ThreadStatic]
        private static Boolean? tls_false = null;

        /// <summary>
        /// Represents <see langword="false"/> in the VM.
        /// </summary>
        public static Boolean False
        {
            get
            {
                tls_false ??= new(false);
                return tls_false;
            }
        }

        [ThreadStatic]
        private static Null? tls_null = null;

        /// <summary>
        /// Represents <see langword="null"/> in the VM.
        /// </summary>
        public static StackItem Null
        {
            get
            {
                tls_null ??= new();
                return tls_null;
            }
        }

        /// <summary>
        /// Indicates whether the object is <see cref="Null"/>.
        /// </summary>
        public bool IsNull => this is Null;

        /// <summary>
        /// The type of this VM object.
        /// </summary>
        public abstract StackItemType Type { get; }

        /// <summary>
        /// Convert the VM object to the specified type.
        /// </summary>
        /// <param name="type">The type to be converted to.</param>
        /// <returns>The converted object.</returns>
        public virtual StackItem ConvertTo(StackItemType type)
        {
            if (type == Type) return this;
            if (type == StackItemType.Boolean) return GetBoolean();
            throw new InvalidCastException();
        }

        internal virtual void Cleanup()
        {
        }

        /// <summary>
        /// Copy the object and all its children.
        /// </summary>
        /// <returns>The copied object.</returns>
        public StackItem DeepCopy(bool asImmutable = false)
        {
            return DeepCopy(new(ReferenceEqualityComparer.Instance), asImmutable);
        }

        internal virtual StackItem DeepCopy(Dictionary<StackItem, StackItem> refMap, bool asImmutable)
        {
            return this;
        }

        public sealed override bool Equals(object? obj)
        {
            if (ReferenceEquals(this, obj)) return true;
            if (obj is StackItem item) return Equals(item);
            return false;
        }

        public virtual bool Equals(StackItem? other)
        {
            return ReferenceEquals(this, other);
        }

        internal virtual bool Equals(StackItem? other, ExecutionEngineLimits limits)
        {
            return Equals(other);
        }

        /// <summary>
        /// Wrap the specified <see cref="object"/> and return an <see cref="InteropInterface"/> containing the <see cref="object"/>.
        /// </summary>
        /// <param name="value">The wrapped <see cref="object"/>.</param>
        /// <returns></returns>
        public static StackItem FromInterface(object? value)
        {
            if (value is null) return Null;
            return new InteropInterface(value);
        }

        /// <summary>
        /// Get the boolean value represented by the VM object.
        /// </summary>
        /// <returns>The boolean value represented by the VM object.</returns>
        public abstract bool GetBoolean();

        /// <summary>
        /// Get the integer value represented by the VM object.
        /// </summary>
        /// <returns>The integer value represented by the VM object.</returns>
        public virtual BigInteger GetInteger()
        {
            throw new InvalidCastException();
        }

        /// <summary>
        /// Get the <see cref="object"/> wrapped by this interface and convert it to the specified type.
        /// </summary>
        /// <typeparam name="T">The type to convert to.</typeparam>
        /// <returns>The wrapped <see cref="object"/>.</returns>
        [return: MaybeNull]
        public virtual T GetInterface<T>() where T : notnull
        {
            throw new InvalidCastException();
        }

        /// <summary>
        /// Get the readonly span used to read the VM object data.
        /// </summary>
        /// <returns></returns>
        public virtual ReadOnlySpan<byte> GetSpan()
        {
            throw new InvalidCastException();
        }

        /// <summary>
        /// Get the <see cref="string"/> value represented by the VM object.
        /// </summary>
        /// <returns>The <see cref="string"/> value represented by the VM object.</returns>
        public virtual string? GetString()
        {
            return Utility.StrictUTF8.GetString(GetSpan());
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator StackItem(sbyte value)
        {
            return (Integer)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator StackItem(byte value)
        {
            return (Integer)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator StackItem(short value)
        {
            return (Integer)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator StackItem(ushort value)
        {
            return (Integer)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator StackItem(int value)
        {
            return (Integer)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator StackItem(uint value)
        {
            return (Integer)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator StackItem(long value)
        {
            return (Integer)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator StackItem(ulong value)
        {
            return (Integer)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator StackItem(BigInteger value)
        {
            return (Integer)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator StackItem(bool value)
        {
            return value ? True : False;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator StackItem(byte[] value)
        {
            return (ByteString)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator StackItem(ReadOnlyMemory<byte> value)
        {
            return (ByteString)value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator StackItem(string value)
        {
            return (ByteString)value;
        }
    }
}
