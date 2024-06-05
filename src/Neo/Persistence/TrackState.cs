// Copyright (C) 2021-2024 The EpicChain Labs.
//
// TrackState.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


namespace Neo.Persistence
{
    /// <summary>
    /// Represents the state of a cached entry.
    /// </summary>
    public enum TrackState : byte
    {
        /// <summary>
        /// Indicates that the entry has been loaded from the underlying storage, but has not been modified.
        /// </summary>
        None,

        /// <summary>
        /// Indicates that this is a newly added record.
        /// </summary>
        Added,

        /// <summary>
        /// Indicates that the entry has been loaded from the underlying storage, and has been modified.
        /// </summary>
        Changed,

        /// <summary>
        /// Indicates that the entry should be deleted from the underlying storage when committing.
        /// </summary>
        Deleted,

        /// <summary>
        /// Indicates that the entry was not found in the underlying storage.
        /// </summary>
        NotFound
    }
}
