// Copyright (C) 2021-2024 The EpicChain Labs.
//
// Settings.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Microsoft.Extensions.Configuration;

namespace Neo.Plugins.StateService
{
    internal class Settings
    {
        public string Path { get; }
        public bool FullState { get; }
        public uint Network { get; }
        public bool AutoVerify { get; }
        public int MaxFindResultItems { get; }

        public static Settings Default { get; private set; }

        private Settings(IConfigurationSection section)
        {
            Path = section.GetValue("Path", "Data_MPT_{0}");
            FullState = section.GetValue("FullState", false);
            Network = section.GetValue("Network", 5195086u);
            AutoVerify = section.GetValue("AutoVerify", false);
            MaxFindResultItems = section.GetValue("MaxFindResultItems", 100);
        }

        public static void Load(IConfigurationSection section)
        {
            Default = new Settings(section);
        }
    }
}
