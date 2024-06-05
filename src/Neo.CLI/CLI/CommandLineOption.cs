// Copyright (C) 2021-2024 The EpicChain Labs.
//
// CommandLineOption.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


namespace Neo.CLI
{
    public class CommandLineOptions
    {
        public string? Config { get; init; }
        public string? Wallet { get; init; }
        public string? Password { get; init; }
        public string[]? Plugins { get; set; }
        public string? DBEngine { get; init; }
        public string? DBPath { get; init; }
        public bool? NoVerify { get; init; }

        /// <summary>
        /// Check if CommandLineOptions was configured
        /// </summary>
        public bool IsValid =>
                !string.IsNullOrEmpty(Config) ||
                !string.IsNullOrEmpty(Wallet) ||
                !string.IsNullOrEmpty(Password) ||
                !string.IsNullOrEmpty(DBEngine) ||
                !string.IsNullOrEmpty(DBPath) ||
                (Plugins?.Length > 0) ||
                NoVerify is not null;
    }
}
