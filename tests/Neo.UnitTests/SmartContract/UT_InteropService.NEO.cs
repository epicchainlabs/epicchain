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
// UT_InteropService.NEO.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.Cryptography;
using Neo.Cryptography.ECC;
using Neo.IO;
using Neo.Network.P2P;
using Neo.Network.P2P.Payloads;
using Neo.SmartContract;
using Neo.SmartContract.Manifest;
using Neo.SmartContract.Native;
using Neo.UnitTests.Extensions;
using Neo.VM;
using Neo.Wallets;
using System;
using System.Linq;

namespace Neo.UnitTests.SmartContract
{
    public partial class UT_InteropService
    {
        [TestMethod]
        public void TestCheckSig()
        {
            var engine = GetEngine(true);
            var iv = engine.ScriptContainer;
            var message = iv.GetSignData(TestProtocolSettings.Default.Network);
            byte[] privateKey = { 0x01,0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01};
            var keyPair = new KeyPair(privateKey);
            var pubkey = keyPair.PublicKey;
            var signature = Crypto.Sign(message, privateKey);
            engine.CheckSig(pubkey.EncodePoint(false), signature).Should().BeTrue();
            Action action = () => engine.CheckSig(new byte[70], signature);
            action.Should().Throw<FormatException>();
        }

        [TestMethod]
        public void TestCrypto_CheckMultiSig()
        {
            var engine = GetEngine(true);
            var iv = engine.ScriptContainer;
            var message = iv.GetSignData(TestProtocolSettings.Default.Network);

            byte[] privkey1 = { 0x01,0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01};
            var key1 = new KeyPair(privkey1);
            var pubkey1 = key1.PublicKey;
            var signature1 = Crypto.Sign(message, privkey1);

            byte[] privkey2 = { 0x01,0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x02};
            var key2 = new KeyPair(privkey2);
            var pubkey2 = key2.PublicKey;
            var signature2 = Crypto.Sign(message, privkey2);

            var pubkeys = new[]
            {
                pubkey1.EncodePoint(false),
                pubkey2.EncodePoint(false)
            };
            var signatures = new[]
            {
                signature1,
                signature2
            };
            engine.CheckMultisig(pubkeys, signatures).Should().BeTrue();

            pubkeys = new byte[0][];
            Assert.ThrowsException<ArgumentException>(() => engine.CheckMultisig(pubkeys, signatures));

            pubkeys = new[]
            {
                pubkey1.EncodePoint(false),
                pubkey2.EncodePoint(false)
            };
            signatures = new byte[0][];
            Assert.ThrowsException<ArgumentException>(() => engine.CheckMultisig(pubkeys, signatures));

            pubkeys = new[]
            {
                pubkey1.EncodePoint(false),
                pubkey2.EncodePoint(false)
            };
            signatures = new[]
            {
                signature1,
                new byte[64]
            };
            engine.CheckMultisig(pubkeys, signatures).Should().BeFalse();

            pubkeys = new[]
            {
                pubkey1.EncodePoint(false),
                new byte[70]
            };
            signatures = new[]
            {
                signature1,
                signature2
            };
            Assert.ThrowsException<FormatException>(() => engine.CheckMultisig(pubkeys, signatures));
        }

        [TestMethod]
        public void TestContract_Create()
        {
            var snapshot = TestBlockchain.GetTestSnapshot();
            var nef = new NefFile()
            {
                Script = Enumerable.Repeat((byte)OpCode.RET, byte.MaxValue).ToArray(),
                Source = string.Empty,
                Compiler = "",
                Tokens = System.Array.Empty<MethodToken>()
            };
            nef.CheckSum = NefFile.ComputeChecksum(nef);
            var nefFile = nef.ToArray();
            var manifest = TestUtils.CreateDefaultManifest();
            Assert.ThrowsException<InvalidOperationException>(() => snapshot.DeployContract(null, nefFile, manifest.ToJson().ToByteArray(false)));
            Assert.ThrowsException<ArgumentException>(() => snapshot.DeployContract(UInt160.Zero, nefFile, new byte[ContractManifest.MaxLength + 1]));
            Assert.ThrowsException<InvalidOperationException>(() => snapshot.DeployContract(UInt160.Zero, nefFile, manifest.ToJson().ToByteArray(true), 10000000));

            var script_exceedMaxLength = new NefFile()
            {
                Script = new byte[ExecutionEngineLimits.Default.MaxItemSize - 50],
                Source = string.Empty,
                Compiler = "",
                Tokens = Array.Empty<MethodToken>()
            };
            script_exceedMaxLength.CheckSum = NefFile.ComputeChecksum(script_exceedMaxLength);

            Assert.ThrowsException<FormatException>(() => script_exceedMaxLength.ToArray().AsSerializable<NefFile>());
            Assert.ThrowsException<InvalidOperationException>(() => snapshot.DeployContract(UInt160.Zero, script_exceedMaxLength.ToArray(), manifest.ToJson().ToByteArray(true)));

            var script_zeroLength = System.Array.Empty<byte>();
            Assert.ThrowsException<ArgumentException>(() => snapshot.DeployContract(UInt160.Zero, script_zeroLength, manifest.ToJson().ToByteArray(true)));

            var manifest_zeroLength = System.Array.Empty<byte>();
            Assert.ThrowsException<ArgumentException>(() => snapshot.DeployContract(UInt160.Zero, nefFile, manifest_zeroLength));

            manifest = TestUtils.CreateDefaultManifest();
            var ret = snapshot.DeployContract(UInt160.Zero, nefFile, manifest.ToJson().ToByteArray(false));
            ret.Hash.ToString().Should().Be("0x7b37d4bd3d87f53825c3554bd1a617318235a685");
            Assert.ThrowsException<InvalidOperationException>(() => snapshot.DeployContract(UInt160.Zero, nefFile, manifest.ToJson().ToByteArray(false)));

            var state = TestUtils.GetContract();
            snapshot.AddContract(state.Hash, state);

            Assert.ThrowsException<InvalidOperationException>(() => snapshot.DeployContract(UInt160.Zero, nefFile, manifest.ToJson().ToByteArray(false)));
        }

