// Copyright (C) 2021-2024 EpicChain Labs.

//
// Pointer.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
    /// Represents the instruction pointer in the VM, used as the target of jump instructions.
    /// </summary>
    [DebuggerDisplay("Type={GetType().Name}, Position={Position}")]
    public class Pointer : StackItem
    {
        /// <summary>
        /// The <see cref="VM.Script"/> object containing this pointer.
        /// </summary>
        public Script Script { get; }

        /// <summary>
        /// The position of the pointer in the script.
        /// </summary>
        public int Position { get; }

        public override StackItemType Type => StackItemType.Pointer;

        /// <summary>
        /// Create a code pointer with the specified script and position.
        /// </summary>
        /// <param name="script">The <see cref="VM.Script"/> object containing this pointer.</param>
        /// <param name="position">The position of the pointer in the script.</param>
        public Pointer(Script script, int position)
        {
            Script = script;
            Position = position;
        }

        public override bool Equals(StackItem? other)
        {
            if (other == this) return true;
            if (other is Pointer p) return Position == p.Position && Script == p.Script;
            return false;
        }

        public override bool GetBoolean()
        {
            return true;
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(Script, Position);
        }

        public override string ToString()
        {
            return Position.ToString();
        }
    }
}
