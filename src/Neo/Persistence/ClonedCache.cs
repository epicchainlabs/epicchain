// Copyright (C) 2021-2024 The EpicChain Labs.
//
// ClonedCache.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.SmartContract;
using System.Collections.Generic;

namespace Neo.Persistence
{
    class ClonedCache : DataCache
    {
        private readonly DataCache innerCache;

        public ClonedCache(DataCache innerCache)
        {
            this.innerCache = innerCache;
        }

        protected override void AddInternal(StorageKey key, StorageItem value)
        {
            innerCache.Add(key, value.Clone());
        }

        protected override void DeleteInternal(StorageKey key)
        {
            innerCache.Delete(key);
        }

        protected override bool ContainsInternal(StorageKey key)
        {
            return innerCache.Contains(key);
        }

        protected override StorageItem GetInternal(StorageKey key)
        {
            return innerCache[key].Clone();
        }

        protected override IEnumerable<(StorageKey, StorageItem)> SeekInternal(byte[] keyOrPreifx, SeekDirection direction)
        {
            foreach (var (key, value) in innerCache.Seek(keyOrPreifx, direction))
                yield return (key, value.Clone());
        }

        protected override StorageItem TryGetInternal(StorageKey key)
        {
            return innerCache.TryGet(key)?.Clone();
        }

        protected override void UpdateInternal(StorageKey key, StorageItem value)
        {
            innerCache.GetAndChange(key).FromReplica(value);
        }
    }
}
