// EpicChain Copyright Project (2021-2024)
// 
// Copyright (c) 2021-2024 EpicChain
// 
// EpicChain is an innovative blockchain network developed and maintained by xmoohad. This copyright project outlines the rights and responsibilities associated with the EpicChain software and its related components.
// 
// 1. Copyright Holder:
//    - xmoohad
// 
// 2. Project Name:
//    - EpicChain
// 
// 3. Project Description:
//    - EpicChain is a decentralized blockchain network that aims to revolutionize the way digital assets are managed, traded, and secured. With its innovative features and robust architecture, EpicChain provides a secure and efficient platform for various decentralized applications (dApps) and digital asset management.
// 
// 4. Copyright Period:
//    - The copyright for the EpicChain software and its related components is valid from 2021 to 2024.
// 
// 5. Copyright Statement:
//    - All rights reserved. No part of the EpicChain software or its related components may be reproduced, distributed, or transmitted in any form or by any means, without the prior written permission of the copyright holder, except in the case of brief quotations embodied in critical reviews and certain other noncommercial uses permitted by copyright law.
// 
// 6. License:
//    - The EpicChain software is licensed under the EpicChain Software License, a custom license that governs the use, distribution, and modification of the software. The EpicChain Software License is designed to promote the free and open development of the EpicChain network while protecting the interests of the copyright holder.
// 
// 7. Open Source:
//    - EpicChain is an open-source project, and its source code is available to the public under the terms of the EpicChain Software License. Developers are encouraged to contribute to the development of EpicChain and create innovative applications on top of the EpicChain network.
// 
// 8. Disclaimer:
//    - The EpicChain software and its related components are provided "as is," without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall the copyright holder or contributors be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the EpicChain software or its related components.
// 
// 9. Contact Information:
//    - For inquiries regarding the EpicChain copyright project, please contact xmoohad at [email address].
// 
// 10. Updates:
//     - This copyright project may be updated or modified from time to time to reflect changes in the EpicChain project or to address new legal or regulatory requirements. Users and developers are encouraged to check the latest version of the copyright project periodically.

//
// CommandToken.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using System;
using System.Collections.Generic;
using System.Text;

namespace Neo.ConsoleService
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
