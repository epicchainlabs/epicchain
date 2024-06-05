// Copyright (C) 2021-2024 The EpicChain Labs.
//
// ContractMethodAttribute.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using System;
using System.Diagnostics;

namespace Neo.SmartContract.Native
{
    [DebuggerDisplay("{Name}")]
    [AttributeUsage(AttributeTargets.Method | AttributeTargets.Property, AllowMultiple = false)]
    internal class ContractMethodAttribute : Attribute
    {
        public string Name { get; init; }
        public CallFlags RequiredCallFlags { get; init; }
        public long CpuFee { get; init; }
        public long StorageFee { get; init; }
        public Hardfork? ActiveIn { get; init; } = null;
        public Hardfork? DeprecatedIn { get; init; } = null;

        public ContractMethodAttribute() { }

        public ContractMethodAttribute(Hardfork activeIn)
        {
            ActiveIn = activeIn;
        }

        public ContractMethodAttribute(bool isDeprecated, Hardfork deprecatedIn)
        {
            if (!isDeprecated) throw new ArgumentException("isDeprecated must be true", nameof(isDeprecated));
            DeprecatedIn = deprecatedIn;
        }
    }
}
