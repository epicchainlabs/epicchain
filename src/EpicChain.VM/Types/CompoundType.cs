// Copyright (C) 2021-2024 EpicChain Labs.

//
// CompoundType.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Collections.Generic;
using System.Diagnostics;

namespace EpicChain.VM.Types
{
    /// <summary>
    /// The base class for complex types in the VM.
    /// </summary>
    [DebuggerDisplay("Type={GetType().Name}, Count={Count}, Id={System.Collections.Generic.ReferenceEqualityComparer.Instance.GetHashCode(this)}")]
    public abstract class CompoundType : StackItem
    {
        /// <summary>
        /// The reference counter used to count the items in the VM object.
        /// </summary>
        protected internal readonly ReferenceCounter? ReferenceCounter;

        /// <summary>
        /// Create a new <see cref="CompoundType"/> with the specified reference counter.
        /// </summary>
        /// <param name="referenceCounter">The reference counter to be used.</param>
        protected CompoundType(ReferenceCounter? referenceCounter)
        {
            ReferenceCounter = referenceCounter;
            referenceCounter?.AddZeroReferred(this);
        }

        /// <summary>
        /// The number of items in this VM object.
        /// </summary>
        public abstract int Count { get; }

        public abstract IEnumerable<StackItem> SubItems { get; }

        public abstract int SubItemsCount { get; }

        public bool IsReadOnly { get; protected set; }

        /// <summary>
        /// Remove all items from the VM object.
        /// </summary>
        public abstract void Clear();

        internal abstract override StackItem DeepCopy(Dictionary<StackItem, StackItem> refMap, bool asImmutable);

        public sealed override bool GetBoolean()
        {
            return true;
        }

        /// <summary>
        /// The operation is not supported. Always throw <see cref="NotSupportedException"/>.
        /// </summary>
        /// <exception cref="NotSupportedException">This method always throws the exception.</exception>
        public override int GetHashCode()
        {
            throw new NotSupportedException();
        }

        public override string ToString()
        {
            return Count.ToString();
        }
    }
}
