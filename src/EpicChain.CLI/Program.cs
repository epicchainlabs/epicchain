// Copyright (C) 2021-2024 EpicChain Labs.

//
// Program.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Threading.Tasks;
using EpicChain;
using EpicChain.CLI.Commands;
using McMaster.Extensions.CommandLineUtils;
using Microsoft.Extensions.DependencyInjection;

namespace EpicChain.CLI
{
    public class Program
    {
        public static async Task<int> Main(string[] args)
        {
            var services = new ServiceCollection()
                .AddSingleton<ProtocolSettings>()
                .AddSingleton<EpicChainSystem>()
                .AddSingleton<ConsensusEngine>()
                .AddSingleton<IConsensusState, ConsensusState>()
                .AddSingleton<IValidator, Validator>()
                .AddSingleton<IBlockProducer, BlockProducer>()
                .BuildServiceProvider();

            var app = new CommandLineApplication
            {
                Name = "epicchain",
                Description = "EpicChain CLI - Command line interface for EpicChain blockchain"
            };

            app.HelpOption();

            app.Command("node", new NodeCommands(services).OnExecuteAsync);
            app.Command("consensus", new ConsensusCommands(services).OnExecuteAsync);

            try
            {
                return await app.ExecuteAsync(args);
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"Error: {ex.Message}");
                return 1;
            }
        }
    }
}
