// Copyright (C) 2021-2024 EpicChain Labs.

//
// ByteExtensions.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Text;

namespace EpicChain.Extensions
{
    public static class ByteExtensions
    {
        /// <summary>
        /// Converts a byte array to hex <see cref="string"/>.
        /// </summary>
        /// <param name="value">The byte array to convert.</param>
        /// <returns>The converted hex <see cref="string"/>.</returns>
        public static string ToHexString(this byte[] value)
        {
            StringBuilder sb = new();
            foreach (var b in value)
                sb.AppendFormat("{0:x2}", b);
            return sb.ToString();
        }

        /// <summary>
        /// Converts a byte array to hex <see cref="string"/>.
        /// </summary>
        /// <param name="value">The byte array to convert.</param>
        /// <param name="reverse">Indicates whether it should be converted in the reversed byte order.</param>
        /// <returns>The converted hex <see cref="string"/>.</returns>
        public static string ToHexString(this byte[] value, bool reverse = false)
        {
            StringBuilder sb = new();
            for (var i = 0; i < value.Length; i++)
                sb.AppendFormat("{0:x2}", value[reverse ? value.Length - i - 1 : i]);
            return sb.ToString();
        }

        /// <summary>
        /// Converts a byte array to hex <see cref="string"/>.
        /// </summary>
        /// <param name="value">The byte array to convert.</param>
        /// <returns>The converted hex <see cref="string"/>.</returns>
        public static string ToHexString(this ReadOnlySpan<byte> value)
        {
            StringBuilder sb = new();
            foreach (var b in value)
                sb.AppendFormat("{0:x2}", b);
            return sb.ToString();
        }
    }
}
