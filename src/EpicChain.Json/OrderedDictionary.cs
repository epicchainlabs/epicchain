// Copyright (C) 2021-2024 EpicChain Labs.

//
// OrderedDictionary.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System.Collections;
using System.Collections.ObjectModel;
using System.Diagnostics.CodeAnalysis;

namespace EpicChain.Json
{
    partial class OrderedDictionary<TKey, TValue> : IDictionary<TKey, TValue> where TKey : notnull
    {
        private class TItem
        {
            public TKey Key;
            public TValue Value;

            public TItem(TKey key, TValue value)
            {
                Key = key;
                Value = value;
            }
        }

        private class InternalCollection : KeyedCollection<TKey, TItem>
        {
            protected override TKey GetKeyForItem(TItem item)
            {
                return item.Key;
            }
        }

        private readonly InternalCollection collection = new();

        public int Count => collection.Count;
        public bool IsReadOnly => false;
        public IReadOnlyList<TKey> Keys { get; }
        public IReadOnlyList<TValue> Values { get; }
        ICollection<TKey> IDictionary<TKey, TValue>.Keys => (KeyCollection)Keys;
        ICollection<TValue> IDictionary<TKey, TValue>.Values => (ValueCollection)Values;

        public OrderedDictionary()
        {
            Keys = new KeyCollection(collection);
            Values = new ValueCollection(collection);
        }

        public TValue this[TKey key]
        {
            get
            {
                return collection[key].Value;
            }
            set
            {
                if (collection.TryGetValue(key, out var entry))
                    entry.Value = value;
                else
                    Add(key, value);
            }
        }

        public TValue this[int index]
        {
            get
            {
                return collection[index].Value;
            }
        }

        public void Add(TKey key, TValue value)
        {
            collection.Add(new TItem(key, value));
        }

        public bool ContainsKey(TKey key)
        {
            return collection.Contains(key);
        }

        public bool Remove(TKey key)
        {
            return collection.Remove(key);
        }

#pragma warning disable CS8767

        public bool TryGetValue(TKey key, [MaybeNullWhen(false)] out TValue value)
        {
            if (collection.TryGetValue(key, out var entry))
            {
                value = entry.Value;
                return true;
            }
            value = default;
            return false;
        }

#pragma warning restore CS8767

        void ICollection<KeyValuePair<TKey, TValue>>.Add(KeyValuePair<TKey, TValue> item)
        {
            Add(item.Key, item.Value);
        }

        public void Clear()
        {
            collection.Clear();
        }

        bool ICollection<KeyValuePair<TKey, TValue>>.Contains(KeyValuePair<TKey, TValue> item)
        {
            return collection.Contains(item.Key);
        }

        void ICollection<KeyValuePair<TKey, TValue>>.CopyTo(KeyValuePair<TKey, TValue>[] array, int arrayIndex)
        {
            for (int i = 0; i < collection.Count; i++)
                array[i + arrayIndex] = new KeyValuePair<TKey, TValue>(collection[i].Key, collection[i].Value);
        }

        bool ICollection<KeyValuePair<TKey, TValue>>.Remove(KeyValuePair<TKey, TValue> item)
        {
            return collection.Remove(item.Key);
        }

        IEnumerator<KeyValuePair<TKey, TValue>> IEnumerable<KeyValuePair<TKey, TValue>>.GetEnumerator()
        {
            return collection.Select(p => new KeyValuePair<TKey, TValue>(p.Key, p.Value)).GetEnumerator();
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            return collection.Select(p => new KeyValuePair<TKey, TValue>(p.Key, p.Value)).GetEnumerator();
        }
    }
}
