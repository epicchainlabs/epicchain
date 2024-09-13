// Copyright (C) 2021-2024 EpicChain Labs.

//
// OrderedDictionary.KeyCollection.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

namespace EpicChain.Json;

partial class OrderedDictionary<TKey, TValue>
{
    class KeyCollection : ICollection<TKey>, IReadOnlyList<TKey>
    {
        private readonly InternalCollection internalCollection;

        public KeyCollection(InternalCollection internalCollection)
        {
            this.internalCollection = internalCollection;
        }

        public TKey this[int index] => internalCollection[index].Key;

        public int Count => internalCollection.Count;

        public bool IsReadOnly => true;

        public void Add(TKey item) => throw new NotSupportedException();

        public void Clear() => throw new NotSupportedException();

        public bool Contains(TKey item) => internalCollection.Contains(item);

        public void CopyTo(TKey[] array, int arrayIndex)
        {
            for (int i = 0; i < internalCollection.Count && i + arrayIndex < array.Length; i++)
                array[i + arrayIndex] = internalCollection[i].Key;
        }

        public IEnumerator<TKey> GetEnumerator() => internalCollection.Select(p => p.Key).GetEnumerator();

        IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();

        public bool Remove(TKey item) => throw new NotSupportedException();
    }
}
