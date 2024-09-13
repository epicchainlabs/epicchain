// Copyright (C) 2021-2024 EpicChain Labs.

//
// InteroperableList.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.VM;
using EpicChain.VM.Types;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace EpicChain.SmartContract.Native
{
    abstract class InteroperableList<T> : IList<T>, IInteroperable
    {
        private List<T> list;
        private List<T> List => list ??= new();

        public T this[int index] { get => List[index]; set => List[index] = value; }
        public int Count => List.Count;
        public bool IsReadOnly => false;

        public void Add(T item) => List.Add(item);
        public void AddRange(IEnumerable<T> collection) => List.AddRange(collection);
        public void Clear() => List.Clear();
        public bool Contains(T item) => List.Contains(item);
        public void CopyTo(T[] array, int arrayIndex) => List.CopyTo(array, arrayIndex);
        IEnumerator IEnumerable.GetEnumerator() => List.GetEnumerator();
        public IEnumerator<T> GetEnumerator() => List.GetEnumerator();
        public int IndexOf(T item) => List.IndexOf(item);
        public void Insert(int index, T item) => List.Insert(index, item);
        public bool Remove(T item) => List.Remove(item);
        public void RemoveAt(int index) => List.RemoveAt(index);
        public void Sort() => List.Sort();

        protected abstract T ElementFromStackItem(StackItem item);
        protected abstract StackItem ElementToStackItem(T element, ReferenceCounter referenceCounter);

        public void FromStackItem(StackItem stackItem)
        {
            List.Clear();
            foreach (StackItem item in (Array)stackItem)
            {
                Add(ElementFromStackItem(item));
            }
        }

        public StackItem ToStackItem(ReferenceCounter referenceCounter)
        {
            return new Array(referenceCounter, this.Select(p => ElementToStackItem(p, referenceCounter)));
        }
    }
}
