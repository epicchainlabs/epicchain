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
// UT_Witness.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.IO;
using Neo.Json;
using Neo.Network.P2P.Payloads;
using Neo.SmartContract;
using Neo.Wallets;
using Neo.Wallets.NEP6;
using System;
using System.Linq;

namespace Neo.UnitTests.Network.P2P.Payloads
{
    [TestClass]
    public class UT_Witness
    {
        Witness uut;

        [TestInitialize]
        public void TestSetup()
        {
            uut = new Witness();
        }

        [TestMethod]
        public void InvocationScript_Get()
        {
            uut.InvocationScript.IsEmpty.Should().BeTrue();
        }

        private static Witness PrepareDummyWitness(int pubKeys, int m)
        {
            var address = new WalletAccount[pubKeys];
            var wallets = new NEP6Wallet[pubKeys];
            var snapshot = TestBlockchain.GetTestSnapshot();

            for (int x = 0; x < pubKeys; x++)
            {
                wallets[x] = TestUtils.GenerateTestWallet("123");
                address[x] = wallets[x].CreateAccount();
            }

            // Generate multisignature

            var multiSignContract = Contract.CreateMultiSigContract(m, address.Select(a => a.GetKey().PublicKey).ToArray());

            for (int x = 0; x < pubKeys; x++)
            {
                wallets[x].CreateAccount(multiSignContract, address[x].GetKey());
            }

            // Sign

            var data = new ContractParametersContext(snapshot, new Transaction()
            {
                Attributes = Array.Empty<TransactionAttribute>(),
                Signers = new[] {new Signer()
                {
                    Account = multiSignContract.ScriptHash,
                    Scopes = WitnessScope.CalledByEntry
                }},
                NetworkFee = 0,
                Nonce = 0,
                Script = Array.Empty<byte>(),
                SystemFee = 0,
                ValidUntilBlock = 0,
                Version = 0,
                Witnesses = Array.Empty<Witness>()
            }, TestProtocolSettings.Default.Network);

            for (int x = 0; x < m; x++)
            {
                Assert.IsTrue(wallets[x].Sign(data));
            }

            Assert.IsTrue(data.Completed);
            return data.GetWitnesses()[0];
        }

        [TestMethod]
        public void MaxSize_OK()
        {
            var witness = PrepareDummyWitness(10, 10);

            // Check max size

            witness.Size.Should().Be(1023);
            witness.InvocationScript.GetVarSize().Should().Be(663);
            witness.VerificationScript.GetVarSize().Should().Be(360);

            var copy = witness.ToArray().AsSerializable<Witness>();

            Assert.IsTrue(witness.InvocationScript.Span.SequenceEqual(copy.InvocationScript.Span));
            Assert.IsTrue(witness.VerificationScript.Span.SequenceEqual(copy.VerificationScript.Span));
        }

        [TestMethod]
        public void MaxSize_Error()
        {
            var witness = new Witness
            {
                InvocationScript = new byte[1025],
                VerificationScript = new byte[10]
            };

            // Check max size

            Assert.ThrowsException<FormatException>(() => witness.ToArray().AsSerializable<Witness>());

            // Check max size

            witness.InvocationScript = new byte[10];
            witness.VerificationScript = new byte[1025];
            Assert.ThrowsException<FormatException>(() => witness.ToArray().AsSerializable<Witness>());
        }

        [TestMethod]
        public void InvocationScript_Set()
        {
            byte[] dataArray = new byte[] { 0, 32, 32, 20, 32, 32 };
            uut.InvocationScript = dataArray;
            uut.InvocationScript.Length.Should().Be(6);
            Assert.AreEqual(uut.InvocationScript.Span.ToHexString(), "002020142020");
        }

        private static void SetupWitnessWithValues(Witness uut, int lenghtInvocation, int lengthVerification, out byte[] invocationScript, out byte[] verificationScript)
        {
            invocationScript = TestUtils.GetByteArray(lenghtInvocation, 0x20);
            verificationScript = TestUtils.GetByteArray(lengthVerification, 0x20);
            uut.InvocationScript = invocationScript;
            uut.VerificationScript = verificationScript;
        }

        [TestMethod]
        public void SizeWitness_Small_Arrary()
        {
            SetupWitnessWithValues(uut, 252, 253, out _, out _);

            uut.Size.Should().Be(509); // (1 + 252*1) + (1 + 2 + 253*1)
        }

        [TestMethod]
        public void SizeWitness_Large_Arrary()
        {
            SetupWitnessWithValues(uut, 65535, 65536, out _, out _);

            uut.Size.Should().Be(131079); // (1 + 2 + 65535*1) + (1 + 4 + 65536*1)
        }

        [TestMethod]
        public void ToJson()
        {
            SetupWitnessWithValues(uut, 2, 3, out _, out _);

            JObject json = uut.ToJson();
            Assert.IsTrue(json.ContainsProperty("invocation"));
            Assert.IsTrue(json.ContainsProperty("verification"));
            Assert.AreEqual(json["invocation"].AsString(), "ICA=");
            Assert.AreEqual(json["verification"].AsString(), "ICAg");
        }
    }
}
