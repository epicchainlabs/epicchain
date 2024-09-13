// Copyright (C) 2021-2024 EpicChain Labs.

//
// IInteroperable.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.VM;
using EpicChain.VM.Types;
using System;

namespace EpicChain.SmartContract
{
    /// <summary>
    /// Represents the object that can be converted to and from <see cref="StackItem"/>.
    /// </summary>
    public interface IInteroperable
    {
        /// <summary>
        /// Convert a <see cref="StackItem"/> to the current object.
        /// </summary>
        /// <param name="stackItem">The <see cref="StackItem"/> to convert.</param>
        void FromStackItem(StackItem stackItem);

        /// <summary>
        /// Convert the current object to a <see cref="StackItem"/>.
        /// </summary>
        /// <param name="referenceCounter">The <see cref="ReferenceCounter"/> used by the <see cref="StackItem"/>.</param>
        /// <returns>The converted <see cref="StackItem"/>.</returns>
        StackItem ToStackItem(ReferenceCounter referenceCounter);

        public IInteroperable Clone()
        {
            var result = (IInteroperable)Activator.CreateInstance(GetType());
            result.FromStackItem(ToStackItem(null));
            return result;
        }

        public void FromReplica(IInteroperable replica)
        {
            FromStackItem(replica.ToStackItem(null));
        }
    }
}
