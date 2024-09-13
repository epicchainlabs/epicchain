// Copyright (C) 2021-2024 EpicChain Labs.

//
// Cache.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Threading;

namespace EpicChain.IO.Caching
{
    internal abstract class Cache<TKey, TValue>
        (int max_capacity, IEqualityComparer<TKey>? comparer = null) : ICollection<TValue>, IDisposable
        where TKey : notnull
    {
        protected class CacheItem
            (TKey key, TValue value)
        {
            public readonly TKey Key = key;
            public readonly TValue Value = value;
            public readonly DateTime Time = DateTime.UtcNow;
        }

        protected readonly ReaderWriterLockSlim RwSyncRootLock = new(LockRecursionPolicy.SupportsRecursion);
        protected readonly Dictionary<TKey, CacheItem> InnerDictionary = new Dictionary<TKey, CacheItem>(comparer);
        private readonly int _max_capacity = max_capacity;

        public TValue this[TKey key]
        {
            get
            {
                RwSyncRootLock.EnterReadLock();
                try
                {
                    if (!InnerDictionary.TryGetValue(key, out CacheItem? item)) throw new KeyNotFoundException();
                    OnAccess(item);
                    return item.Value;
                }
                finally
                {
                    RwSyncRootLock.ExitReadLock();
                }
            }
        }

        public int Count
        {
            get
            {
                RwSyncRootLock.EnterReadLock();
                try
                {
                    return InnerDictionary.Count;
                }
                finally
                {
                    RwSyncRootLock.ExitReadLock();
                }
            }
        }

        public bool IsReadOnly => false;

        public void Add(TValue item)
        {
            var key = GetKeyForItem(item);
            RwSyncRootLock.EnterWriteLock();
            try
            {
                AddInternal(key, item);
            }
            finally
            {
                RwSyncRootLock.ExitWriteLock();
            }
        }

        private void AddInternal(TKey key, TValue item)
        {
            if (InnerDictionary.TryGetValue(key, out CacheItem? cacheItem))
            {
                OnAccess(cacheItem);
            }
            else
            {
                if (InnerDictionary.Count >= _max_capacity)
                {
                    //TODO: Perform a performance test on the PLINQ query to determine which algorithm is better here (parallel or not)
                    foreach (var item_del in InnerDictionary.Values.AsParallel().OrderBy(p => p.Time).Take(InnerDictionary.Count - _max_capacity + 1))
                    {
                        RemoveInternal(item_del);
                    }
                }
                InnerDictionary.Add(key, new CacheItem(key, item));
            }
        }

        public void AddRange(IEnumerable<TValue> items)
        {
            RwSyncRootLock.EnterWriteLock();
            try
            {
                foreach (var item in items)
                {
                    var key = GetKeyForItem(item);
                    AddInternal(key, item);
                }
            }
            finally
            {
                RwSyncRootLock.ExitWriteLock();
            }
        }

        public void Clear()
        {
            RwSyncRootLock.EnterWriteLock();
            try
            {
                foreach (var item_del in InnerDictionary.Values.ToArray())
                {
                    RemoveInternal(item_del);
                }
            }
            finally
            {
                RwSyncRootLock.ExitWriteLock();
            }
        }

        public bool Contains(TKey key)
        {
            RwSyncRootLock.EnterReadLock();
            try
            {
                if (!InnerDictionary.TryGetValue(key, out CacheItem? cacheItem)) return false;
                OnAccess(cacheItem);
                return true;
            }
            finally
            {
                RwSyncRootLock.ExitReadLock();
            }
        }

        public bool Contains(TValue item)
        {
            return Contains(GetKeyForItem(item));
        }

        public void CopyTo(TValue[] array, int arrayIndex)
        {
            if (array == null) throw new ArgumentNullException();
            if (arrayIndex < 0) throw new ArgumentOutOfRangeException();
            if (arrayIndex + InnerDictionary.Count > array.Length) throw new ArgumentException();
            foreach (var item in this)
            {
                array[arrayIndex++] = item;
            }
        }

        public void Dispose()
        {
            Clear();
            RwSyncRootLock.Dispose();
        }

        public IEnumerator<TValue> GetEnumerator()
        {
            RwSyncRootLock.EnterReadLock();
            try
            {
                foreach (var item in InnerDictionary.Values.Select(p => p.Value))
                {
                    yield return item;
                }
            }
            finally
            {
                RwSyncRootLock.ExitReadLock();
            }
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            return GetEnumerator();
        }

        protected abstract TKey GetKeyForItem(TValue item);

        public bool Remove(TKey key)
        {
            RwSyncRootLock.EnterWriteLock();
            try
            {
                if (!InnerDictionary.TryGetValue(key, out CacheItem? cacheItem)) return false;
                RemoveInternal(cacheItem);
                return true;
            }
            finally
            {
                RwSyncRootLock.ExitWriteLock();
            }
        }

        protected abstract void OnAccess(CacheItem item);

        public bool Remove(TValue item)
        {
            return Remove(GetKeyForItem(item));
        }

        private void RemoveInternal(CacheItem item)
        {
            InnerDictionary.Remove(item.Key);
            if (item.Value is IDisposable disposable)
            {
                disposable.Dispose();
            }
        }

        public bool TryGet(TKey key, out TValue item)
        {
            RwSyncRootLock.EnterReadLock();
            try
            {
                if (InnerDictionary.TryGetValue(key, out CacheItem? cacheItem))
                {
                    OnAccess(cacheItem);
                    item = cacheItem.Value;
                    return true;
                }
            }
            finally
            {
                RwSyncRootLock.ExitReadLock();
            }
            item = default!;
            return false;
        }
    }
}
