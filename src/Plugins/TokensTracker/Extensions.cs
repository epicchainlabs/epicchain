// Copyright (C) 2021-2024 EpicChain Labs.

//
// Extensions.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.IO;
using EpicChain.Persistence;
using EpicChain.VM.Types;
using System;
using System.Collections.Generic;
using System.Numerics;

namespace EpicChain.Plugins
{
    public static class Extensions
    {
        public static bool NotNull(this StackItem item)
        {
            return !item.IsNull;
        }

        public static string ToBase64(this ReadOnlySpan<byte> item)
        {
            return item == null ? String.Empty : Convert.ToBase64String(item);
        }

        public static int GetVarSize(this ByteString item)
        {
            var length = item.GetSpan().Length;
            return IO.Helper.GetVarSize(length) + length;
        }

        public static int GetVarSize(this BigInteger item)
        {
            var length = item.GetByteCount();
            return IO.Helper.GetVarSize(length) + length;
        }

        public static IEnumerable<(TKey, TValue)> FindPrefix<TKey, TValue>(this IStore db, byte[] prefix)
            where TKey : ISerializable, new()
            where TValue : class, ISerializable, new()
        {
            foreach (var (key, value) in db.Seek(prefix, SeekDirection.Forward))
            {
                if (!key.AsSpan().StartsWith(prefix)) break;
                yield return (key.AsSerializable<TKey>(1), value.AsSerializable<TValue>());
            }
        }

        public static IEnumerable<(TKey, TValue)> FindRange<TKey, TValue>(this IStore db, byte[] startKey, byte[] endKey)
            where TKey : ISerializable, new()
            where TValue : class, ISerializable, new()
        {
            foreach (var (key, value) in db.Seek(startKey, SeekDirection.Forward))
            {
                if (key.AsSpan().SequenceCompareTo(endKey) > 0) break;
                yield return (key.AsSerializable<TKey>(1), value.AsSerializable<TValue>());
            }
        }
    }
}
