// Copyright (C) 2021-2024 EpicChain Labs.

//
// Boolean.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
    /// Represents a boolean (<see langword="true" /> or <see langword="false" />) value in the VM.
    /// </summary>
    [DebuggerDisplay("Type={GetType().Name}, Value={value}")]
    public class Boolean : PrimitiveType
    {
        private static readonly ReadOnlyMemory<byte> TRUE = new byte[] { 1 };
        private static readonly ReadOnlyMemory<byte> FALSE = new byte[] { 0 };

        private readonly bool value;

        public override ReadOnlyMemory<byte> Memory => value ? TRUE : FALSE;
        public override int Size => sizeof(bool);
        public override StackItemType Type => StackItemType.Boolean;

        /// <summary>
        /// Create a new VM object representing the boolean type.
        /// </summary>
        /// <param name="value">The initial value of the object.</param>
        internal Boolean(bool value)
        {
            this.value = value;
        }

        public override bool Equals(StackItem? other)
        {
            if (ReferenceEquals(this, other)) return true;
            if (other is Boolean b) return value == b.value;
            return false;
        }

        public override bool GetBoolean()
        {
            return value;
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(value);
        }

        public override BigInteger GetInteger()
        {
            return value ? BigInteger.One : BigInteger.Zero;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static implicit operator Boolean(bool value)
        {
            return value ? True : False;
        }

        public override string ToString()
        {
            return value.ToString();
        }
    }
}
