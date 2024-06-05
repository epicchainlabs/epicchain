// Copyright (C) 2021-2024 The EpicChain Labs.
//
// StorageContext.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


namespace Neo.SmartContract
{
    /// <summary>
    /// The storage context used to read and write data in smart contracts.
    /// </summary>
    public class StorageContext
    {
        /// <summary>
        /// The id of the contract that owns the context.
        /// </summary>
        public int Id;

        /// <summary>
        /// Indicates whether the context is read-only.
        /// </summary>
        public bool IsReadOnly;
    }
}
