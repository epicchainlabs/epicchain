// =============================================================================================
//  © Copyright (C) 2021-2025 EpicChain Labs. All rights reserved.
// =============================================================================================
//
//  File: MainService.CommandLine.cs
//  Project: EpicChain Labs - Core Blockchain Infrastructure
//  Author: Xmoohad (Muhammad Ibrahim Muhammad)
//
// ---------------------------------------------------------------------------------------------
//  Description:
//  This file is an integral part of the EpicChain Labs ecosystem, a forward-looking, open-source
//  blockchain initiative founded by Xmoohad. The EpicChain project aims to create a robust,
//  decentralized, and developer-friendly blockchain infrastructure that empowers innovation,
//  transparency, and digital sovereignty.
//
// ---------------------------------------------------------------------------------------------
//  Licensing:
//  This file is distributed under the permissive MIT License, which grants anyone the freedom
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of this
//  software. These rights are granted with the understanding that the original license notice
//  and copyright attribution remain intact.
//
//  For the full license text, please refer to the LICENSE file included in the root directory of
//  this repository or visit the official MIT License page at:
//  ➤ https://opensource.org/licenses/MIT
//
// ---------------------------------------------------------------------------------------------
//  Community and Contribution:
//  EpicChain Labs is deeply rooted in the principles of open-source development. We believe that
//  collaboration, transparency, and inclusiveness are the cornerstones of sustainable technology.
//
//  This file, like all components of the EpicChain ecosystem, is offered to the global development
//  community to explore, extend, and improve. Whether you're fixing bugs, optimizing performance,
//  or building new features, your contributions are welcome and appreciated.
//
//  By contributing to this project, you become part of a community dedicated to shaping the future
//  of blockchain technology. Join us in our mission to create more secure, scalable, and accessible
//  digital infrastructure for all.
//
// ---------------------------------------------------------------------------------------------
//  Terms of Use:
//  Redistribution and usage of this file in both source and compiled (binary) forms—with or without
//  modification—are fully permitted under the MIT License. Users of this software are expected to
//  adhere to the simple and clear guidelines established in the LICENSE file.
//
//  By using this file and other components of the EpicChain Labs project, you acknowledge and agree
//  to the terms of the MIT License. This ensures that the ethos of free and open software development
//  continues to flourish and remain protected.
//
// ---------------------------------------------------------------------------------------------
//  Final Note:
//  EpicChain Labs remains committed to pushing the boundaries of blockchain innovation. Whether
//  you're an experienced developer, a researcher, a student, or simply a curious enthusiast, we
//  invite you to explore the possibilities of EpicChain—and contribute toward a decentralized future.
//
//  Learn more about the project, get involved, or access full documentation at:
//  ➤ https://epic-chain.org
//
// =============================================================================================



using Microsoft.Extensions.Configuration;
using System.CommandLine;
using System.CommandLine.Invocation;
using System.CommandLine.NamingConventionBinder;
using System.Reflection;

namespace EpicChain.CLI
{
    public partial class MainService
    {
        public int OnStartWithCommandLine(string[] args)
        {
            RootCommand rootCommand = new(Assembly.GetExecutingAssembly().GetCustomAttribute<AssemblyTitleAttribute>()!.Title)
            {
                new Option<string>(new[] { "-c", "--config","/config" }, "Specifies the config file."),
                new Option<string>(new[] { "-w", "--wallet","/wallet" }, "The path of the epicchain wallet [*.json]."),
                new Option<string>(new[] { "-p", "--password" ,"/password" }, "Password to decrypt the wallet, either from the command line or config file."),
                new Option<string>(new[] { "--db-engine","/db-engine" }, "Specify the db engine."),
                new Option<string>(new[] { "--db-path","/db-path" }, "Specify the db path."),
                new Option<string>(new[] { "--noverify","/noverify" }, "Indicates whether the blocks need to be verified when importing."),
                new Option<string[]>(new[] { "--plugins","/plugins" }, "The list of plugins, if not present, will be installed [plugin1 plugin2]."),
            };

            rootCommand.Handler = CommandHandler.Create<RootCommand, CommandLineOptions, InvocationContext>(Handle);
            return rootCommand.Invoke(args);
        }

        private void Handle(RootCommand command, CommandLineOptions options, InvocationContext context)
        {
            Start(options);
        }

        private static void CustomProtocolSettings(CommandLineOptions options, ProtocolSettings settings)
        {
            var tempSetting = settings;
            // if specified config, then load the config and check the network
            if (!string.IsNullOrEmpty(options.Config))
            {
                tempSetting = ProtocolSettings.Load(options.Config);
            }

            var customSetting = new ProtocolSettings
            {
                Network = tempSetting.Network,
                AddressVersion = tempSetting.AddressVersion,
                StandbyCommittee = tempSetting.StandbyCommittee,
                ValidatorsCount = tempSetting.ValidatorsCount,
                SeedList = tempSetting.SeedList,
                MillisecondsPerBlock = tempSetting.MillisecondsPerBlock,
                MaxTransactionsPerBlock = tempSetting.MaxTransactionsPerBlock,
                MemoryPoolMaxTransactions = tempSetting.MemoryPoolMaxTransactions,
                MaxTraceableBlocks = tempSetting.MaxTraceableBlocks,
                InitialEpicPulseDistribution = tempSetting.InitialEpicPulseDistribution,
                Hardforks = tempSetting.Hardforks
            };

            if (!string.IsNullOrEmpty(options.Config)) ProtocolSettings.Custom = customSetting;
        }

        private static void CustomApplicationSettings(CommandLineOptions options, Settings settings)
        {
            var tempSetting = string.IsNullOrEmpty(options.Config) ? settings : new Settings(new ConfigurationBuilder().AddJsonFile(options.Config, optional: true).Build().GetSection("ApplicationConfiguration"));
            var customSetting = new Settings
            {
                Logger = tempSetting.Logger,
                Storage = new StorageSettings
                {
                    Engine = options.DBEngine ?? tempSetting.Storage.Engine,
                    Path = options.DBPath ?? tempSetting.Storage.Path
                },
                P2P = tempSetting.P2P,
                UnlockWallet = new UnlockWalletSettings
                {
                    Path = options.Wallet ?? tempSetting.UnlockWallet.Path,
                    Password = options.Password ?? tempSetting.UnlockWallet.Password
                },
                Contracts = tempSetting.Contracts
            };
            if (options.IsValid) Settings.Custom = customSetting;
        }
    }
}
