// Copyright (C) 2021-2024 EpicChain Labs.

//
// OldUInt160.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Extensions;
using EpicChain.IO;
using System.Globalization;
using System.Runtime.InteropServices;

namespace Neo
{
    /// <summary>
    /// Represents a 160-bit unsigned integer.
    /// </summary>
    [StructLayout(LayoutKind.Explicit, Size = 20)]
    public class OldUInt160 : IComparable<OldUInt160>, IEquatable<OldUInt160>, ISerializable
    {
        /// <summary>
        /// The length of <see cref="OldUInt160"/> values.
        /// </summary>
        public const int Length = 20;

        /// <summary>
        /// Represents 0.
        /// </summary>
        public static readonly OldUInt160 Zero = new();

        [FieldOffset(0)] private ulong value1;
        [FieldOffset(8)] private ulong value2;
        [FieldOffset(16)] private uint value3;

        public int Size => Length;

        /// <summary>
        /// Initializes a new instance of the <see cref="OldUInt160"/> class.
        /// </summary>
        public OldUInt160()
        {
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="OldUInt160"/> class.
        /// </summary>
        /// <param name="value">The value of the <see cref="OldUInt160"/>.</param>
        public unsafe OldUInt160(ReadOnlySpan<byte> value)
        {
            if (value.Length != Length) throw new FormatException();
            fixed (ulong* p = &value1)
            {
                Span<byte> dst = new(p, Length);
                value[..Length].CopyTo(dst);
            }
        }

        public int CompareTo(OldUInt160 other)
        {
            int result = value3.CompareTo(other.value3);
            if (result != 0) return result;
            result = value2.CompareTo(other.value2);
            if (result != 0) return result;
            return value1.CompareTo(other.value1);
        }

        public void Deserialize(ref MemoryReader reader)
        {
            value1 = reader.ReadUInt64();
            value2 = reader.ReadUInt64();
            value3 = reader.ReadUInt32();
        }

        public override bool Equals(object obj)
        {
            if (ReferenceEquals(obj, this)) return true;
            return Equals(obj as OldUInt160);
        }

        public bool Equals(OldUInt160 other)
        {
            if (other is null) return false;
            return value1 == other.value1
                && value2 == other.value2
                && value3 == other.value3;
        }

        public override int GetHashCode()
        {
            return (int)value1;
        }

        /// <summary>
        /// Parses an <see cref="OldUInt160"/> from the specified <see cref="string"/>.
        /// </summary>
        /// <param name="value">An <see cref="OldUInt160"/> represented by a <see cref="string"/>.</param>
        /// <returns>The parsed <see cref="OldUInt160"/>.</returns>
        /// <exception cref="FormatException"><paramref name="value"/> is not in the correct format.</exception>
        public static OldUInt160 Parse(string value)
        {
            if (!TryParse(value, out var result)) throw new FormatException();
            return result;
        }

        public void Serialize(BinaryWriter writer)
        {
            writer.Write(value1);
            writer.Write(value2);
            writer.Write(value3);
        }

        public override string ToString()
        {
            return "0x" + this.ToArray().ToHexString(reverse: true);
        }

        /// <summary>
        /// Parses an <see cref="OldUInt160"/> from the specified <see cref="string"/>.
        /// </summary>
        /// <param name="s">An <see cref="OldUInt160"/> represented by a <see cref="string"/>.</param>
        /// <param name="result">The parsed <see cref="OldUInt160"/>.</param>
        /// <returns><see langword="true"/> if an <see cref="OldUInt160"/> is successfully parsed; otherwise, <see langword="false"/>.</returns>
        public static bool TryParse(string s, out OldUInt160 result)
        {
            if (s == null)
            {
                result = null;
                return false;
            }
            if (s.StartsWith("0x", StringComparison.InvariantCultureIgnoreCase))
                s = s[2..];
            if (s.Length != Length * 2)
            {
                result = null;
                return false;
            }
            byte[] data = new byte[Length];
            for (int i = 0; i < Length; i++)
                if (!byte.TryParse(s.Substring(i * 2, 2), NumberStyles.AllowHexSpecifier, null, out data[Length - i - 1]))
                {
                    result = null;
                    return false;
                }
            result = new OldUInt160(data);
            return true;
        }

        public static bool operator ==(OldUInt160 left, OldUInt160 right)
        {
            if (ReferenceEquals(left, right)) return true;
            if (left is null || right is null) return false;
            return left.Equals(right);
        }

        public static bool operator !=(OldUInt160 left, OldUInt160 right)
        {
            return !(left == right);
        }

        public static bool operator >(OldUInt160 left, OldUInt160 right)
        {
            return left.CompareTo(right) > 0;
        }

        public static bool operator >=(OldUInt160 left, OldUInt160 right)
        {
            return left.CompareTo(right) >= 0;
        }

        public static bool operator <(OldUInt160 left, OldUInt160 right)
        {
            return left.CompareTo(right) < 0;
        }

        public static bool operator <=(OldUInt160 left, OldUInt160 right)
        {
            return left.CompareTo(right) <= 0;
        }
    }
}
