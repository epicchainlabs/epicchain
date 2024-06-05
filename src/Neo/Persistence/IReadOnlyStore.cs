// Copyright (C) 2021-2024 The EpicChain Labs.
//
// IReadOnlyStore.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using System.Collections.Generic;

namespace Neo.Persistence
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
