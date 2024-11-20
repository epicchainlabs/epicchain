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
using System;
using System.Linq;

namespace EpicChain.Plugins.OracleService
{
    class HttpsSettings
    {
        public TimeSpan Timeout { get; }

        public HttpsSettings(IConfigurationSection section)
        {
            Timeout = TimeSpan.FromMilliseconds(section.GetValue("Timeout", 5000));
        }
    }

    class NeoFSSettings
    {
        public string EndPoint { get; }
        public TimeSpan Timeout { get; }

        public NeoFSSettings(IConfigurationSection section)
        {
            EndPoint = section.GetValue("EndPoint", "127.0.0.1:8080");
            Timeout = TimeSpan.FromMilliseconds(section.GetValue("Timeout", 15000));
        }
    }

    class Settings : PluginSettings
    {
        public uint Network { get; }
        public Uri[] Nodes { get; }
        public TimeSpan MaxTaskTimeout { get; }
        public TimeSpan MaxOracleTimeout { get; }
        public bool AllowPrivateHost { get; }
        public string[] AllowedContentTypes { get; }
        public HttpsSettings Https { get; }
        public NeoFSSettings NeoFS { get; }
        public bool AutoStart { get; }

        public static Settings Default { get; private set; }

        private Settings(IConfigurationSection section) : base(section)
        {
            Network = section.GetValue("Network", 5195086u);
            Nodes = section.GetSection("Nodes").GetChildren().Select(p => new Uri(p.Get<string>(), UriKind.Absolute)).ToArray();
            MaxTaskTimeout = TimeSpan.FromMilliseconds(section.GetValue("MaxTaskTimeout", 432000000));
            MaxOracleTimeout = TimeSpan.FromMilliseconds(section.GetValue("MaxOracleTimeout", 15000));
            AllowPrivateHost = section.GetValue("AllowPrivateHost", false);
            AllowedContentTypes = section.GetSection("AllowedContentTypes").GetChildren().Select(p => p.Get<string>()).ToArray();
            Https = new HttpsSettings(section.GetSection("Https"));
            NeoFS = new NeoFSSettings(section.GetSection("NeoFS"));
            AutoStart = section.GetValue("AutoStart", false);
        }

        public static void Load(IConfigurationSection section)
        {
            Default = new Settings(section);
        }
    }
}
