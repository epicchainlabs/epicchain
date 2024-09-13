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
using EpicChain.SmartContract.Native;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Net;

namespace EpicChain.Plugins.RpcServer
{
    class Settings : PluginSettings
    {
        public IReadOnlyList<RpcServerSettings> Servers { get; init; }

        public Settings(IConfigurationSection section) : base(section)
        {
            Servers = section.GetSection(nameof(Servers)).GetChildren().Select(p => RpcServerSettings.Load(p)).ToArray();
        }
    }

    public record RpcServerSettings
    {
        public uint Network { get; init; }
        public IPAddress BindAddress { get; init; }
        public ushort Port { get; init; }
        public string SslCert { get; init; }
        public string SslCertPassword { get; init; }
        public string[] TrustedAuthorities { get; init; }
        public int MaxConcurrentConnections { get; init; }
        public int MaxRequestBodySize { get; init; }
        public string RpcUser { get; init; }
        public string RpcPass { get; init; }
        public bool EnableCors { get; init; }
        public string[] AllowOrigins { get; init; }
        public int KeepAliveTimeout { get; init; }
        public uint RequestHeadersTimeout { get; init; }
        // In the unit of datoshi, 1 EpicPulse = 10^8 datoshi
        public long maxEpicPulseInvoke { get; init; }
        // In the unit of datoshi, 1 EpicPulse = 10^8 datoshi
        public long MaxFee { get; init; }
        public int MaxIteratorResultItems { get; init; }
        public int MaxStackSize { get; init; }
        public string[] DisabledMethods { get; init; }
        public bool SessionEnabled { get; init; }
        public TimeSpan SessionExpirationTime { get; init; }
        public int FindStoragePageSize { get; init; }

        public static RpcServerSettings Default { get; } = new RpcServerSettings
        {
            Network = 5195086u,
            BindAddress = IPAddress.None,
            SslCert = string.Empty,
            SslCertPassword = string.Empty,
            maxEpicPulseInvoke = (long)new BigDecimal(10M, NativeContract.EpicPulse.Decimals).Value,
            MaxFee = (long)new BigDecimal(0.1M, NativeContract.EpicPulse.Decimals).Value,
            TrustedAuthorities = Array.Empty<string>(),
            EnableCors = true,
            AllowOrigins = Array.Empty<string>(),
            KeepAliveTimeout = 60,
            RequestHeadersTimeout = 15,
            MaxIteratorResultItems = 100,
            MaxStackSize = ushort.MaxValue,
            DisabledMethods = Array.Empty<string>(),
            MaxConcurrentConnections = 40,
            MaxRequestBodySize = 5 * 1024 * 1024,
            SessionEnabled = false,
            SessionExpirationTime = TimeSpan.FromSeconds(60),
            FindStoragePageSize = 50
        };

        public static RpcServerSettings Load(IConfigurationSection section) => new()
        {
            Network = section.GetValue("Network", Default.Network),
            BindAddress = IPAddress.Parse(section.GetSection("BindAddress").Value),
            Port = ushort.Parse(section.GetSection("Port").Value),
            SslCert = section.GetSection("SslCert").Value,
            SslCertPassword = section.GetSection("SslCertPassword").Value,
            TrustedAuthorities = section.GetSection("TrustedAuthorities").GetChildren().Select(p => p.Get<string>()).ToArray(),
            RpcUser = section.GetSection("RpcUser").Value,
            RpcPass = section.GetSection("RpcPass").Value,
            EnableCors = section.GetValue(nameof(EnableCors), Default.EnableCors),
            AllowOrigins = section.GetSection(nameof(AllowOrigins)).GetChildren().Select(p => p.Get<string>()).ToArray(),
            KeepAliveTimeout = section.GetValue(nameof(KeepAliveTimeout), Default.KeepAliveTimeout),
            RequestHeadersTimeout = section.GetValue(nameof(RequestHeadersTimeout), Default.RequestHeadersTimeout),
            maxEpicPulseInvoke = (long)new BigDecimal(section.GetValue<decimal>("maxEpicPulseInvoke", Default.maxEpicPulseInvoke), NativeContract.EpicPulse.Decimals).Value,
            MaxFee = (long)new BigDecimal(section.GetValue<decimal>("MaxFee", Default.MaxFee), NativeContract.EpicPulse.Decimals).Value,
            MaxIteratorResultItems = section.GetValue("MaxIteratorResultItems", Default.MaxIteratorResultItems),
            MaxStackSize = section.GetValue("MaxStackSize", Default.MaxStackSize),
            DisabledMethods = section.GetSection("DisabledMethods").GetChildren().Select(p => p.Get<string>()).ToArray(),
            MaxConcurrentConnections = section.GetValue("MaxConcurrentConnections", Default.MaxConcurrentConnections),
            MaxRequestBodySize = section.GetValue("MaxRequestBodySize", Default.MaxRequestBodySize),
            SessionEnabled = section.GetValue("SessionEnabled", Default.SessionEnabled),
            SessionExpirationTime = TimeSpan.FromSeconds(section.GetValue("SessionExpirationTime", (int)Default.SessionExpirationTime.TotalSeconds)),
            FindStoragePageSize = section.GetValue("FindStoragePageSize", Default.FindStoragePageSize)
        };
    }
}
