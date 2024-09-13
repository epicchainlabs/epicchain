// Copyright (C) 2021-2024 EpicChain Labs.

//
// IStore.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

namespace EpicChain.Persistence
{
    /// <summary>
    /// This interface provides methods for reading, writing from/to database. Developers should implement this interface to provide new storage engines for EpicChain.
    /// </summary>
    public interface IStore : IDisposable, IReadOnlyStore
    {
        /// <summary>
        /// Deletes an entry from the database.
        /// </summary>
        /// <param name="key">The key of the entry.</param>
        void Delete(byte[] key);

        /// <summary>
        /// Creates a snapshot of the database.
        /// </summary>
        /// <returns>A snapshot of the database.</returns>
        ISnapshot GetSnapshot();

        /// <summary>
        /// Puts an entry to the database.
        /// </summary>
        /// <param name="key">The key of the entry.</param>
        /// <param name="value">The data of the entry.</param>
        void Put(byte[] key, byte[] value);

        /// <summary>
        /// Puts an entry to the database synchronously.
        /// </summary>
        /// <param name="key">The key of the entry.</param>
        /// <param name="value">The data of the entry.</param>
        void PutSync(byte[] key, byte[] value) => Put(key, value);
    }
}