        [TestMethod]
        public void TestContract_Update()
        {
            var snapshot = TestBlockchain.GetTestSnapshot();
            var nef = new NefFile()
            {
                Script = new[] { (byte)OpCode.RET },
                Source = string.Empty,
                Compiler = "",
                Tokens = Array.Empty<MethodToken>()
            };
            nef.CheckSum = NefFile.ComputeChecksum(nef);
            Assert.ThrowsException<InvalidOperationException>(() => snapshot.UpdateContract(null, nef.ToArray(), new byte[0]));

            var manifest = TestUtils.CreateDefaultManifest();
            byte[] privkey = { 0x01,0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01};
            var key = new KeyPair(privkey);
            var pubkey = key.PublicKey;
            var state = TestUtils.GetContract();
            var signature = Crypto.Sign(state.Hash.ToArray(), privkey);
            manifest.Groups = new ContractGroup[]
            {
                new()
                {
                    PubKey = pubkey,
                    Signature = signature
                }
            };

            var storageItem = new StorageItem
            {
                Value = new byte[] { 0x01 }
            };

            var storageKey = new StorageKey
            {
                Id = state.Id,
                Key = new byte[] { 0x01 }
            };
            snapshot.AddContract(state.Hash, state);
            snapshot.Add(storageKey, storageItem);
            state.UpdateCounter.Should().Be(0);
            snapshot.UpdateContract(state.Hash, nef.ToArray(), manifest.ToJson().ToByteArray(false));
            var ret = NativeContract.ContractManagement.GetContract(snapshot, state.Hash);
            snapshot.Find(BitConverter.GetBytes(state.Id)).ToList().Count().Should().Be(1);
            ret.UpdateCounter.Should().Be(1);
            ret.Id.Should().Be(state.Id);
            ret.Manifest.ToJson().ToString().Should().Be(manifest.ToJson().ToString());
            ret.Script.Span.ToHexString().Should().Be(nef.Script.Span.ToHexString().ToString());
        }

        [TestMethod]
        public void TestContract_Update_Invalid()
        {
            var nefFile = new NefFile()
            {
                Script = new byte[] { 0x01 },
                Source = string.Empty,
                Compiler = "",
                Tokens = System.Array.Empty<MethodToken>()
            };
            nefFile.CheckSum = NefFile.ComputeChecksum(nefFile);

            var snapshot = TestBlockchain.GetTestSnapshot();

            Assert.ThrowsException<InvalidOperationException>(() => snapshot.UpdateContract(null, null, new byte[] { 0x01 }));
            Assert.ThrowsException<InvalidOperationException>(() => snapshot.UpdateContract(null, nefFile.ToArray(), null));
            Assert.ThrowsException<ArgumentException>(() => snapshot.UpdateContract(null, null, null));

            nefFile = new NefFile()
            {
                Script = new byte[0],
                Source = string.Empty,
                Compiler = "",
                Tokens = System.Array.Empty<MethodToken>()
            };
            nefFile.CheckSum = NefFile.ComputeChecksum(nefFile);

            Assert.ThrowsException<InvalidOperationException>(() => snapshot.UpdateContract(null, nefFile.ToArray(), new byte[] { 0x01 }));
            Assert.ThrowsException<InvalidOperationException>(() => snapshot.UpdateContract(null, nefFile.ToArray(), new byte[0]));
        }

        [TestMethod]
        public void TestStorage_Find()
        {
            var snapshot = TestBlockchain.GetTestSnapshot();
            var state = TestUtils.GetContract();

            var storageItem = new StorageItem
            {
                Value = new byte[] { 0x01, 0x02, 0x03, 0x04 }
            };
            var storageKey = new StorageKey
            {
                Id = state.Id,
                Key = new byte[] { 0x01 }
            };
            snapshot.AddContract(state.Hash, state);
            snapshot.Add(storageKey, storageItem);
            var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshot);
            engine.LoadScript(new byte[] { 0x01 });

            var iterator = engine.Find(new StorageContext
            {
                Id = state.Id,
                IsReadOnly = false
            }, new byte[] { 0x01 }, FindOptions.ValuesOnly);
            iterator.Next();
            var ele = iterator.Value(null);
            ele.GetSpan().ToHexString().Should().Be(storageItem.Value.Span.ToHexString());
        }
    }
}
