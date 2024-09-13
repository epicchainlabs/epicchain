// Copyright (C) 2021-2024 EpicChain Labs.

//
// IReadOnlyStore.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System.Collections.Generic;

namespace EpicChain.Persistence
{
    /// <summary>
    /// This interface provides methods to read from the database.
    /// </summary>
    public interface IReadOnlyStore
    {
        /// <summary>
        /// Seeks to the entry with the specified key.
        /// </summary>
        /// <param name="key">The key to be sought.</param>
        /// <param name="direction">The direction of seek.</param>
        /// <returns>An enumerator containing all the entries after seeking.</returns>
        IEnumerable<(byte[] Key, byte[] Value)> Seek(byte[] key, SeekDirection direction);

        /// <summary>
        /// Reads a specified entry from the database.
        /// </summary>
        /// <param name="key">The key of the entry.</param>
        /// <returns>The data of the entry. Or <see langword="null"/> if it doesn't exist.</returns>
        byte[] TryGet(byte[] key);

        /// <summary>
        /// Determines whether the database contains the specified entry.
        /// </summary>
        /// <param name="key">The key of the entry.</param>
        /// <returns><see langword="true"/> if the database contains an entry with the specified key; otherwise, <see langword="false"/>.</returns>
        bool Contains(byte[] key);
    }
}
