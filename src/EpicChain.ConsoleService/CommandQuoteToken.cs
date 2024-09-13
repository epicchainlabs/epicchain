// Copyright (C) 2021-2024 EpicChain Labs.

//
// CommandQuoteToken.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

namespace EpicChain.ConsoleService
{
    [DebuggerDisplay("Value={Value}, Value={Value}")]
    internal class CommandQuoteToken : CommandToken
    {
        /// <summary>
        /// Constructor
        /// </summary>
        /// <param name="offset">Offset</param>
        /// <param name="value">Value</param>
        public CommandQuoteToken(int offset, char value) : base(CommandTokenType.Quote, offset)
        {
            if (value != '\'' && value != '"')
            {
                throw new ArgumentException("Not valid quote");
            }

            Value = value.ToString();
        }

        /// <summary>
        /// Parse command line quotes
        /// </summary>
        /// <param name="commandLine">Command line</param>
        /// <param name="index">Index</param>
        /// <returns>CommandQuoteToken</returns>
        internal static CommandQuoteToken Parse(string commandLine, ref int index)
        {
            var c = commandLine[index];

            if (c == '\'' || c == '"')
            {
                index++;
                return new CommandQuoteToken(index - 1, c);
            }

            throw new ArgumentException("No quote found");
        }
    }
}
