// Copyright (C) 2021-2024 EpicChain Labs.

//
// StackItemType.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


namespace EpicChain.VM.Types
{
    /// <summary>
    /// An enumeration representing the types in the VM.
    /// </summary>
    public enum StackItemType : byte
    {
        /// <summary>
        /// Represents any type.
        /// </summary>
        Any = 0x00,

        /// <summary>
        /// Represents a code pointer.
        /// </summary>
        Pointer = 0x10,

        /// <summary>
        /// Represents the boolean (<see langword="true" /> or <see langword="false" />) type.
        /// </summary>
        Boolean = 0x20,

        /// <summary>
        /// Represents an integer.
        /// </summary>
        Integer = 0x21,

        /// <summary>
        /// Represents an immutable memory block.
        /// </summary>
        ByteString = 0x28,

        /// <summary>
        /// Represents a memory block that can be used for reading and writing.
        /// </summary>
        Buffer = 0x30,

        /// <summary>
        /// Represents an array or a complex object.
        /// </summary>
        Array = 0x40,

        /// <summary>
        /// Represents a structure.
        /// </summary>
        Struct = 0x41,

        /// <summary>
        /// Represents an ordered collection of key-value pairs.
        /// </summary>
        Map = 0x48,

        /// <summary>
        /// Represents an interface used to interoperate with the outside of the the VM.
        /// </summary>
        InteropInterface = 0x60,
    }
}
