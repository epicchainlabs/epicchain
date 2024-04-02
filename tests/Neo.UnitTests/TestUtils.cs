// EpicChain Copyright Project (2021-2024)
// 
// Copyright (c) 2021-2024 EpicChain
// 
// EpicChain is an innovative blockchain network developed and maintained by xmoohad. This copyright project outlines the rights and responsibilities associated with the EpicChain software and its related components.
// 
// 1. Copyright Holder:
//    - xmoohad
// 
// 2. Project Name:
//    - EpicChain
// 
// 3. Project Description:
//    - EpicChain is a decentralized blockchain network that aims to revolutionize the way digital assets are managed, traded, and secured. With its innovative features and robust architecture, EpicChain provides a secure and efficient platform for various decentralized applications (dApps) and digital asset management.
// 
// 4. Copyright Period:
//    - The copyright for the EpicChain software and its related components is valid from 2021 to 2024.
// 
// 5. Copyright Statement:
//    - All rights reserved. No part of the EpicChain software or its related components may be reproduced, distributed, or transmitted in any form or by any means, without the prior written permission of the copyright holder, except in the case of brief quotations embodied in critical reviews and certain other noncommercial uses permitted by copyright law.
// 
// 6. License:
//    - The EpicChain software is licensed under the EpicChain Software License, a custom license that governs the use, distribution, and modification of the software. The EpicChain Software License is designed to promote the free and open development of the EpicChain network while protecting the interests of the copyright holder.
// 
// 7. Open Source:
//    - EpicChain is an open-source project, and its source code is available to the public under the terms of the EpicChain Software License. Developers are encouraged to contribute to the development of EpicChain and create innovative applications on top of the EpicChain network.
// 
// 8. Disclaimer:
//    - The EpicChain software and its related components are provided "as is," without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall the copyright holder or contributors be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the EpicChain software or its related components.
// 
// 9. Contact Information:
//    - For inquiries regarding the EpicChain copyright project, please contact xmoohad at [email address].
// 
// 10. Updates:
//     - This copyright project may be updated or modified from time to time to reflect changes in the EpicChain project or to address new legal or regulatory requirements. Users and developers are encouraged to check the latest version of the copyright project periodically.

//
// TestUtils.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using FluentAssertions;
using Neo.Cryptography;
using Neo.Cryptography.ECC;
using Neo.IO;
using Neo.Json;
using Neo.Network.P2P.Payloads;
using Neo.SmartContract;
using Neo.SmartContract.Manifest;
using Neo.SmartContract.Native;
using Neo.VM;
using Neo.Wallets.NEP6;
using System;
using System.Linq;

namespace Neo.UnitTests
{
    public static class TestUtils
    {
        public static readonly Random TestRandom = new Random(1337); // use fixed seed for guaranteed determinism

        public static ContractManifest CreateDefaultManifest()
        {
            return new ContractManifest()
            {
                Name = "testManifest",
                Groups = new ContractGroup[0],
                SupportedStandards = Array.Empty<string>(),
                Abi = new ContractAbi()
                {
                    Events = new ContractEventDescriptor[0],
                    Methods = new[]
                    {
                        new ContractMethodDescriptor
                        {
                            Name = "testMethod",
                            Parameters = new ContractParameterDefinition[0],
                            ReturnType = ContractParameterType.Void,
                            Offset = 0,
                            Safe = true
                        }
                    }
                },
                Permissions = new[] { ContractPermission.DefaultPermission },
                Trusts = WildcardContainer<ContractPermissionDescriptor>.Create(),
                Extra = null
            };
        }

        public static ContractManifest CreateManifest(string method, ContractParameterType returnType, params ContractParameterType[] parameterTypes)
        {
            ContractManifest manifest = CreateDefaultManifest();
            manifest.Abi.Methods = new ContractMethodDescriptor[]
            {
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
            };
            return manifest;
        }

        public static StorageKey CreateStorageKey(this NativeContract contract, byte prefix, ISerializable key = null)
        {
            var k = new KeyBuilder(contract.Id, prefix);
            if (key != null) k = k.Add(key);
            return k;
        }

        public static StorageKey CreateStorageKey(this NativeContract contract, byte prefix, uint value)
        {
            return new KeyBuilder(contract.Id, prefix).AddBigEndian(value);
        }

        public static byte[] GetByteArray(int length, byte firstByte)
        {
            byte[] array = new byte[length];
            array[0] = firstByte;
            for (int i = 1; i < length; i++)
            {
                array[i] = 0x20;
            }
            return array;
        }

        public static NEP6Wallet GenerateTestWallet(string password)
        {
            JObject wallet = new JObject();
            wallet["name"] = "noname";
            wallet["version"] = new Version("1.0").ToString();
            wallet["scrypt"] = new ScryptParameters(2, 1, 1).ToJson();
            wallet["accounts"] = new JArray();
            wallet["extra"] = null;
            wallet.ToString().Should().Be("{\"name\":\"noname\",\"version\":\"1.0\",\"scrypt\":{\"n\":2,\"r\":1,\"p\":1},\"accounts\":[],\"extra\":null}");
            return new NEP6Wallet(null, password, TestProtocolSettings.Default, wallet);
        }

