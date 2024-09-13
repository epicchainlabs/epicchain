// Copyright (C) 2021-2024 EpicChain Labs.

//
// ByteArrayComparer.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Runtime.CompilerServices;

namespace EpicChain.IO
{
    internal class ByteArrayComparer : IComparer<byte[]>
    {
        public static readonly ByteArrayComparer Default = new(1);
        public static readonly ByteArrayComparer Reverse = new(-1);

        private readonly int _direction;

        private ByteArrayComparer(int direction)
        {
            _direction = direction;
        }

        public int Compare(byte[]? x, byte[]? y)
        {
            if (x == y) return 0;
            if (x is null && y is not null)
                return _direction > 0 ? -y.Length : y.Length;
            if (y is null && x is not null)
                return _direction > 0 ? x.Length : -x.Length;
            return _direction > 0 ?
                    CompareInternal(x!, y!) :
                    -CompareInternal(x!, y!);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        private static int CompareInternal(byte[] x, byte[] y)
        {
            var length = Math.Min(x.Length, y.Length);
            for (var i = 0; i < length; i++)
            {
                var r = x[i].CompareTo(y[i]);
                if (r != 0) return r;
            }
            return x.Length.CompareTo(y.Length);
        }
    }
}
