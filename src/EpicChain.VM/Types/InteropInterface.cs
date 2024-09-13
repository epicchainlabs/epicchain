// Copyright (C) 2021-2024 EpicChain Labs.

//
// InteropInterface.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Diagnostics;

namespace EpicChain.VM.Types
{
    /// <summary>
    /// Represents an interface used to interoperate with the outside of the the VM.
    /// </summary>
    [DebuggerDisplay("Type={GetType().Name}, Value={_object}")]
    public class InteropInterface : StackItem
    {
        private readonly object _object;

        public override StackItemType Type => StackItemType.InteropInterface;

        /// <summary>
        /// Create an interoperability interface that wraps the specified <see cref="object"/>.
        /// </summary>
        /// <param name="value">The wrapped <see cref="object"/>.</param>
        public InteropInterface(object value)
        {
            _object = value ?? throw new ArgumentNullException(nameof(value));
        }

        public override bool Equals(StackItem? other)
        {
            if (ReferenceEquals(this, other)) return true;
            if (other is InteropInterface i) return _object.Equals(i._object);
            return false;
        }

        public override bool GetBoolean()
        {
            return true;
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(_object);
        }

        public override T GetInterface<T>()
        {
            if (_object is T t) return t;
            throw new InvalidCastException($"The item can't be casted to type {typeof(T)}");
        }

        internal object GetInterface()
        {
            return _object;
        }

        public override string ToString()
        {
            return _object.ToString() ?? "NULL";
        }
    }
}
