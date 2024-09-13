// Copyright (C) 2021-2024 EpicChain Labs.

//
// RandomExtensions.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Numerics;

namespace EpicChain.Extensions
{
    public static class RandomExtensions
    {
        public static BigInteger NextBigInteger(this Random rand, int sizeInBits)
        {
            if (sizeInBits < 0)
                throw new ArgumentException("sizeInBits must be non-negative");
            if (sizeInBits == 0)
                return 0;
            Span<byte> b = stackalloc byte[sizeInBits / 8 + 1];
            rand.NextBytes(b);
            if (sizeInBits % 8 == 0)
                b[^1] = 0;
            else
                b[^1] &= (byte)((1 << sizeInBits % 8) - 1);
            return new BigInteger(b);
        }
    }
}
