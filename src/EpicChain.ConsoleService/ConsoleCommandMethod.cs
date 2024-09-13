// Copyright (C) 2021-2024 EpicChain Labs.

//
// ConsoleCommandMethod.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System.Collections.Generic;
using System.Diagnostics;
using System.Reflection;

namespace EpicChain.ConsoleService
{
    [DebuggerDisplay("Key={Key}")]
    internal class ConsoleCommandMethod
    {
        /// <summary>
        /// Verbs
        /// </summary>
        public string[] Verbs { get; }

        /// <summary>
        /// Key
        /// </summary>
        public string Key => string.Join(' ', Verbs);

        /// <summary>
        /// Help category
        /// </summary>
        public string HelpCategory { get; set; }

        /// <summary>
        /// Help message
        /// </summary>
        public string HelpMessage { get; set; }

        /// <summary>
        /// Instance
        /// </summary>
        public object Instance { get; }

        /// <summary>
        /// Method
        /// </summary>
        public MethodInfo Method { get; }

        /// <summary>
        /// Set instance command
        /// </summary>
        /// <param name="instance">Instance</param>
        /// <param name="method">Method</param>
        /// <param name="attribute">Attribute</param>
        public ConsoleCommandMethod(object instance, MethodInfo method, ConsoleCommandAttribute attribute)
        {
            Method = method;
            Instance = instance;
            Verbs = attribute.Verbs;
            HelpCategory = attribute.Category;
            HelpMessage = attribute.Description;
        }

        /// <summary>
        /// Is this command
        /// </summary>
        /// <param name="tokens">Tokens</param>
        /// <param name="consumedArgs">Consumed Arguments</param>
        /// <returns>True if is this command</returns>
        public bool IsThisCommand(CommandToken[] tokens, out int consumedArgs)
        {
            int checks = Verbs.Length;
            bool quoted = false;
            var tokenList = new List<CommandToken>(tokens);

            while (checks > 0 && tokenList.Count > 0)
            {
                switch (tokenList[0])
                {
                    case CommandSpaceToken _:
                        {
                            tokenList.RemoveAt(0);
                            break;
                        }
                    case CommandQuoteToken _:
                        {
                            quoted = !quoted;
                            tokenList.RemoveAt(0);
                            break;
                        }
                    case CommandStringToken str:
                        {
                            if (Verbs[^checks] != str.Value.ToLowerInvariant())
                            {
                                consumedArgs = 0;
                                return false;
                            }

                            checks--;
                            tokenList.RemoveAt(0);
                            break;
                        }
                }
            }

            if (quoted && tokenList.Count > 0 && tokenList[0].Type == CommandTokenType.Quote)
            {
                tokenList.RemoveAt(0);
            }

            // Trim start

            while (tokenList.Count > 0 && tokenList[0].Type == CommandTokenType.Space) tokenList.RemoveAt(0);

            consumedArgs = tokens.Length - tokenList.Count;
            return checks == 0;
        }
    }
}
