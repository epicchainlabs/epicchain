// EpicChain Copyright Project (2021-2024)
// 
// Copyright (c) 2021-2024 EpicChain
// 
// EpicChain is an innovative blockchain network developed and maintained by xmoohad. This copyright project outlines the rights and responsibilities associated with the EpicChain software and its related components.
// 
// 1. Copyright Holder:
//    - xmoohad
// 
// 2. Project Name:
//    - EpicChain
// 
// 3. Project Description:
//    - EpicChain is a decentralized blockchain network that aims to revolutionize the way digital assets are managed, traded, and secured. With its innovative features and robust architecture, EpicChain provides a secure and efficient platform for various decentralized applications (dApps) and digital asset management.
// 
// 4. Copyright Period:
//    - The copyright for the EpicChain software and its related components is valid from 2021 to 2024.
// 
// 5. Copyright Statement:
//    - All rights reserved. No part of the EpicChain software or its related components may be reproduced, distributed, or transmitted in any form or by any means, without the prior written permission of the copyright holder, except in the case of brief quotations embodied in critical reviews and certain other noncommercial uses permitted by copyright law.
// 
// 6. License:
//    - The EpicChain software is licensed under the EpicChain Software License, a custom license that governs the use, distribution, and modification of the software. The EpicChain Software License is designed to promote the free and open development of the EpicChain network while protecting the interests of the copyright holder.
// 
// 7. Open Source:
//    - EpicChain is an open-source project, and its source code is available to the public under the terms of the EpicChain Software License. Developers are encouraged to contribute to the development of EpicChain and create innovative applications on top of the EpicChain network.
// 
// 8. Disclaimer:
//    - The EpicChain software and its related components are provided "as is," without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall the copyright holder or contributors be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the EpicChain software or its related components.
// 
// 9. Contact Information:
//    - For inquiries regarding the EpicChain copyright project, please contact xmoohad at [email address].
// 
// 10. Updates:
//     - This copyright project may be updated or modified from time to time to reflect changes in the EpicChain project or to address new legal or regulatory requirements. Users and developers are encouraged to check the latest version of the copyright project periodically.

//
// IndexedQueue.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace Neo.IO.Caching
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
                int newSize = _array.Length * GrowthFactor;
                if (_head == 0)
                {
                    Array.Resize(ref _array, newSize);
                }
                else
                {
                    T[] buffer = new T[newSize];
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
                item = default;
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
            T result = _array[_head];
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
                item = default;
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
                T[] arr = new T[_count];
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
            T[] result = new T[_count];
            CopyTo(result, 0);
            return result;
        }

        public IEnumerator<T> GetEnumerator()
        {
            for (int i = 0; i < _count; i++)
                yield return _array[(_head + i) % _array.Length];
        }

        IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();
    }
}
