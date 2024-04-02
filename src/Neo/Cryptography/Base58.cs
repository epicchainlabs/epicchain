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
// Base58.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

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
