// Copyright (C) 2021-2024 The EpicChain Labs.
//
// Extensions.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.IO;
using Neo.Persistence;
using Neo.VM.Types;
using System;
using System.Collections.Generic;
using System.Numerics;

namespace Neo.Plugins
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
