// Copyright (C) 2021-2024 EpicChain Labs.

//
// ByteArrayEqualityComparer.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System.Collections.Generic;

namespace EpicChain.IO
{
    internal class ByteArrayEqualityComparer : IEqualityComparer<byte[]>
    {
        public static readonly ByteArrayEqualityComparer Default = new();

        public unsafe bool Equals(byte[]? x, byte[]? y)
        {
            if (ReferenceEquals(x, y)) return true;
            if (x is null || y is null) return false;
            var len = x.Length;
            if (len != y.Length) return false;
            if (len == 0) return true;
            fixed (byte* xp = x, yp = y)
            {
                long* xlp = (long*)xp, ylp = (long*)yp;
                for (; len >= 8; len -= 8)
                {
                    if (*xlp != *ylp) return false;
                    xlp++;
                    ylp++;
                }
                byte* xbp = (byte*)xlp, ybp = (byte*)ylp;
                for (; len > 0; len--)
                {
                    if (*xbp != *ybp) return false;
                    xbp++;
                    ybp++;
                }
            }
            return true;
        }

        public int GetHashCode(byte[] obj)
        {
            unchecked
            {
                var hash = 17;
                foreach (var element in obj)
                    hash = hash * 31 + element;
                return hash;
            }
        }
    }
}
