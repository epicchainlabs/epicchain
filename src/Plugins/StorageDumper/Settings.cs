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
using Neo.SmartContract.Native;

namespace Neo.Plugins.StorageDumper
{
    internal class Settings
    {
        /// <summary>
        /// Amount of storages states (heights) to be dump in a given json file
        /// </summary>
        public uint BlockCacheSize { get; }
        /// <summary>
        /// Height to begin storage dump
        /// </summary>
        public uint HeightToBegin { get; }
        /// <summary>
        /// Default number of items per folder
        /// </summary>
        public uint StoragePerFolder { get; }
        public IReadOnlyList<int> Exclude { get; }

        public static Settings? Default { get; private set; }

        private Settings(IConfigurationSection section)
        {
            // Geting settings for storage changes state dumper
            BlockCacheSize = section.GetValue("BlockCacheSize", 1000u);
            HeightToBegin = section.GetValue("HeightToBegin", 0u);
            StoragePerFolder = section.GetValue("StoragePerFolder", 100000u);
            Exclude = section.GetSection("Exclude").Exists()
                ? section.GetSection("Exclude").GetChildren().Select(p => int.Parse(p.Value!)).ToArray()
                : new[] { NativeContract.Ledger.Id };
        }

        public static void Load(IConfigurationSection section)
        {
            Default = new Settings(section);
        }
    }
}
