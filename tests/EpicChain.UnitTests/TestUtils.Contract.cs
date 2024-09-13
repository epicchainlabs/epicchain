// Copyright (C) 2021-2024 EpicChain Labs.

//
// TestUtils.Contract.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.SmartContract;
using EpicChain.SmartContract.Manifest;
using System;
using System.Linq;

namespace EpicChain.UnitTests;

partial class TestUtils
{
    public static ContractManifest CreateDefaultManifest()
    {
        return new ContractManifest
        {
            Name = "testManifest",
            Groups = [],
            SupportedStandards = [],
            Abi = new ContractAbi
            {
                Events = [],
                Methods =
                [
                    new ContractMethodDescriptor
                    {
                        Name = "testMethod",
                        Parameters = [],
                        ReturnType = ContractParameterType.Void,
                        Offset = 0,
                        Safe = true
                    }
                ]
            },
            Permissions = [ContractPermission.DefaultPermission],
            Trusts = WildcardContainer<ContractPermissionDescriptor>.Create(),
            Extra = null
        };
    }

    public static ContractManifest CreateManifest(string method, ContractParameterType returnType, params ContractParameterType[] parameterTypes)
    {
        var manifest = CreateDefaultManifest();
        manifest.Abi.Methods =
        [
            new ContractMethodDescriptor()
            {
                Name = method,
                Parameters = parameterTypes.Select((p, i) => new ContractParameterDefinition
                {
                    Name = $"p{i}",
                    Type = p
                }).ToArray(),
                ReturnType = returnType
            }
        ];
        return manifest;
    }

    public static ContractState GetContract(string method = "test", int parametersCount = 0)
    {
        NefFile nef = new()
        {
            Compiler = "",
            Source = "",
            Tokens = [],
            Script = new byte[] { 0x01, 0x01, 0x01, 0x01 }
        };
        nef.CheckSum = NefFile.ComputeChecksum(nef);
        return new ContractState
        {
            Id = 0x43000000,
            Nef = nef,
            Hash = nef.Script.Span.ToScriptHash(),
            Manifest = CreateManifest(method, ContractParameterType.Any, Enumerable.Repeat(ContractParameterType.Any, parametersCount).ToArray())
        };
    }

    internal static ContractState GetContract(byte[] script, ContractManifest manifest = null)
    {
        NefFile nef = new()
        {
            Compiler = "",
            Source = "",
            Tokens = [],
            Script = script
        };
        nef.CheckSum = NefFile.ComputeChecksum(nef);
        return new ContractState
        {
            Id = 1,
            Hash = script.ToScriptHash(),
            Nef = nef,
            Manifest = manifest ?? CreateDefaultManifest()
        };
    }
}
