// Copyright (C) 2021-2024 EpicChain Labs.

//
// VMUnhandledException.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.VM.Types;
using System;
using System.Text;
using Array = EpicChain.VM.Types.Array;

namespace EpicChain.VM
{
    /// <summary>
    /// Represents an unhandled exception in the VM.
    /// Thrown when there is an exception in the VM that is not caught by any script.
    /// </summary>
    public class VMUnhandledException : Exception
    {
        /// <summary>
        /// The unhandled exception in the VM.
        /// </summary>
        public StackItem ExceptionObject { get; }

        /// <summary>
        /// Initializes a new instance of the <see cref="VMUnhandledException"/> class.
        /// </summary>
        /// <param name="ex">The unhandled exception in the VM.</param>
        public VMUnhandledException(StackItem ex) : base(GetExceptionMessage(ex))
        {
            ExceptionObject = ex;
        }

        private static string GetExceptionMessage(StackItem e)
        {
            StringBuilder sb = new("An unhandled exception was thrown.");
            ByteString? s = e as ByteString;
            if (s is null && e is Array array && array.Count > 0)
                s = array[0] as ByteString;
            if (s != null)
            {
                sb.Append(' ');
                sb.Append(Encoding.UTF8.GetString(s.GetSpan()));
            }
            return sb.ToString();
        }
    }
}
