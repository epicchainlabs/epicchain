// Copyright (C) 2021-2024 EpicChain Labs.

//
// MainService.CommandLine.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
                new Option<string>(new[] { "-w", "--wallet","/wallet" }, "The path of the neo3 wallet [*.json]."),
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
