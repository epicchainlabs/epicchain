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
// UT_Helper.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.Cryptography.ECC;
using Neo.Network.P2P.Payloads;
using Neo.SmartContract;
using Neo.SmartContract.Native;
using Neo.VM;
using Neo.Wallets;
using System;
using static Neo.SmartContract.Helper;

namespace Neo.UnitTests.SmartContract
{
    [TestClass]
    public class UT_Helper
    {
        private KeyPair _key;

        [TestInitialize]
        public void Init()
        {
            var pk = new byte[32];
            new Random().NextBytes(pk);
            _key = new KeyPair(pk);
        }

        [TestMethod]
        public void TestGetContractHash()
        {
            var nef = new NefFile()
            {
                Compiler = "test",
                Source = string.Empty,
                Tokens = Array.Empty<MethodToken>(),
                Script = new byte[] { 1, 2, 3 }
            };
            nef.CheckSum = NefFile.ComputeChecksum(nef);

            Assert.AreEqual("0x9b9628e4f1611af90e761eea8cc21372380c74b6", Neo.SmartContract.Helper.GetContractHash(UInt160.Zero, nef.CheckSum, "").ToString());
            Assert.AreEqual("0x66eec404d86b918d084e62a29ac9990e3b6f4286", Neo.SmartContract.Helper.GetContractHash(UInt160.Parse("0xa400ff00ff00ff00ff00ff00ff00ff00ff00ff01"), nef.CheckSum, "").ToString());
        }

        [TestMethod]
        public void TestIsMultiSigContract()
        {
            var case1 = new byte[]
            {
                0, 2, 12, 33, 221, 221, 221, 221, 221, 221, 221, 221, 221, 221, 221, 221, 221, 221, 221, 221, 221, 221,
                221, 221, 221, 221, 221, 221, 221, 221, 221, 221, 221, 221, 221, 221, 221, 12, 33, 255, 255, 255, 255,
                255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                255, 255, 255, 255, 255, 255, 255, 255, 0,
            };
            Assert.IsFalse(IsMultiSigContract(case1));

            var case2 = new byte[]
            {
                18, 12, 33, 2, 111, 240, 59, 148, 146, 65, 206, 29, 173, 212, 53, 25, 230, 150, 14, 10, 133, 180, 26,
                105, 160, 92, 50, 129, 3, 170, 43, 206, 21, 148, 202, 22, 12, 33, 2, 111, 240, 59, 148, 146, 65, 206,
                29, 173, 212, 53, 25, 230, 150, 14, 10, 133, 180, 26, 105, 160, 92, 50, 129, 3, 170, 43, 206, 21, 148,
                202, 22, 18
            };
            Assert.IsFalse(IsMultiSigContract(case2));
        }

        [TestMethod]
        public void TestSignatureContractCost()
        {
            var contract = Contract.CreateSignatureContract(_key.PublicKey);

            var tx = TestUtils.CreateRandomHashTransaction();
            tx.Signers[0].Account = contract.ScriptHash;

            using ScriptBuilder invocationScript = new();
            invocationScript.EmitPush(Neo.Wallets.Helper.Sign(tx, _key, TestProtocolSettings.Default.Network));
            tx.Witnesses = new Witness[] { new Witness() { InvocationScript = invocationScript.ToArray(), VerificationScript = contract.Script } };

            using var engine = ApplicationEngine.Create(TriggerType.Verification, tx, null, null, TestProtocolSettings.Default);
            engine.LoadScript(contract.Script);
            engine.LoadScript(new Script(invocationScript.ToArray(), true), configureState: p => p.CallFlags = CallFlags.None);
            Assert.AreEqual(VMState.HALT, engine.Execute());
            Assert.IsTrue(engine.ResultStack.Pop().GetBoolean());

            Assert.AreEqual(Neo.SmartContract.Helper.SignatureContractCost() * PolicyContract.DefaultExecFeeFactor, engine.GasConsumed);
        }

        [TestMethod]
        public void TestMultiSignatureContractCost()
        {
            var contract = Contract.CreateMultiSigContract(1, new ECPoint[] { _key.PublicKey });

            var tx = TestUtils.CreateRandomHashTransaction();
            tx.Signers[0].Account = contract.ScriptHash;

            using ScriptBuilder invocationScript = new();
            invocationScript.EmitPush(Neo.Wallets.Helper.Sign(tx, _key, TestProtocolSettings.Default.Network));

            using var engine = ApplicationEngine.Create(TriggerType.Verification, tx, null, null, TestProtocolSettings.Default);
            engine.LoadScript(contract.Script);
            engine.LoadScript(new Script(invocationScript.ToArray(), true), configureState: p => p.CallFlags = CallFlags.None);
            Assert.AreEqual(VMState.HALT, engine.Execute());
            Assert.IsTrue(engine.ResultStack.Pop().GetBoolean());

            Assert.AreEqual(Neo.SmartContract.Helper.MultiSignatureContractCost(1, 1) * PolicyContract.DefaultExecFeeFactor, engine.GasConsumed);
        }
    }
}
