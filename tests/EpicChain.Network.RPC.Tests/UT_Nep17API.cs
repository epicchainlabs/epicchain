// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_Xep17API.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Extensions;
using EpicChain.Json;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using EpicChain.Wallets;
using System.Linq;
using System.Numerics;
using System.Threading.Tasks;

namespace EpicChain.Network.RPC.Tests
{
    [TestClass]
    public class UT_Xep17API
    {
        Mock<RpcClient> rpcClientMock;
        KeyPair keyPair1;
        UInt160 sender;
        Xep17API Xep17API;

        [TestInitialize]
        public void TestSetup()
        {
            keyPair1 = new KeyPair(Wallet.GetPrivateKeyFromWIF("KyXwTh1hB76RRMquSvnxZrJzQx7h9nQP2PCRL38v6VDb5ip3nf1p"));
            sender = Contract.CreateSignatureRedeemScript(keyPair1.PublicKey).ToScriptHash();
            rpcClientMock = UT_TransactionManager.MockRpcClient(sender, new byte[0]);
            Xep17API = new Xep17API(rpcClientMock.Object);
        }

        [TestMethod]
        public async Task TestBalanceOf()
        {
            byte[] testScript = NativeContract.EpicPulse.Hash.MakeScript("balanceOf", UInt160.Zero);
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript, new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(10000) });

            var balance = await Xep17API.BalanceOfAsync(NativeContract.EpicPulse.Hash, UInt160.Zero);
            Assert.AreEqual(10000, (int)balance);
        }

        [TestMethod]
        public async Task TestGetSymbol()
        {
            byte[] testScript = NativeContract.EpicPulse.Hash.MakeScript("symbol");
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript, new ContractParameter { Type = ContractParameterType.String, Value = NativeContract.EpicPulse.Symbol });

            var result = await Xep17API.SymbolAsync(NativeContract.EpicPulse.Hash);
            Assert.AreEqual(NativeContract.EpicPulse.Symbol, result);
        }

        [TestMethod]
        public async Task TestGetDecimals()
        {
            byte[] testScript = NativeContract.EpicPulse.Hash.MakeScript("decimals");
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript, new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(NativeContract.EpicPulse.Decimals) });

            var result = await Xep17API.DecimalsAsync(NativeContract.EpicPulse.Hash);
            Assert.AreEqual(NativeContract.EpicPulse.Decimals, result);
        }

        [TestMethod]
        public async Task TestGetTotalSupply()
        {
            byte[] testScript = NativeContract.EpicPulse.Hash.MakeScript("totalSupply");
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript, new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(1_00000000) });

            var result = await Xep17API.TotalSupplyAsync(NativeContract.EpicPulse.Hash);
            Assert.AreEqual(1_00000000, (int)result);
        }

        [TestMethod]
        public async Task TestGetTokenInfo()
        {
            UInt160 scriptHash = NativeContract.EpicPulse.Hash;
            byte[] testScript = [
                .. scriptHash.MakeScript("symbol"),
                .. scriptHash.MakeScript("decimals"),
                .. scriptHash.MakeScript("totalSupply")];
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript,
                new ContractParameter { Type = ContractParameterType.String, Value = NativeContract.EpicPulse.Symbol },
                new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(NativeContract.EpicPulse.Decimals) },
                new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(1_00000000) });

            scriptHash = NativeContract.EpicChain.Hash;
            testScript = [
                .. scriptHash.MakeScript("symbol"),
                .. scriptHash.MakeScript("decimals"),
                .. scriptHash.MakeScript("totalSupply")];
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript,
                new ContractParameter { Type = ContractParameterType.String, Value = NativeContract.EpicChain.Symbol },
                new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(NativeContract.EpicChain.Decimals) },
                new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(1_00000000) });

            var tests = TestUtils.RpcTestCases.Where(p => p.Name == "getcontractstateasync");
            var haveEpicPulseUT = false;
            var haveEpicChainUT = false;
            foreach (var test in tests)
            {
                rpcClientMock.Setup(p => p.RpcSendAsync("getcontractstate", It.Is<JToken[]>(u => true)))
                .ReturnsAsync(test.Response.Result)
                .Verifiable();
                if (test.Request.Params[0].AsString() == NativeContract.EpicPulse.Hash.ToString() || test.Request.Params[0].AsString().Equals(NativeContract.EpicPulse.Name, System.StringComparison.OrdinalIgnoreCase))
                {
                    var result = await Xep17API.GetTokenInfoAsync(NativeContract.EpicPulse.Name.ToLower());
                    Assert.AreEqual(NativeContract.EpicPulse.Symbol, result.Symbol);
                    Assert.AreEqual(8, result.Decimals);
                    Assert.AreEqual(1_00000000, (int)result.TotalSupply);
                    Assert.AreEqual("EpicPulse", result.Name);

                    result = await Xep17API.GetTokenInfoAsync(NativeContract.EpicPulse.Hash);
                    Assert.AreEqual(NativeContract.EpicPulse.Symbol, result.Symbol);
                    Assert.AreEqual(8, result.Decimals);
                    Assert.AreEqual(1_00000000, (int)result.TotalSupply);
                    Assert.AreEqual("EpicPulse", result.Name);
                    haveEpicPulseUT = true;
                }
                else if (test.Request.Params[0].AsString() == NativeContract.EpicChain.Hash.ToString() || test.Request.Params[0].AsString().Equals(NativeContract.EpicChain.Name, System.StringComparison.OrdinalIgnoreCase))
                {
                    var result = await Xep17API.GetTokenInfoAsync(NativeContract.EpicChain.Name.ToLower());
                    Assert.AreEqual(NativeContract.EpicChain.Symbol, result.Symbol);
                    Assert.AreEqual(0, result.Decimals);
                    Assert.AreEqual(1_00000000, (int)result.TotalSupply);
                    Assert.AreEqual("EpicChain", result.Name);

                    result = await Xep17API.GetTokenInfoAsync(NativeContract.EpicChain.Hash);
                    Assert.AreEqual(NativeContract.EpicChain.Symbol, result.Symbol);
                    Assert.AreEqual(0, result.Decimals);
                    Assert.AreEqual(1_00000000, (int)result.TotalSupply);
                    Assert.AreEqual("EpicChain", result.Name);
                    haveEpicChainUT = true;
                }
            }
            Assert.IsTrue(haveEpicPulseUT && haveEpicChainUT); //Update RpcTestCases.json
        }

        [TestMethod]
        public async Task TestTransfer()
        {
            byte[] testScript = NativeContract.EpicPulse.Hash.MakeScript("transfer", sender, UInt160.Zero, new BigInteger(1_00000000), null)
                .Concat(new[] { (byte)OpCode.ASSERT })
                .ToArray();
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript, new ContractParameter());

            var client = rpcClientMock.Object;
            var result = await Xep17API.CreateTransferTxAsync(NativeContract.EpicPulse.Hash, keyPair1, UInt160.Zero, new BigInteger(1_00000000), null, true);

            testScript = NativeContract.EpicPulse.Hash.MakeScript("transfer", sender, UInt160.Zero, new BigInteger(1_00000000), string.Empty)
                .Concat(new[] { (byte)OpCode.ASSERT })
                .ToArray();
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript, new ContractParameter());

            result = await Xep17API.CreateTransferTxAsync(NativeContract.EpicPulse.Hash, keyPair1, UInt160.Zero, new BigInteger(1_00000000), string.Empty, true);
            Assert.IsNotNull(result);
        }
    }
}
