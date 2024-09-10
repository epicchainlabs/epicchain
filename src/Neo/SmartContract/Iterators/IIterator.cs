// Copyright (C) 2021-2024 EpicChain Labs.

//
// IIterator.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
