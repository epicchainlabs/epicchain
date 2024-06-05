// Copyright (C) 2021-2024 The EpicChain Labs.
//
// Base58.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using System;
using System.Linq;
using System.Numerics;
using System.Text;
using static Neo.Helper;

namespace Neo.Cryptography
{
    /// <summary>
    /// A helper class for base-58 encoder.
    /// </summary>
    public static class Base58
    {
        /// <summary>
        /// Represents the alphabet of the base-58 encoder.
        /// </summary>
        public const string Alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

        /// <summary>
        /// Converts the specified <see cref="string"/>, which encodes binary data as base-58 digits, to an equivalent byte array. The encoded <see cref="string"/> contains the checksum of the binary data.
        /// </summary>
        /// <param name="input">The <see cref="string"/> to convert.</param>
        /// <returns>A byte array that is equivalent to <paramref name="input"/>.</returns>
        public static byte[] Base58CheckDecode(this string input)
        {
            if (input is null) throw new ArgumentNullException(nameof(input));
            byte[] buffer = Decode(input);
            if (buffer.Length < 4) throw new FormatException();
            byte[] checksum = buffer.Sha256(0, buffer.Length - 4).Sha256();
            if (!buffer.AsSpan(^4).SequenceEqual(checksum.AsSpan(..4)))
                throw new FormatException();
            var ret = buffer[..^4];
            Array.Clear(buffer, 0, buffer.Length);
            return ret;
        }

        /// <summary>
        /// Converts a byte array to its equivalent <see cref="string"/>
        /// representation that is encoded with base-58 digits.
        /// The encoded <see cref="string"/> contains the checksum of the binary data.
        /// </summary>
        /// <param name="data">The byte array to convert.</param>
        /// <returns>The <see cref="string"/> representation, in base-58, of the contents of <paramref name="data"/>.</returns>
        public static string Base58CheckEncode(this ReadOnlySpan<byte> data)
        {
            byte[] checksum = data.Sha256().Sha256();
            Span<byte> buffer = stackalloc byte[data.Length + 4];
            data.CopyTo(buffer);
            checksum.AsSpan(..4).CopyTo(buffer[data.Length..]);
            var ret = Encode(buffer);
            buffer.Clear();
            return ret;
        }

        /// <summary>
        /// Converts the specified <see cref="string"/>, which encodes binary data as base-58 digits, to an equivalent byte array.
        /// </summary>
        /// <param name="input">The <see cref="string"/> to convert.</param>
        /// <returns>A byte array that is equivalent to <paramref name="input"/>.</returns>
        public static byte[] Decode(string input)
        {
            // Decode Base58 string to BigInteger
            var bi = BigInteger.Zero;
            for (int i = 0; i < input.Length; i++)
            {
                int digit = Alphabet.IndexOf(input[i]);
                if (digit < 0)
                    throw new FormatException($"Invalid Base58 character '{input[i]}' at position {i}");
                bi = bi * Alphabet.Length + digit;
            }

            // Encode BigInteger to byte[]
            // Leading zero bytes get encoded as leading `1` characters
            int leadingZeroCount = input.TakeWhile(c => c == Alphabet[0]).Count();
            var leadingZeros = new byte[leadingZeroCount];
            if (bi.IsZero) return leadingZeros;
            var bytesWithoutLeadingZeros = bi.ToByteArray(isUnsigned: true, isBigEndian: true);
            return Concat(leadingZeros, bytesWithoutLeadingZeros);
        }

        /// <summary>
        /// Converts a byte array to its equivalent <see cref="string"/> representation that is encoded with base-58 digits.
        /// </summary>
        /// <param name="input">The byte array to convert.</param>
        /// <returns>The <see cref="string"/> representation, in base-58, of the contents of <paramref name="input"/>.</returns>
        public static string Encode(ReadOnlySpan<byte> input)
        {
            // Decode byte[] to BigInteger
            BigInteger value = new(input, isUnsigned: true, isBigEndian: true);

            // Encode BigInteger to Base58 string
            var sb = new StringBuilder();

            while (value > 0)
            {
                value = BigInteger.DivRem(value, Alphabet.Length, out var remainder);
                sb.Insert(0, Alphabet[(int)remainder]);
            }

            // Append `1` for each leading 0 byte
            for (int i = 0; i < input.Length && input[i] == 0; i++)
            {
                sb.Insert(0, Alphabet[0]);
            }
            return sb.ToString();
        }
    }
}
