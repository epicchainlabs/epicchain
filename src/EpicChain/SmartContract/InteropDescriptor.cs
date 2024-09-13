// Copyright (C) 2021-2024 EpicChain Labs.

//
// InteropDescriptor.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Cryptography;
using System.Buffers.Binary;
using System.Collections.Generic;
using System.Linq;
using System.Reflection;
using System.Text;

namespace EpicChain.SmartContract
{
    /// <summary>
    /// Represents a descriptor of an interoperable service.
    /// </summary>
    public record InteropDescriptor
    {
        /// <summary>
        /// The name of the interoperable service.
        /// </summary>
        public string Name { get; init; }

        private uint _hash;
        /// <summary>
        /// The hash of the interoperable service.
        /// </summary>
        public uint Hash
        {
            get
            {
                if (_hash == 0)
                    _hash = BinaryPrimitives.ReadUInt32LittleEndian(Encoding.ASCII.GetBytes(Name).Sha256());
                return _hash;
            }
        }

        /// <summary>
        /// The <see cref="MethodInfo"/> used to handle the interoperable service.
        /// </summary>
        public MethodInfo Handler { get; init; }

        private IReadOnlyList<InteropParameterDescriptor> _parameters;
        /// <summary>
        /// The parameters of the interoperable service.
        /// </summary>
        public IReadOnlyList<InteropParameterDescriptor> Parameters => _parameters ??= Handler.GetParameters().Select(p => new InteropParameterDescriptor(p)).ToList().AsReadOnly();

        /// <summary>
        /// The fixed price for calling the interoperable service. It can be 0 if the interoperable service has a variable price.
        /// </summary>
        public long FixedPrice { get; init; }

        /// <summary>
        /// The required <see cref="CallFlags"/> for the interoperable service.
        /// </summary>
        public CallFlags RequiredCallFlags { get; init; }

        public static implicit operator uint(InteropDescriptor descriptor)
        {
            return descriptor.Hash;
        }
    }
}
