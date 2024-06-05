// Copyright (C) 2021-2024 The EpicChain Labs.
//
// IIterator.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.VM;
using Neo.VM.Types;
using System;

namespace Neo.SmartContract.Iterators
{
    /// <summary>
    /// Represents iterators in smart contract.
    /// </summary>
    public interface IIterator : IDisposable
    {
        /// <summary>
        /// Advances the iterator to the next element of the collection.
        /// </summary>
        /// <returns><see langword="true"/> if the iterator was successfully advanced to the next element; <see langword="false"/> if the iterator has passed the end of the collection.</returns>
        bool Next();

        /// <summary>
        /// Gets the element in the collection at the current position of the iterator.
        /// </summary>
        /// <returns>The element in the collection at the current position of the iterator.</returns>
        StackItem Value(ReferenceCounter referenceCounter);
    }
}
