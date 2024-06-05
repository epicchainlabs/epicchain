// Copyright (C) 2021-2024 The EpicChain Labs.
//
// DeployedContract.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
using System.Linq;

namespace Neo.SmartContract
{
    /// <summary>
    /// Represents a deployed contract that can be invoked.
    /// </summary>
    public class DeployedContract : Contract
    {
        public override UInt160 ScriptHash { get; }

        /// <summary>
        /// Initializes a new instance of the <see cref="DeployedContract"/> class with the specified <see cref="ContractState"/>.
        /// </summary>
        /// <param name="contract">The <see cref="ContractState"/> corresponding to the contract.</param>
        public DeployedContract(ContractState contract)
        {
            if (contract is null) throw new ArgumentNullException(nameof(contract));

            Script = null;
            ScriptHash = contract.Hash;
            ContractMethodDescriptor descriptor = contract.Manifest.Abi.GetMethod("verify", -1);
            if (descriptor is null) throw new NotSupportedException("The smart contract haven't got verify method.");

            ParameterList = descriptor.Parameters.Select(u => u.Type).ToArray();
        }
    }
}
