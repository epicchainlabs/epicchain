// Copyright (C) 2021-2024 The EpicChain Labs.
//
// HashSetCache.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using System;
using System.Collections;
using System.Collections.Generic;

namespace Neo.IO.Caching
{
    class HashSetCache<T> : IReadOnlyCollection<T> where T : IEquatable<T>
    {
        /// <summary>
        /// Sets where the Hashes are stored
        /// </summary>
        private readonly LinkedList<HashSet<T>> sets = new();

        /// <summary>
        /// Maximum capacity of each bucket inside each HashSet of <see cref="sets"/>.
        /// </summary>
        private readonly int bucketCapacity;

        /// <summary>
        /// Maximum number of buckets for the LinkedList, meaning its maximum cardinality.
        /// </summary>
        private readonly int maxBucketCount;

        /// <summary>
        /// Entry count
        /// </summary>
        public int Count { get; private set; }

        public HashSetCache(int bucketCapacity, int maxBucketCount = 10)
        {
            if (bucketCapacity <= 0) throw new ArgumentOutOfRangeException($"{nameof(bucketCapacity)} should be greater than 0");
            if (maxBucketCount <= 0) throw new ArgumentOutOfRangeException($"{nameof(maxBucketCount)} should be greater than 0");

            Count = 0;
            this.bucketCapacity = bucketCapacity;
            this.maxBucketCount = maxBucketCount;
            sets.AddFirst(new HashSet<T>());
        }

        public bool Add(T item)
        {
            if (Contains(item)) return false;
            Count++;
            if (sets.First.Value.Count < bucketCapacity) return sets.First.Value.Add(item);
            var newSet = new HashSet<T>
            {
                item
            };
            sets.AddFirst(newSet);
            if (sets.Count > maxBucketCount)
            {
                Count -= sets.Last.Value.Count;
                sets.RemoveLast();
            }
            return true;
        }

        public bool Contains(T item)
        {
            foreach (var set in sets)
            {
                if (set.Contains(item)) return true;
            }
            return false;
        }

        public void ExceptWith(IEnumerable<T> items)
        {
            List<HashSet<T>> removeList = null;
            foreach (var item in items)
            {
                foreach (var set in sets)
                {
                    if (set.Remove(item))
                    {
                        Count--;
                        if (set.Count == 0)
                        {
                            removeList ??= new List<HashSet<T>>();
                            removeList.Add(set);
                        }
                        break;
                    }
                }
            }
            if (removeList == null) return;
            foreach (var set in removeList)
            {
                sets.Remove(set);
            }
        }

        public IEnumerator<T> GetEnumerator()
        {
            foreach (var set in sets)
            {
                foreach (var item in set)
                {
                    yield return item;
                }
            }
        }

        IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();
    }
}
