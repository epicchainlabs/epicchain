// Copyright (C) 2021-2024 EpicChain Labs.

//
// ContractParameterType.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


namespace EpicChain.SmartContract
{
    /// <summary>
    /// Represents the type of <see cref="ContractParameter"/>.
    /// </summary>
    public enum ContractParameterType : byte
    {
        /// <summary>
        /// Indicates that the parameter can be of any type.
        /// </summary>
        Any = 0x00,

        /// <summary>
        /// Indicates that the parameter is of Boolean type.
        /// </summary>
        Boolean = 0x10,

        /// <summary>
        /// Indicates that the parameter is an integer.
        /// </summary>
        Integer = 0x11,

        /// <summary>
        /// Indicates that the parameter is a byte array.
        /// </summary>
        ByteArray = 0x12,

        /// <summary>
        /// Indicates that the parameter is a string.
        /// </summary>
        String = 0x13,

        /// <summary>
        /// Indicates that the parameter is a 160-bit hash.
        /// </summary>
        Hash160 = 0x14,

        /// <summary>
        /// Indicates that the parameter is a 256-bit hash.
        /// </summary>
        Hash256 = 0x15,

        /// <summary>
        /// Indicates that the parameter is a public key.
        /// </summary>
        PublicKey = 0x16,

        /// <summary>
        /// Indicates that the parameter is a signature.
        /// </summary>
        Signature = 0x17,

        /// <summary>
        /// Indicates that the parameter is an array.
        /// </summary>
        Array = 0x20,

        /// <summary>
        /// Indicates that the parameter is a map.
        /// </summary>
        Map = 0x22,

        /// <summary>
        /// Indicates that the parameter is an interoperable interface.
        /// </summary>
        InteropInterface = 0x30,

        /// <summary>
        /// It can be only used as the return type of a method, meaning that the method has no return value.
        /// </summary>
        Void = 0xff
    }
}
