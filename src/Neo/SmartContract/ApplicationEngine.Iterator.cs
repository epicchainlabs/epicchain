// Copyright (C) 2021-2024 The EpicChain Labs.
//
// ApplicationEngine.Iterator.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.SmartContract.Iterators;
using Neo.VM.Types;

namespace Neo.SmartContract
{
    partial class ApplicationEngine
    {
        /// <summary>
        /// The <see cref="InteropDescriptor"/> of System.Iterator.Next.
        /// Advances the iterator to the next element of the collection.
        /// </summary>
        public static readonly InteropDescriptor System_Iterator_Next = Register("System.Iterator.Next", nameof(IteratorNext), 1 << 15, CallFlags.None);

        /// <summary>
        /// The <see cref="InteropDescriptor"/> of System.Iterator.Value.
        /// Gets the element in the collection at the current position of the iterator.
        /// </summary>
        public static readonly InteropDescriptor System_Iterator_Value = Register("System.Iterator.Value", nameof(IteratorValue), 1 << 4, CallFlags.None);

        /// <summary>
        /// The implementation of System.Iterator.Next.
        /// Advances the iterator to the next element of the collection.
        /// </summary>
        /// <param name="iterator">The iterator to be advanced.</param>
        /// <returns><see langword="true"/> if the iterator was successfully advanced to the next element; <see langword="false"/> if the iterator has passed the end of the collection.</returns>
        internal protected static bool IteratorNext(IIterator iterator)
        {
            return iterator.Next();
        }

        /// <summary>
        /// The implementation of System.Iterator.Value.
        /// Gets the element in the collection at the current position of the iterator.
        /// </summary>
        /// <param name="iterator">The iterator to be used.</param>
        /// <returns>The element in the collection at the current position of the iterator.</returns>
        internal protected StackItem IteratorValue(IIterator iterator)
        {
            return iterator.Value(ReferenceCounter);
        }
    }
}
