// Copyright (C) 2021-2024 The EpicChain Labs.
//
// Helper.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.SmartContract.Manifest;
using System;

namespace Neo.CLI
{
    internal static class Helper
    {
        public static bool IsYes(this string input)
        {
            if (input == null) return false;

            input = input.ToLowerInvariant();

            return input == "yes" || input == "y";
        }

        public static string ToBase64String(this byte[] input) => System.Convert.ToBase64String(input);

        public static void IsScriptValid(this ReadOnlyMemory<byte> script, ContractAbi abi)
        {
            try
            {
                SmartContract.Helper.Check(script.ToArray(), abi);
            }
            catch (Exception e)
            {
                throw new FormatException($"Bad Script or Manifest Format: {e.Message}");
            }
        }
    }
}
