// Copyright (C) 2021-2024 EpicChain Labs.

//
// FindOptions.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

namespace EpicChain.SmartContract
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
