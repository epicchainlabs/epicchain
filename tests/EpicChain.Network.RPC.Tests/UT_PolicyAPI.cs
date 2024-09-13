// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_PolicyAPI.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Microsoft.VisualStudio.TestTools.UnitTesting;
using Moq;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using EpicChain.Wallets;
using System.Numerics;
using System.Threading.Tasks;

namespace EpicChain.Network.RPC.Tests
{
    [TestClass]
    public class UT_PolicyAPI
    {
        Mock<RpcClient> rpcClientMock;
        KeyPair keyPair1;
        UInt160 sender;
        PolicyAPI policyAPI;

        [TestInitialize]
        public void TestSetup()
        {
            keyPair1 = new KeyPair(Wallet.GetPrivateKeyFromWIF("KyXwTh1hB76RRMquSvnxZrJzQx7h9nQP2PCRL38v6VDb5ip3nf1p"));
            sender = Contract.CreateSignatureRedeemScript(keyPair1.PublicKey).ToScriptHash();
            rpcClientMock = UT_TransactionManager.MockRpcClient(sender, new byte[0]);
            policyAPI = new PolicyAPI(rpcClientMock.Object);
        }

        [TestMethod]
        public async Task TestGetExecFeeFactor()
        {
            byte[] testScript = NativeContract.Policy.Hash.MakeScript("getExecFeeFactor");
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript, new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(30) });

            var result = await policyAPI.GetExecFeeFactorAsync();
            Assert.AreEqual(30u, result);
        }

        [TestMethod]
        public async Task TestGetStoragePrice()
        {
            byte[] testScript = NativeContract.Policy.Hash.MakeScript("getStoragePrice");
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript, new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(100000) });

            var result = await policyAPI.GetStoragePriceAsync();
            Assert.AreEqual(100000u, result);
        }

        [TestMethod]
        public async Task TestGetFeePerByte()
        {
            byte[] testScript = NativeContract.Policy.Hash.MakeScript("getFeePerByte");
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript, new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(1000) });

            var result = await policyAPI.GetFeePerByteAsync();
            Assert.AreEqual(1000L, result);
        }

        [TestMethod]
        public async Task TestIsBlocked()
        {
            byte[] testScript = NativeContract.Policy.Hash.MakeScript("isBlocked", UInt160.Zero);
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript, new ContractParameter { Type = ContractParameterType.Boolean, Value = true });
            var result = await policyAPI.IsBlockedAsync(UInt160.Zero);
            Assert.AreEqual(true, result);
        }
    }
}
