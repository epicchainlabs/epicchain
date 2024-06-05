// Copyright (C) 2021-2024 The EpicChain Labs.
//
// SnapshotCache.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
using Neo.SmartContract;
using System;
using System.Collections.Generic;
using System.Linq;

namespace Neo.Persistence
{
    /// <summary>
    /// Represents a cache for the snapshot or database of the NEO blockchain.
    /// </summary>
    public class SnapshotCache : DataCache, IDisposable
    {
        private readonly IReadOnlyStore store;
        private readonly ISnapshot snapshot;

        /// <summary>
        /// Initializes a new instance of the <see cref="SnapshotCache"/> class.
        /// </summary>
        /// <param name="store">An <see cref="IReadOnlyStore"/> to create a readonly cache; or an <see cref="ISnapshot"/> to create a snapshot cache.</param>
        public SnapshotCache(IReadOnlyStore store)
        {
            this.store = store;
            snapshot = store as ISnapshot;
        }

        protected override void AddInternal(StorageKey key, StorageItem value)
        {
            snapshot?.Put(key.ToArray(), value.ToArray());
        }

        protected override void DeleteInternal(StorageKey key)
        {
            snapshot?.Delete(key.ToArray());
        }

        public override void Commit()
        {
            base.Commit();
            snapshot?.Commit();
        }

        protected override bool ContainsInternal(StorageKey key)
        {
            return store.Contains(key.ToArray());
        }

        public void Dispose()
        {
            snapshot?.Dispose();
        }

        protected override StorageItem GetInternal(StorageKey key)
        {
            byte[] value = store.TryGet(key.ToArray());
            if (value == null) throw new KeyNotFoundException();
            return new(value);
        }

        protected override IEnumerable<(StorageKey, StorageItem)> SeekInternal(byte[] keyOrPrefix, SeekDirection direction)
        {
            return store.Seek(keyOrPrefix, direction).Select(p => (new StorageKey(p.Key), new StorageItem(p.Value)));
        }

        protected override StorageItem TryGetInternal(StorageKey key)
        {
            byte[] value = store.TryGet(key.ToArray());
            if (value == null) return null;
            return new(value);
        }

        protected override void UpdateInternal(StorageKey key, StorageItem value)
        {
            snapshot?.Put(key.ToArray(), value.ToArray());
        }
    }
}
