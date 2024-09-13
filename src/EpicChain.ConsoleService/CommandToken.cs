// Copyright (C) 2021-2024 EpicChain Labs.

//
// CommandToken.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Text;

namespace EpicChain.ConsoleService
{
    internal abstract class CommandToken
    {
        /// <summary>
        /// Offset
        /// </summary>
        public int Offset { get; }

        /// <summary>
        /// Type
        /// </summary>
        public CommandTokenType Type { get; }

        /// <summary>
        /// Value
        /// </summary>
        public string Value { get; protected init; } = string.Empty;

        /// <summary>
        /// Constructor
        /// </summary>
        /// <param name="type">Type</param>
        /// <param name="offset">Offset</param>
        protected CommandToken(CommandTokenType type, int offset)
        {
            Type = type;
            Offset = offset;
        }

        /// <summary>
        /// Parse command line
        /// </summary>
        /// <param name="commandLine">Command line</param>
        /// <returns></returns>
        public static IEnumerable<CommandToken> Parse(string commandLine)
        {
            CommandToken? lastToken = null;

            for (int index = 0, count = commandLine.Length; index < count;)
            {
                switch (commandLine[index])
                {
                    case ' ':
                        {
                            lastToken = CommandSpaceToken.Parse(commandLine, ref index);
                            yield return lastToken;
                            break;
                        }
                    case '"':
                    case '\'':
                        {
                            // "'"
                            if (lastToken is CommandQuoteToken quote && quote.Value[0] != commandLine[index])
                            {
                                goto default;
                            }

                            lastToken = CommandQuoteToken.Parse(commandLine, ref index);
                            yield return lastToken;
                            break;
                        }
                    default:
                        {
                            lastToken = CommandStringToken.Parse(commandLine, ref index,
                                lastToken is CommandQuoteToken quote ? quote : null);

                            if (lastToken is not null)
                            {
                                yield return lastToken;
                            }
                            break;
                        }
                }
            }
        }

        /// <summary>
        /// Create string arguments
        /// </summary>
        /// <param name="tokens">Tokens</param>
        /// <param name="removeEscape">Remove escape</param>
        /// <returns>Arguments</returns>
        public static string[] ToArguments(IEnumerable<CommandToken> tokens, bool removeEscape = true)
        {
            var list = new List<string>();

            CommandToken? lastToken = null;

            foreach (var token in tokens)
            {
                if (token is CommandStringToken str)
                {
                    if (removeEscape && lastToken is CommandQuoteToken quote)
                    {
                        // Remove escape

                        list.Add(str.Value.Replace("\\" + quote.Value, quote.Value));
                    }
                    else
                    {
                        list.Add(str.Value);
                    }
                }

                lastToken = token;
            }

            return list.ToArray();
        }

        /// <summary>
        /// Create a string from token list
        /// </summary>
        /// <param name="tokens">Tokens</param>
        /// <returns>String</returns>
        public static string ToString(IEnumerable<CommandToken> tokens)
        {
            var sb = new StringBuilder();

            foreach (var token in tokens)
            {
                sb.Append(token.Value);
            }

            return sb.ToString();
        }

        /// <summary>
        /// Trim
        /// </summary>
        /// <param name="args">Args</param>
        public static void Trim(List<CommandToken> args)
        {
            // Trim start

            while (args.Count > 0 && args[0].Type == CommandTokenType.Space)
            {
                args.RemoveAt(0);
            }

            // Trim end

            while (args.Count > 0 && args[^1].Type == CommandTokenType.Space)
            {
                args.RemoveAt(args.Count - 1);
            }
        }

        /// <summary>
        /// Read String
        /// </summary>
        /// <param name="args">Args</param>
        /// <param name="consumeAll">Consume all if not quoted</param>
        /// <returns>String</returns>
        public static string? ReadString(List<CommandToken> args, bool consumeAll)
        {
            Trim(args);

            var quoted = false;

            if (args.Count > 0 && args[0].Type == CommandTokenType.Quote)
            {
                quoted = true;
                args.RemoveAt(0);
            }
            else
            {
                if (consumeAll)
                {
                    // Return all if it's not quoted

                    var ret = ToString(args);
                    args.Clear();

                    return ret;
                }
            }

            if (args.Count > 0)
            {
                switch (args[0])
                {
                    case CommandQuoteToken _:
                        {
                            if (quoted)
                            {
                                args.RemoveAt(0);
                                return "";
                            }

                            throw new ArgumentException();
                        }
                    case CommandSpaceToken _: throw new ArgumentException();
                    case CommandStringToken str:
                        {
                            args.RemoveAt(0);

                            if (quoted && args.Count > 0 && args[0].Type == CommandTokenType.Quote)
                            {
                                // Remove last quote

                                args.RemoveAt(0);
                            }

                            return str.Value;
                        }
                }
            }

            return null;
        }
    }
}
