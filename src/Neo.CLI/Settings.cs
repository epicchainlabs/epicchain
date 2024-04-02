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
// Settings.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Microsoft.Extensions.Configuration;
using Neo.Network.P2P;
using Neo.Persistence;
using System;
using System.Reflection;
using System.Threading;

namespace Neo
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
            Port = section.GetValue<ushort>(nameof(Port), 10111);
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
        public UInt160 NeoNameService { get; init; } = UInt160.Zero;

        public ContractsSettings(IConfigurationSection section)
        {
            if (section.Exists())
            {
                if (UInt160.TryParse(section.GetValue(nameof(NeoNameService), string.Empty), out var hash))
                {
                    NeoNameService = hash;
                }
                else
                    throw new ArgumentException("Neo Name Service (NNS): NeoNameService hash is invalid. Check your config.json.", nameof(NeoNameService));
            }
        }

        public ContractsSettings() { }
    }

    public class PluginsSettings
    {
        public Uri DownloadUrl { get; init; } = new("https://api.github.com/repos/neo-project/neo-modules/releases");
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
