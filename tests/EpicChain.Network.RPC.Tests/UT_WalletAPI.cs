// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_WalletAPI.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Cryptography.ECC;
using EpicChain.Extensions;
using EpicChain.Json;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Network.RPC.Models;
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
    public class UT_WalletAPI
    {
        Mock<RpcClient> rpcClientMock;
        KeyPair keyPair1;
        string address1;
        UInt160 sender;
        WalletAPI walletAPI;
        UInt160 multiSender;
        RpcClient client;

        [TestInitialize]
        public void TestSetup()
        {
            keyPair1 = new KeyPair(Wallet.GetPrivateKeyFromWIF("KyXwTh1hB76RRMquSvnxZrJzQx7h9nQP2PCRL38v6VDb5ip3nf1p"));
            sender = Contract.CreateSignatureRedeemScript(keyPair1.PublicKey).ToScriptHash();
            multiSender = Contract.CreateMultiSigContract(1, new ECPoint[] { keyPair1.PublicKey }).ScriptHash;
            rpcClientMock = UT_TransactionManager.MockRpcClient(sender, new byte[0]);
            client = rpcClientMock.Object;
            address1 = Wallets.Helper.ToAddress(sender, client.protocolSettings.AddressVersion);
            walletAPI = new WalletAPI(rpcClientMock.Object);
        }

        [TestMethod]
        public async Task TestGetUnclaimedEpicPulse()
        {
            byte[] testScript = NativeContract.EpicChain.Hash.MakeScript("UnclaimedEpicPulse", sender, 99);
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript, new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(1_10000000) });

            var balance = await walletAPI.GetUnclaimedEpicPulseAsync(address1);
            Assert.AreEqual(1.1m, balance);
        }

        [TestMethod]
        public async Task TestGetEpicChainBalance()
        {
            byte[] testScript = NativeContract.EpicChain.Hash.MakeScript("balanceOf", sender);
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript, new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(1_00000000) });

            var balance = await walletAPI.GetEpicChainBalanceAsync(address1);
            Assert.AreEqual(1_00000000u, balance);
        }

        [TestMethod]
        public async Task TestGetepicpulseBalance()
        {
            byte[] testScript = NativeContract.EpicPulse.Hash.MakeScript("balanceOf", sender);
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript, new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(1_10000000) });

            var balance = await walletAPI.GetEpicPulseBalanceAsync(address1);
            Assert.AreEqual(1.1m, balance);
        }

        [TestMethod]
        public async Task TestGetTokenBalance()
        {
            byte[] testScript = UInt160.Zero.MakeScript("balanceOf", sender);
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript, new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(1_10000000) });

            var balance = await walletAPI.GetTokenBalanceAsync(UInt160.Zero.ToString(), address1);
            Assert.AreEqual(1_10000000, balance);
        }

        [TestMethod]
        public async Task TestClaimEpicPulse()
        {
            byte[] balanceScript = NativeContract.EpicChain.Hash.MakeScript("balanceOf", sender);
            UT_TransactionManager.MockInvokeScript(rpcClientMock, balanceScript, new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(1_00000000) });

            byte[] testScript = NativeContract.EpicChain.Hash.MakeScript("transfer", sender, sender, new BigInteger(1_00000000), null);
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript, new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(1_10000000) });

            var json = new JObject();
            json["hash"] = UInt256.Zero.ToString();
            rpcClientMock.Setup(p => p.RpcSendAsync("sendrawtransaction", It.IsAny<JToken>())).ReturnsAsync(json);

            var tranaction = await walletAPI.ClaimEpicPulseAsync(keyPair1.Export(), false);
            Assert.AreEqual(testScript.ToHexString(), tranaction.Script.Span.ToHexString());
        }

        [TestMethod]
        public async Task TestTransfer()
        {
            byte[] decimalsScript = NativeContract.EpicPulse.Hash.MakeScript("decimals");
            UT_TransactionManager.MockInvokeScript(rpcClientMock, decimalsScript, new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(8) });

            byte[] testScript = NativeContract.EpicPulse.Hash.MakeScript("transfer", sender, UInt160.Zero, NativeContract.EpicPulse.Factor * 100, null)
                .Concat(new[] { (byte)OpCode.ASSERT })
                .ToArray();
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript, new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(1_10000000) });

            var json = new JObject();
            json["hash"] = UInt256.Zero.ToString();
            rpcClientMock.Setup(p => p.RpcSendAsync("sendrawtransaction", It.IsAny<JToken>())).ReturnsAsync(json);

            var tranaction = await walletAPI.TransferAsync(NativeContract.EpicPulse.Hash.ToString(), keyPair1.Export(), UInt160.Zero.ToAddress(client.protocolSettings.AddressVersion), 100, null, true);
            Assert.AreEqual(testScript.ToHexString(), tranaction.Script.Span.ToHexString());
        }

        [TestMethod]
        public async Task TestTransferfromMultiSigAccount()
        {
            byte[] balanceScript = NativeContract.EpicPulse.Hash.MakeScript("balanceOf", multiSender);
            var balanceResult = new ContractParameter() { Type = ContractParameterType.Integer, Value = BigInteger.Parse("10000000000000000") };

            UT_TransactionManager.MockInvokeScript(rpcClientMock, balanceScript, balanceResult);

            byte[] decimalsScript = NativeContract.EpicPulse.Hash.MakeScript("decimals");
            UT_TransactionManager.MockInvokeScript(rpcClientMock, decimalsScript, new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(8) });

            byte[] testScript = NativeContract.EpicPulse.Hash.MakeScript("transfer", multiSender, UInt160.Zero, NativeContract.EpicPulse.Factor * 100, null)
                .Concat(new[] { (byte)OpCode.ASSERT })
                .ToArray();
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript, new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(1_10000000) });

            var json = new JObject();
            json["hash"] = UInt256.Zero.ToString();
            rpcClientMock.Setup(p => p.RpcSendAsync("sendrawtransaction", It.IsAny<JToken>())).ReturnsAsync(json);

            var tranaction = await walletAPI.TransferAsync(NativeContract.EpicPulse.Hash, 1, new[] { keyPair1.PublicKey }, new[] { keyPair1 }, UInt160.Zero, NativeContract.EpicPulse.Factor * 100, null, true);
            Assert.AreEqual(testScript.ToHexString(), tranaction.Script.Span.ToHexString());

            try
            {
                tranaction = await walletAPI.TransferAsync(NativeContract.EpicPulse.Hash, 2, new[] { keyPair1.PublicKey }, new[] { keyPair1 }, UInt160.Zero, NativeContract.EpicPulse.Factor * 100, null, true);
                Assert.Fail();
            }
            catch (System.Exception e)
            {
                Assert.AreEqual(e.Message, $"Need at least 2 KeyPairs for signing!");
            }

            testScript = NativeContract.EpicPulse.Hash.MakeScript("transfer", multiSender, UInt160.Zero, NativeContract.EpicPulse.Factor * 100, string.Empty)
                .Concat(new[] { (byte)OpCode.ASSERT })
                .ToArray();
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript, new ContractParameter { Type = ContractParameterType.Integer, Value = new BigInteger(1_10000000) });

            tranaction = await walletAPI.TransferAsync(NativeContract.EpicPulse.Hash, 1, new[] { keyPair1.PublicKey }, new[] { keyPair1 }, UInt160.Zero, NativeContract.EpicPulse.Factor * 100, string.Empty, true);
            Assert.AreEqual(testScript.ToHexString(), tranaction.Script.Span.ToHexString());
        }

        [TestMethod]
        public async Task TestWaitTransaction()
        {
            Transaction transaction = TestUtils.GetTransaction();
            rpcClientMock.Setup(p => p.RpcSendAsync("getrawtransaction", It.Is<JToken[]>(j => j[0].AsString() == transaction.Hash.ToString())))
                .ReturnsAsync(new RpcTransaction { Transaction = transaction, VMState = VMState.HALT, BlockHash = UInt256.Zero, BlockTime = 100, Confirmations = 1 }.ToJson(client.protocolSettings));

            var tx = await walletAPI.WaitTransactionAsync(transaction);
            Assert.AreEqual(VMState.HALT, tx.VMState);
            Assert.AreEqual(UInt256.Zero, tx.BlockHash);
        }
    }
}
