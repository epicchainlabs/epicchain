// 
// Copyright (C) 2021-2024 EpicChain Lab's
// All rights reserved.
// 
// This file is part of the EpicChain project, developed by xmoohad.
// 
// This file is subject to the terms and conditions defined in the LICENSE file found in the top-level 
// directory of this distribution. Unauthorized copying, modification, or distribution of this file,
// via any medium, is strictly prohibited. Any use of this file without explicit permission from EpicChain Lab's
// is a violation of copyright law and will be prosecuted to the fullest extent possible.
// 
// This file is licensed under the MIT License; you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     https://opensource.org/licenses/MIT
// 
// Unless required by applicable law or agreed to in writing, software distributed under the License is distributed 
// on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for 
// the specific language governing permissions and limitations under the License.
// 
// For more information about EpicChain Lab's projects and innovations, visit our website at https://epic-chain.org
// or contact us at xmoohad@epic-chain.org.
// 
//

using Microsoft.Extensions.Configuration;
using System.CommandLine;
using System.CommandLine.Invocation;
using System.CommandLine.NamingConventionBinder;
using System.Reflection;

namespace Neo.CLI
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
                InitialGasDistribution = tempSetting.InitialGasDistribution,
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
