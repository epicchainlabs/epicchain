// Copyright (C) 2021-2024 EpicChain Labs.

//
// CommandStringToken.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
    [DebuggerDisplay("Value={Value}, RequireQuotes={RequireQuotes}")]
    internal class CommandStringToken : CommandToken
    {
        /// <summary>
        /// Require quotes
        /// </summary>
        public bool RequireQuotes { get; }

        /// <summary>
        /// Constructor
        /// </summary>
        /// <param name="offset">Offset</param>
        /// <param name="value">Value</param>
        public CommandStringToken(int offset, string value) : base(CommandTokenType.String, offset)
        {
            Value = value;
            RequireQuotes = value.IndexOfAny(new char[] { '\'', '"' }) != -1;
        }

        /// <summary>
        /// Parse command line spaces
        /// </summary>
        /// <param name="commandLine">Command line</param>
        /// <param name="index">Index</param>
        /// <param name="quote">Quote (could be null)</param>
        /// <returns>CommandSpaceToken</returns>
        internal static CommandStringToken Parse(string commandLine, ref int index, CommandQuoteToken? quote)
        {
            int end;
            int offset = index;

            if (quote != null)
            {
                var ix = index;

                do
                {
                    end = commandLine.IndexOf(quote.Value[0], ix + 1);

                    if (end == -1)
                    {
                        throw new ArgumentException("String not closed");
                    }

                    if (IsScaped(commandLine, end - 1))
                    {
                        ix = end;
                        end = -1;
                    }
                }
                while (end < 0);
            }
            else
            {
                end = commandLine.IndexOf(' ', index + 1);
            }

            if (end == -1)
            {
                end = commandLine.Length;
            }

            var ret = new CommandStringToken(offset, commandLine.Substring(index, end - index));
            index += end - index;
            return ret;
        }

        private static bool IsScaped(string commandLine, int index)
        {
            // TODO: Scape the scape

            return (commandLine[index] == '\\');
        }
    }
}
