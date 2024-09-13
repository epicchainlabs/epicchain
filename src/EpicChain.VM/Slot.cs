// Copyright (C) 2021-2024 EpicChain Labs.

//
// Slot.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Collections;
using System.Collections.Generic;

namespace EpicChain.VM
{
    /// <summary>
    /// Used to store local variables, arguments and static fields in the VM.
    /// </summary>
    public class Slot : IReadOnlyList<StackItem>
    {
        private readonly ReferenceCounter referenceCounter;
        private readonly StackItem[] items;

        /// <summary>
        /// Gets the item at the specified index in the slot.
        /// </summary>
        /// <param name="index">The zero-based index of the item to get.</param>
        /// <returns>The item at the specified index in the slot.</returns>
        public StackItem this[int index]
        {
            get
            {
                return items[index];
            }
            internal set
            {
                ref var oldValue = ref items[index];
                referenceCounter.RemoveStackReference(oldValue);
                oldValue = value;
                referenceCounter.AddStackReference(value);
            }
        }

        /// <summary>
        /// Gets the number of items in the slot.
        /// </summary>
        public int Count => items.Length;

        /// <summary>
        /// Creates a slot containing the specified items.
        /// </summary>
        /// <param name="items">The items to be contained.</param>
        /// <param name="referenceCounter">The reference counter to be used.</param>
        public Slot(StackItem[] items, ReferenceCounter referenceCounter)
        {
            this.referenceCounter = referenceCounter;
            this.items = items;
            foreach (StackItem item in items)
                referenceCounter.AddStackReference(item);
        }

        /// <summary>
        /// Create a slot of the specified size.
        /// </summary>
        /// <param name="count">Indicates the number of items contained in the slot.</param>
        /// <param name="referenceCounter">The reference counter to be used.</param>
        public Slot(int count, ReferenceCounter referenceCounter)
        {
            this.referenceCounter = referenceCounter;
            items = new StackItem[count];
            System.Array.Fill(items, StackItem.Null);
            referenceCounter.AddStackReference(StackItem.Null, count);
        }

        internal void ClearReferences()
        {
            foreach (StackItem item in items)
                referenceCounter.RemoveStackReference(item);
        }

        IEnumerator<StackItem> IEnumerable<StackItem>.GetEnumerator()
        {
            foreach (StackItem item in items) yield return item;
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            return items.GetEnumerator();
        }
    }
}
