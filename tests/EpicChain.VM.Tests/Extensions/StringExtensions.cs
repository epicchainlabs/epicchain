// Copyright (C) 2021-2024 EpicChain Labs.

//
// StringExtensions.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Globalization;

namespace EpicChain.Test.Extensions
{
    internal static class StringExtensions
    {
        /// <summary>
        /// Convert buffer to hex string
        /// </summary>
        /// <param name="data">Data</param>
        /// <returns>Return hex string</returns>
        public static string ToHexString(this byte[] data)
        {
            if (data == null) return "";

            var m = data.Length;
            if (m == 0) return "";

            var sb = new char[(m * 2) + 2];

            sb[0] = '0';
            sb[1] = 'x';

            for (int x = 0, y = 2; x < m; x++, y += 2)
            {
                var hex = data[x].ToString("x2");

                sb[y] = hex[0];
                sb[y + 1] = hex[1];
            }

            return new string(sb);
        }

        /// <summary>
        /// Convert string in Hex format to byte array
        /// </summary>
        /// <param name="value">Hexadecimal string</param>
        /// <returns>Return byte array</returns>
        public static byte[] FromHexString(this string value)
        {
            if (string.IsNullOrEmpty(value))
                return Array.Empty<byte>();
            if (value.StartsWith("0x"))
                value = value[2..];
            if (value.Length % 2 == 1)
                throw new FormatException();

            var result = new byte[value.Length / 2];
            for (var i = 0; i < result.Length; i++)
                result[i] = byte.Parse(value.Substring(i * 2, 2), NumberStyles.AllowHexSpecifier);

            return result;
        }
    }
}
