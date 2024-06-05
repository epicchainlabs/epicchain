// Copyright (C) 2021-2024 The EpicChain Labs.
//
// FindOptions.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using System;

namespace Neo.SmartContract
{
    /// <summary>
    /// Specify the options to be used during the search.
    /// </summary>
    [Flags]
    public enum FindOptions : byte
    {
        /// <summary>
        /// No option is set. The results will be an iterator of (key, value).
        /// </summary>
        None = 0,

        /// <summary>
        /// Indicates that only keys need to be returned. The results will be an iterator of keys.
        /// </summary>
        KeysOnly = 1 << 0,

        /// <summary>
        /// Indicates that the prefix byte of keys should be removed before return.
        /// </summary>
        RemovePrefix = 1 << 1,

        /// <summary>
        /// Indicates that only values need to be returned. The results will be an iterator of values.
        /// </summary>
        ValuesOnly = 1 << 2,

        /// <summary>
        /// Indicates that values should be deserialized before return.
        /// </summary>
        DeserializeValues = 1 << 3,

        /// <summary>
        /// Indicates that only the field 0 of the deserialized values need to be returned. This flag must be set together with <see cref="DeserializeValues"/>.
        /// </summary>
        PickField0 = 1 << 4,

        /// <summary>
        /// Indicates that only the field 1 of the deserialized values need to be returned. This flag must be set together with <see cref="DeserializeValues"/>.
        /// </summary>
        PickField1 = 1 << 5,

        /// <summary>
        /// Indicates that results should be returned in backwards (descending) order.
        /// </summary>
        Backwards = 1 << 7,

        /// <summary>
        /// This value is only for internal use, and shouldn't be used in smart contracts.
        /// </summary>
        All = KeysOnly | RemovePrefix | ValuesOnly | DeserializeValues | PickField0 | PickField1 | Backwards
    }
}
