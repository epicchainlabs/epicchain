// Copyright (C) 2021-2024 EpicChain Labs.

//
// DeployedContract.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.SmartContract.Manifest;
using System;
using System.Linq;

namespace EpicChain.SmartContract
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
            ContractMethodDescriptor descriptor = contract.Manifest.Abi.GetMethod(ContractBasicMethod.Verify, ContractBasicMethod.VerifyPCount);
            if (descriptor is null) throw new NotSupportedException("The smart contract haven't got verify method.");

            ParameterList = descriptor.Parameters.Select(u => u.Type).ToArray();
        }
    }
}
