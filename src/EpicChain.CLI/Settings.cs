// Copyright (C) 2021-2024 EpicChain Labs.

//
// Settings.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Network.P2P;
using EpicChain.Persistence;
using System;
using System.Linq;
using System.Reflection;
using System.Threading;

namespace EpicChain
{
    public class Settings
    {
        public LoggerSettings Logger { get; init; }
        public StorageSettings Storage { get; init; }
        public P2PSettings P2P { get; init; }
        public UnlockWalletSettings UnlockWallet { get; init; }
        public ContractsSettings Contracts { get; init; }
        public PluginsSettings Plugins { get; init; }

        static Settings? s_default;

        static bool UpdateDefault(IConfiguration configuration)
        {
            var settings = new Settings(configuration.GetSection("ApplicationConfiguration"));
            return null == Interlocked.CompareExchange(ref s_default, settings, null);
        }

        public static bool Initialize(IConfiguration configuration)
        {
            return UpdateDefault(configuration);
        }

        public static Settings Default
        {
            get
            {
                if (s_default == null)
                {
                    var config = new ConfigurationBuilder().AddJsonFile("config.json", optional: true).Build();
                    Initialize(config);
                }
                return Custom ?? s_default!;
            }
        }

        public static Settings? Custom { get; set; }

        public Settings(IConfigurationSection section)
        {
            Contracts = new(section.GetSection(nameof(Contracts)));
            Logger = new(section.GetSection(nameof(Logger)));
            Storage = new(section.GetSection(nameof(Storage)));
            P2P = new(section.GetSection(nameof(P2P)));
            UnlockWallet = new(section.GetSection(nameof(UnlockWallet)));
            Plugins = new(section.GetSection(nameof(Plugins)));
        }

        public Settings()
        {
            Logger = new LoggerSettings();
            Storage = new StorageSettings();
            P2P = new P2PSettings();
            UnlockWallet = new UnlockWalletSettings();
            Contracts = new ContractsSettings();
            Plugins = new PluginsSettings();
        }
    }

    public class LoggerSettings
    {
        public string Path { get; init; } = string.Empty;
        public bool ConsoleOutput { get; init; }
        public bool Active { get; init; }

        public LoggerSettings(IConfigurationSection section)
        {
            Path = section.GetValue(nameof(Path), "Logs")!;
            ConsoleOutput = section.GetValue(nameof(ConsoleOutput), false);
            Active = section.GetValue(nameof(Active), false);
        }

        public LoggerSettings() { }
    }

    public class StorageSettings
    {
        public string Engine { get; init; } = nameof(MemoryStore);
        public string Path { get; init; } = string.Empty;

        public StorageSettings(IConfigurationSection section)
        {
            Engine = section.GetValue(nameof(Engine), nameof(MemoryStore))!;
            Path = section.GetValue(nameof(Path), string.Empty)!;
        }

        public StorageSettings() { }
    }

    public class P2PSettings
    {
        public ushort Port { get; }
        public int MinDesiredConnections { get; }
        public int MaxConnections { get; }
        public int MaxConnectionsPerAddress { get; }

        public P2PSettings(IConfigurationSection section)
        {
            Port = section.GetValue<ushort>(nameof(Port), 10333);
            MinDesiredConnections = section.GetValue(nameof(MinDesiredConnections), Peer.DefaultMinDesiredConnections);
            MaxConnections = section.GetValue(nameof(MaxConnections), Peer.DefaultMaxConnections);
            MaxConnectionsPerAddress = section.GetValue(nameof(MaxConnectionsPerAddress), 3);
        }

        public P2PSettings() { }
    }

    public class UnlockWalletSettings
    {
        public string? Path { get; init; } = string.Empty;
        public string? Password { get; init; } = string.Empty;
        public bool IsActive { get; init; } = false;

        public UnlockWalletSettings(IConfigurationSection section)
        {
            if (section.Exists())
            {
                Path = section.GetValue(nameof(Path), string.Empty)!;
                Password = section.GetValue(nameof(Password), string.Empty)!;
                IsActive = section.GetValue(nameof(IsActive), false);
            }
        }

        public UnlockWalletSettings() { }
    }

    public class ContractsSettings
    {
        public UInt160 EpicChainNameService { get; init; } = UInt160.Zero;

        public ContractsSettings(IConfigurationSection section)
        {
            if (section.Exists())
            {
                if (UInt160.TryParse(section.GetValue(nameof(EpicChainNameService), string.Empty), out var hash))
                {
                    EpicChainNameService = hash;
                }
                else
                    throw new ArgumentException("EpicChain Name Service (NNS): EpicChainNameService hash is invalid. Check your config.json.", nameof(EpicChainNameService));
            }
        }

        public ContractsSettings() { }
    }

    public class PluginsSettings
    {
        public Uri DownloadUrl { get; init; } = new("https://api.github.com/repos/epicchainlabs/epicchain/releases");
        public bool Prerelease { get; init; } = false;
        public Version Version { get; init; } = Assembly.GetExecutingAssembly().GetName().Version!;

        public PluginsSettings(IConfigurationSection section)
        {
            if (section.Exists())
            {
                DownloadUrl = section.GetValue(nameof(DownloadUrl), DownloadUrl)!;
#if DEBUG
                Prerelease = section.GetValue(nameof(Prerelease), Prerelease);
                Version = section.GetValue(nameof(Version), Version)!;
#endif
            }
        }

        public PluginsSettings() { }
    }
}
