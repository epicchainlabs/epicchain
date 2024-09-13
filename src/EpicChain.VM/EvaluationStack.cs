// Copyright (C) 2021-2024 EpicChain Labs.

//
// EvaluationStack.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.VM.Types;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.CompilerServices;

namespace EpicChain.VM
{
    /// <summary>
    /// Represents the evaluation stack in the VM.
    /// </summary>
    public sealed class EvaluationStack : IReadOnlyList<StackItem>
    {
        private readonly List<StackItem> innerList = new();
        private readonly ReferenceCounter referenceCounter;

        internal ReferenceCounter ReferenceCounter => referenceCounter;

        internal EvaluationStack(ReferenceCounter referenceCounter)
        {
            this.referenceCounter = referenceCounter;
        }

        /// <summary>
        /// Gets the number of items on the stack.
        /// </summary>
        public int Count => innerList.Count;

        internal void Clear()
        {
            foreach (StackItem item in innerList)
                referenceCounter.RemoveStackReference(item);
            innerList.Clear();
        }

        internal void CopyTo(EvaluationStack stack, int count = -1)
        {
            if (count < -1 || count > innerList.Count)
                throw new ArgumentOutOfRangeException(nameof(count));
            if (count == 0) return;
            if (count == -1 || count == innerList.Count)
                stack.innerList.AddRange(innerList);
            else
                stack.innerList.AddRange(innerList.Skip(innerList.Count - count));
        }

        public IEnumerator<StackItem> GetEnumerator()
        {
            return innerList.GetEnumerator();
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            return innerList.GetEnumerator();
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        internal void Insert(int index, StackItem item)
        {
            if (index > innerList.Count) throw new InvalidOperationException($"Insert out of bounds: {index}/{innerList.Count}");
            innerList.Insert(innerList.Count - index, item);
            referenceCounter.AddStackReference(item);
        }

        internal void MoveTo(EvaluationStack stack, int count = -1)
        {
            if (count == 0) return;
            CopyTo(stack, count);
            if (count == -1 || count == innerList.Count)
                innerList.Clear();
            else
                innerList.RemoveRange(innerList.Count - count, count);
        }

        /// <summary>
        /// Returns the item at the specified index from the top of the stack without removing it.
        /// </summary>
        /// <param name="index">The index of the object from the top of the stack.</param>
        /// <returns>The item at the specified index.</returns>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public StackItem Peek(int index = 0)
        {
            if (index >= innerList.Count) throw new InvalidOperationException($"Peek out of bounds: {index}/{innerList.Count}");
            if (index < 0)
            {
                index += innerList.Count;
                if (index < 0) throw new InvalidOperationException($"Peek out of bounds: {index}/{innerList.Count}");
            }
            return innerList[innerList.Count - index - 1];
        }

        StackItem IReadOnlyList<StackItem>.this[int index] => Peek(index);

        /// <summary>
        /// Pushes an item onto the top of the stack.
        /// </summary>
        /// <param name="item">The item to be pushed.</param>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public void Push(StackItem item)
        {
            innerList.Add(item);
            referenceCounter.AddStackReference(item);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        internal void Reverse(int n)
        {
            if (n < 0 || n > innerList.Count)
                throw new ArgumentOutOfRangeException(nameof(n));
            if (n <= 1) return;
            innerList.Reverse(innerList.Count - n, n);
        }

        /// <summary>
        /// Removes and returns the item at the top of the stack.
        /// </summary>
        /// <returns>The item removed from the top of the stack.</returns>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public StackItem Pop()
        {
            return Remove<StackItem>(0);
        }

        /// <summary>
        /// Removes and returns the item at the top of the stack and convert it to the specified type.
        /// </summary>
        /// <typeparam name="T">The type to convert to.</typeparam>
        /// <returns>The item removed from the top of the stack.</returns>
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public T Pop<T>() where T : StackItem
        {
            return Remove<T>(0);
        }

        internal T Remove<T>(int index) where T : StackItem
        {
            if (index >= innerList.Count)
                throw new ArgumentOutOfRangeException(nameof(index));
            if (index < 0)
            {
                index += innerList.Count;
                if (index < 0)
                    throw new ArgumentOutOfRangeException(nameof(index));
            }
            index = innerList.Count - index - 1;
            if (innerList[index] is not T item)
                throw new InvalidCastException($"The item can't be casted to type {typeof(T)}");
            innerList.RemoveAt(index);
            referenceCounter.RemoveStackReference(item);
            return item;
        }

        public override string ToString()
        {
            return $"[{string.Join(", ", innerList.Select(p => $"{p.Type}({p})"))}]";
        }
    }
}