        public static Transaction GetTransaction(UInt160 sender)
        {
            return new Transaction
            {
                Script = new byte[] { (byte)OpCode.PUSH2 },
                Attributes = Array.Empty<TransactionAttribute>(),
                Signers = new[]{ new Signer()
                {
                    Account = sender,
                    Scopes = WitnessScope.CalledByEntry,
                    AllowedContracts = Array.Empty<UInt160>(),
                    AllowedGroups = Array.Empty<ECPoint>(),
                    Rules = Array.Empty<WitnessRule>(),
                } },
                Witnesses = new Witness[]{ new Witness
                {
                    InvocationScript = Array.Empty<byte>(),
                    VerificationScript = Array.Empty<byte>()
                } }
            };
        }

        internal static ContractState GetContract(string method = "test", int parametersCount = 0)
        {
            NefFile nef = new()
            {
                Compiler = "",
                Source = "",
                Tokens = Array.Empty<MethodToken>(),
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
                Tokens = Array.Empty<MethodToken>(),
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

        internal static StorageItem GetStorageItem(byte[] value)
        {
            return new StorageItem
            {
                Value = value
            };
        }

        internal static StorageKey GetStorageKey(int id, byte[] keyValue)
        {
            return new StorageKey
            {
                Id = id,
                Key = keyValue
            };
        }

        /// <summary>
        /// Test Util function SetupHeaderWithValues
        /// </summary>
        /// <param name="header">The header to be assigned</param>
        /// <param name="val256">PrevHash</param>
        /// <param name="merkRootVal">MerkleRoot</param>
        /// <param name="val160">NextConsensus</param>
        /// <param name="timestampVal">Timestamp</param>
        /// <param name="indexVal">Index</param>
        /// <param name="nonceVal">Nonce</param>
        /// <param name="scriptVal">Witness</param>
        public static void SetupHeaderWithValues(Header header, UInt256 val256, out UInt256 merkRootVal, out UInt160 val160, out ulong timestampVal, out ulong nonceVal, out uint indexVal, out Witness scriptVal)
        {
            header.PrevHash = val256;
            header.MerkleRoot = merkRootVal = UInt256.Parse("0x6226416a0e5aca42b5566f5a19ab467692688ba9d47986f6981a7f747bba2772");
            header.Timestamp = timestampVal = new DateTime(1980, 06, 01, 0, 0, 1, 001, DateTimeKind.Utc).ToTimestampMS(); // GMT: Sunday, June 1, 1980 12:00:01.001 AM
            header.Index = indexVal = 0;
            header.Nonce = nonceVal = 0;
            header.NextConsensus = val160 = UInt160.Zero;
            header.Witness = scriptVal = new Witness
            {
                InvocationScript = new byte[0],
                VerificationScript = new[] { (byte)OpCode.PUSH1 }
            };
        }

        public static void SetupBlockWithValues(Block block, UInt256 val256, out UInt256 merkRootVal, out UInt160 val160, out ulong timestampVal, out ulong nonceVal, out uint indexVal, out Witness scriptVal, out Transaction[] transactionsVal, int numberOfTransactions)
        {
            Header header = new Header();
            SetupHeaderWithValues(header, val256, out merkRootVal, out val160, out timestampVal, out nonceVal, out indexVal, out scriptVal);

            transactionsVal = new Transaction[numberOfTransactions];
            if (numberOfTransactions > 0)
            {
                for (int i = 0; i < numberOfTransactions; i++)
                {
                    transactionsVal[i] = GetTransaction(UInt160.Zero);
                }
            }

            block.Header = header;
            block.Transactions = transactionsVal;

            header.MerkleRoot = merkRootVal = MerkleTree.ComputeRoot(block.Transactions.Select(p => p.Hash).ToArray());
        }

        public static Transaction CreateRandomHashTransaction()
        {
            var randomBytes = new byte[16];
            TestRandom.NextBytes(randomBytes);
            return new Transaction
            {
                Script = randomBytes,
                Attributes = Array.Empty<TransactionAttribute>(),
                Signers = new Signer[] { new Signer() { Account = UInt160.Zero } },
                Witnesses = new[]
                {
                    new Witness
                    {
                        InvocationScript = new byte[0],
                        VerificationScript = new byte[0]
                    }
                }
            };
        }

        public static T CopyMsgBySerialization<T>(T serializableObj, T newObj) where T : ISerializable
        {
            MemoryReader reader = new(serializableObj.ToArray());
            newObj.Deserialize(ref reader);
            return newObj;
        }

        public static bool EqualsTo(this StorageItem item, StorageItem other)
        {
            return item.Value.Span.SequenceEqual(other.Value.Span);
        }
    }
}
