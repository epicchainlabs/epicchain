// Copyright (C) 2021-2024 EpicChain Labs.

//
// StackItem.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


#pragma warning disable CS0659

using System;
using System.Collections.Generic;
using System.Diagnostics.CodeAnalysis;
using System.Numerics;
using System.Runtime.CompilerServices;

namespace EpicChain.VM.Types
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
