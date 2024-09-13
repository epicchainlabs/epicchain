// Copyright (C) 2021-2024 EpicChain Labs.

//
// IndexedQueue.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

namespace EpicChain.IO.Caching
{
    /// <summary>
    /// Represents a queue with indexed access to the items
    /// </summary>
    /// <typeparam name="T">The type of items in the queue</typeparam>
    class IndexedQueue<T> : IReadOnlyCollection<T>
    {
        private const int DefaultCapacity = 16;
        private const int GrowthFactor = 2;
        private const float TrimThreshold = 0.9f;

        private T[] _array;
        private int _head;
        private int _count;

        /// <summary>
        /// Indicates the count of items in the queue
        /// </summary>
        public int Count => _count;

        /// <summary>
        /// Creates a queue with the default capacity
        /// </summary>
        public IndexedQueue() : this(DefaultCapacity)
        {
        }

        /// <summary>
        /// Creates a queue with the specified capacity
        /// </summary>
        /// <param name="capacity">The initial capacity of the queue</param>
        public IndexedQueue(int capacity)
        {
            if (capacity <= 0)
                throw new ArgumentOutOfRangeException(nameof(capacity), "The capacity must be greater than zero.");
            _array = new T[capacity];
            _head = 0;
            _count = 0;
        }

        /// <summary>
        /// Creates a queue filled with the specified items
        /// </summary>
        /// <param name="collection">The collection of items to fill the queue with</param>
        public IndexedQueue(IEnumerable<T> collection)
        {
            _array = collection.ToArray();
            _head = 0;
            _count = _array.Length;
        }

        /// <summary>
        /// Gets the value at the index
        /// </summary>
        /// <param name="index">The index</param>
        /// <returns>The value at the specified index</returns>
        public ref T this[int index]
        {
            get
            {
                if (index < 0 || index >= _count)
                    throw new IndexOutOfRangeException();
                return ref _array[(index + _head) % _array.Length];
            }
        }

        /// <summary>
        /// Inserts an item at the rear of the queue
        /// </summary>
        /// <param name="item">The item to insert</param>
        public void Enqueue(T item)
        {
            if (_array.Length == _count)
            {
                var newSize = _array.Length * GrowthFactor;
                if (_head == 0)
                {
                    Array.Resize(ref _array, newSize);
                }
                else
                {
                    var buffer = new T[newSize];
                    Array.Copy(_array, _head, buffer, 0, _array.Length - _head);
                    Array.Copy(_array, 0, buffer, _array.Length - _head, _head);
                    _array = buffer;
                    _head = 0;
                }
            }
            _array[(_head + _count) % _array.Length] = item;
            ++_count;
        }

        /// <summary>
        /// Provides access to the item at the front of the queue without dequeuing it
        /// </summary>
        /// <returns>The front most item</returns>
        public T Peek()
        {
            if (_count == 0)
                throw new InvalidOperationException("The queue is empty.");
            return _array[_head];
        }

        /// <summary>
        /// Attempts to return an item from the front of the queue without removing it
        /// </summary>
        /// <param name="item">The item</param>
        /// <returns>True if the queue returned an item or false if the queue is empty</returns>
        public bool TryPeek(out T item)
        {
            if (_count == 0)
            {
                item = default!;
                return false;
            }
            else
            {
                item = _array[_head];
                return true;
            }
        }

        /// <summary>
        /// Removes an item from the front of the queue, returning it
        /// </summary>
        /// <returns>The item that was removed</returns>
        public T Dequeue()
        {
            if (_count == 0)
                throw new InvalidOperationException("The queue is empty");
            var result = _array[_head];
            ++_head;
            _head %= _array.Length;
            --_count;
            return result;
        }

        /// <summary>
        /// Attempts to return an item from the front of the queue, removing it
        /// </summary>
        /// <param name="item">The item</param>
        /// <returns>True if the queue returned an item or false if the queue is empty</returns>
        public bool TryDequeue(out T item)
        {
            if (_count == 0)
            {
                item = default!;
                return false;
            }
            else
            {
                item = _array[_head];
                ++_head;
                _head %= _array.Length;
                --_count;
                return true;
            }
        }

        /// <summary>
        /// Clears the items from the queue
        /// </summary>
        public void Clear()
        {
            _head = 0;
            _count = 0;
        }

        /// <summary>
        /// Trims the extra array space that isn't being used.
        /// </summary>
        public void TrimExcess()
        {
            if (_count == 0)
            {
                _array = new T[DefaultCapacity];
            }
            else if (_array.Length * TrimThreshold >= _count)
            {
                var arr = new T[_count];
                CopyTo(arr, 0);
                _array = arr;
                _head = 0;
            }
        }

        /// <summary>
        /// Copy the queue's items to a destination array
        /// </summary>
        /// <param name="array">The destination array</param>
        /// <param name="arrayIndex">The index in the destination to start copying at</param>
        public void CopyTo(T[] array, int arrayIndex)
        {
            if (array is null) throw new ArgumentNullException(nameof(array));
            if (arrayIndex < 0 || arrayIndex + _count > array.Length)
                throw new ArgumentOutOfRangeException(nameof(arrayIndex));
            if (_head + _count <= _array.Length)
            {
                Array.Copy(_array, _head, array, arrayIndex, _count);
            }
            else
            {
                Array.Copy(_array, _head, array, arrayIndex, _array.Length - _head);
                Array.Copy(_array, 0, array, arrayIndex + _array.Length - _head, _count + _head - _array.Length);
            }
        }

        /// <summary>
        /// Returns an array of the items in the queue
        /// </summary>
        /// <returns>An array containing the queue's items</returns>
        public T[] ToArray()
        {
            var result = new T[_count];
            CopyTo(result, 0);
            return result;
        }

        public IEnumerator<T> GetEnumerator()
        {
            for (var i = 0; i < _count; i++)
                yield return _array[(_head + i) % _array.Length];
        }

        IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();
    }
}
