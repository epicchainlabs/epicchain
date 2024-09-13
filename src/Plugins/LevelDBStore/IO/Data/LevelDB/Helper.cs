// Copyright (C) 2021-2024 EpicChain Labs.

//
// Helper.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Persistence;
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;

namespace EpicChain.IO.Data.LevelDB
{
    public static class Helper
    {
        public static IEnumerable<T> Seek<T>(this DB db, ReadOptions options, byte[] prefix, SeekDirection direction, Func<byte[], byte[], T> resultSelector)
        {
            using Iterator it = db.NewIterator(options);
            if (direction == SeekDirection.Forward)
            {
                for (it.Seek(prefix); it.Valid(); it.Next())
                    yield return resultSelector(it.Key(), it.Value());
            }
            else
            {
                // SeekForPrev

                it.Seek(prefix);
                if (!it.Valid())
                    it.SeekToLast();
                else if (it.Key().AsSpan().SequenceCompareTo(prefix) > 0)
                    it.Prev();

                for (; it.Valid(); it.Prev())
                    yield return resultSelector(it.Key(), it.Value());
            }
        }

        public static IEnumerable<T> FindRange<T>(this DB db, ReadOptions options, byte[] startKey, byte[] endKey, Func<byte[], byte[], T> resultSelector)
        {
            using Iterator it = db.NewIterator(options);
            for (it.Seek(startKey); it.Valid(); it.Next())
            {
                byte[] key = it.Key();
                if (key.AsSpan().SequenceCompareTo(endKey) > 0) break;
                yield return resultSelector(key, it.Value());
            }
        }

        internal static byte[] ToByteArray(this IntPtr data, UIntPtr length)
        {
            if (data == IntPtr.Zero) return null;
            byte[] buffer = new byte[(int)length];
            Marshal.Copy(data, buffer, 0, (int)length);
            return buffer;
        }
    }
}
