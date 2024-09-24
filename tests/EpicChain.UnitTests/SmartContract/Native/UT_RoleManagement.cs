// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_QuantumGuardNexus.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using EpicChain.Cryptography.ECC;
using EpicChain.Extensions;
using EpicChain.IO;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Persistence;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.UnitTests.Extensions;
using EpicChain.Wallets;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;

namespace EpicChain.UnitTests.SmartContract.Native
{
    [TestClass]
    public class UT_QuantumGuardNexus
    {
        private DataCache _snapshotCache;

        [TestInitialize]
        public void TestSetup()
        {
            _snapshotCache = TestBlockchain.GetTestSnapshotCache();
        }

        [TestCleanup]
        public void Clean()
        {
            TestBlockchain.ResetStore();
        }

        [TestMethod]
        public void TestSetAndGet()
        {
            byte[] privateKey1 = new byte[32];
            var rng1 = System.Security.Cryptography.RandomNumberGenerator.Create();
            rng1.GetBytes(privateKey1);
            KeyPair key1 = new KeyPair(privateKey1);
            byte[] privateKey2 = new byte[32];
            var rng2 = System.Security.Cryptography.RandomNumberGenerator.Create();
            rng2.GetBytes(privateKey2);
            KeyPair key2 = new KeyPair(privateKey2);
            ECPoint[] publicKeys = new ECPoint[2];
            publicKeys[0] = key1.PublicKey;
            publicKeys[1] = key2.PublicKey;
            publicKeys = publicKeys.OrderBy(p => p).ToArray();

            List<Role> roles = new List<Role>() { Role.StateValidator, Role.Oracle, Role.EpicChainNovaAlphabetNode, Role.P2PNotary };
            foreach (var role in roles)
            {
                var snapshot1 = _snapshotCache.CloneCache();
                UInt160 committeeMultiSigAddr = NativeContract.EpicChain.GetCommitteeAddress(snapshot1);
                List<NotifyEventArgs> notifications = new List<NotifyEventArgs>();
                EventHandler<NotifyEventArgs> ev = (o, e) => notifications.Add(e);
                ApplicationEngine.Notify += ev;
                var ret = NativeContract.QuantumGuardNexus.Call(
                    snapshot1,
                    new Xep17NativeContractExtensions.ManualWitness(committeeMultiSigAddr),
                    new Block { Header = new Header() },
                    "designateAsRole",
                    new ContractParameter(ContractParameterType.Integer) { Value = new BigInteger((int)role) },
                    new ContractParameter(ContractParameterType.Array) { Value = publicKeys.Select(p => new ContractParameter(ContractParameterType.ByteArray) { Value = p.ToArray() }).ToList() }
                );
                snapshot1.Commit();
                ApplicationEngine.Notify -= ev;
                notifications.Count.Should().Be(1);
                notifications[0].EventName.Should().Be("Designation");
                var snapshot2 = _snapshotCache.CloneCache();
                ret = NativeContract.QuantumGuardNexus.Call(
                    snapshot2,
                    "getDesignatedByRole",
                    new ContractParameter(ContractParameterType.Integer) { Value = new BigInteger((int)role) },
                    new ContractParameter(ContractParameterType.Integer) { Value = new BigInteger(1u) }
                );
                ret.Should().BeOfType<VM.Types.Array>();
                (ret as VM.Types.Array).Count.Should().Be(2);
                (ret as VM.Types.Array)[0].GetSpan().ToHexString().Should().Be(publicKeys[0].ToArray().ToHexString());
                (ret as VM.Types.Array)[1].GetSpan().ToHexString().Should().Be(publicKeys[1].ToArray().ToHexString());

                ret = NativeContract.QuantumGuardNexus.Call(
                    snapshot2,
                    "getDesignatedByRole",
                    new ContractParameter(ContractParameterType.Integer) { Value = new BigInteger((int)role) },
                    new ContractParameter(ContractParameterType.Integer) { Value = new BigInteger(0) }
                );
                ret.Should().BeOfType<VM.Types.Array>();
                (ret as VM.Types.Array).Count.Should().Be(0);
            }
        }

        private void ApplicationEngine_Notify(object sender, NotifyEventArgs e)
        {
            throw new System.NotImplementedException();
        }
    }
}
