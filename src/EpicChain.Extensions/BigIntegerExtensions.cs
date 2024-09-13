// Copyright (C) 2021-2024 EpicChain Labs.

//
// BigIntegerExtensions.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Collections.Generic;
using System.Numerics;
using System.Runtime.CompilerServices;

namespace EpicChain.Extensions
{
    public static class BigIntegerExtensions
    {
        public static int GetLowestSetBit(this BigInteger i)
        {
            if (i.Sign == 0)
                return -1;
            var b = i.ToByteArray();
            var w = 0;
            while (b[w] == 0)
                w++;
            for (var x = 0; x < 8; x++)
                if ((b[w] & 1 << x) > 0)
                    return x + w * 8;
            throw new Exception();
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static BigInteger Mod(this BigInteger x, BigInteger y)
        {
            x %= y;
            if (x.Sign < 0)
                x += y;
            return x;
        }

        public static BigInteger ModInverse(this BigInteger a, BigInteger n)
        {
            if (BigInteger.GreatestCommonDivisor(a, n) != 1)
            {
                throw new ArithmeticException("No modular inverse exists for the given inputs.");
            }

            BigInteger i = n, v = 0, d = 1;
            while (a > 0)
            {
                BigInteger t = i / a, x = a;
                a = i % x;
                i = x;
                x = d;
                d = v - t * x;
                v = x;
            }
            v %= n;
            if (v < 0) v = (v + n) % n;
            return v;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static bool TestBit(this BigInteger i, int index)
        {
            return (i & (BigInteger.One << index)) > BigInteger.Zero;
        }

        /// <summary>
        /// Finds the sum of the specified integers.
        /// </summary>
        /// <param name="source">The specified integers.</param>
        /// <returns>The sum of the integers.</returns>
        public static BigInteger Sum(this IEnumerable<BigInteger> source)
        {
            var sum = BigInteger.Zero;
            foreach (var bi in source) sum += bi;
            return sum;
        }

        /// <summary>
        /// Converts a <see cref="BigInteger"/> to byte array and eliminates all the leading zeros.
        /// </summary>
        /// <param name="i">The <see cref="BigInteger"/> to convert.</param>
        /// <returns>The converted byte array.</returns>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static byte[] ToByteArrayStandard(this BigInteger i)
        {
            if (i.IsZero) return Array.Empty<byte>();
            return i.ToByteArray();
        }
    }
}
