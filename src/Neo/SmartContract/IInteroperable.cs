// Copyright (C) 2021-2024 The EpicChain Labs.
//
// IInteroperable.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.VM;
using Neo.VM.Types;
using System;

namespace Neo.SmartContract
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
