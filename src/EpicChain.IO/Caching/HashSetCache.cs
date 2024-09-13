// Copyright (C) 2021-2024 EpicChain Labs.

//
// HashSetCache.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

namespace EpicChain.IO.Caching
{
    internal class HashSetCache<T> : IReadOnlyCollection<T> where T : IEquatable<T>
    {
        /// <summary>
        /// Sets where the Hashes are stored
        /// </summary>
        private readonly LinkedList<HashSet<T>> _sets = new();

        /// <summary>
        /// Maximum capacity of each bucket inside each HashSet of <see cref="_sets"/>.
        /// </summary>
        private readonly int _bucketCapacity;

        /// <summary>
        /// Maximum number of buckets for the LinkedList, meaning its maximum cardinality.
        /// </summary>
        private readonly int _maxBucketCount;

        /// <summary>
        /// Entry count
        /// </summary>
        public int Count { get; private set; }

        public HashSetCache(int bucketCapacity, int maxBucketCount = 10)
        {
            if (bucketCapacity <= 0) throw new ArgumentOutOfRangeException($"{nameof(bucketCapacity)} should be greater than 0");
            if (maxBucketCount <= 0) throw new ArgumentOutOfRangeException($"{nameof(maxBucketCount)} should be greater than 0");

            Count = 0;
            _bucketCapacity = bucketCapacity;
            _maxBucketCount = maxBucketCount;
            _sets.AddFirst([]);
        }

        public bool Add(T item)
        {
            if (Contains(item)) return false;
            Count++;
            if (_sets.First?.Value.Count < _bucketCapacity) return _sets.First.Value.Add(item);
            var newSet = new HashSet<T>
            {
                item
            };
            _sets.AddFirst(newSet);
            if (_sets.Count > _maxBucketCount)
            {
                Count -= _sets.Last?.Value.Count ?? 0;
                _sets.RemoveLast();
            }
            return true;
        }

        public bool Contains(T item)
        {
            foreach (var set in _sets)
            {
                if (set.Contains(item)) return true;
            }
            return false;
        }

        public void ExceptWith(IEnumerable<T> items)
        {
            List<HashSet<T>> removeList = default!;
            foreach (var item in items)
            {
                foreach (var set in _sets)
                {
                    if (set.Remove(item))
                    {
                        Count--;
                        if (set.Count == 0)
                        {
                            removeList ??= [];
                            removeList.Add(set);
                        }
                        break;
                    }
                }
            }
            if (removeList == null) return;
            foreach (var set in removeList)
            {
                _sets.Remove(set);
            }
        }

        public IEnumerator<T> GetEnumerator()
        {
            foreach (var set in _sets)
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
