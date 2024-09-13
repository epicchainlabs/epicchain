// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_RpcServer.Node.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Akka.Actor;
using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using EpicChain.IO;
using EpicChain.Json;
using EpicChain.Network.P2P;
using EpicChain.Network.P2P.Payloads;
using EpicChain.SmartContract.Native;
using EpicChain.UnitTests;
using System;
using System.Collections.Generic;
using System.Net;

namespace EpicChain.Plugins.RpcServer.Tests
{
    partial class UT_RpcServer
    {
        [TestMethod]
        public void TestGetConnectionCount()
        {
            var result = _rpcServer.GetConnectionCount(new JArray());
            result.GetType().Should().Be(typeof(JNumber));
        }

        [TestMethod]
        public void TestGetPeers()
        {
            var settings = TestProtocolSettings.SoleNode;
            var EpicChainSystem = new EpicChainSystem(settings, _memoryStoreProvider);
            var localNode = EpicChainSystem.LocalNode.Ask<LocalNode>(new LocalNode.GetInstance()).Result;
            localNode.AddPeers(new List<IPEndPoint>() { new IPEndPoint(new IPAddress(new byte[] { 127, 0, 0, 1 }), 11332) });
            localNode.AddPeers(new List<IPEndPoint>() { new IPEndPoint(new IPAddress(new byte[] { 127, 0, 0, 1 }), 12332) });
            localNode.AddPeers(new List<IPEndPoint>() { new IPEndPoint(new IPAddress(new byte[] { 127, 0, 0, 1 }), 13332) });
            var rpcServer = new RpcServer(EpicChainSystem, RpcServerSettings.Default);

            var result = rpcServer.GetPeers(new JArray());
            Assert.IsInstanceOfType(result, typeof(JObject));
            var json = (JObject)result;
            json.ContainsProperty("unconnected").Should().BeTrue();
            (json["unconnected"] as JArray).Count.Should().Be(3);
            json.ContainsProperty("bad").Should().BeTrue();
            json.ContainsProperty("connected").Should().BeTrue();
        }

        [TestMethod]
        public void TestGetVersion()
        {
            var result = _rpcServer.GetVersion(new JArray());
            Assert.IsInstanceOfType(result, typeof(JObject));

            var json = (JObject)result;
            Assert.IsTrue(json.ContainsProperty("tcpport"));
            Assert.IsTrue(json.ContainsProperty("nonce"));
            Assert.IsTrue(json.ContainsProperty("useragent"));

            Assert.IsTrue(json.ContainsProperty("protocol"));
            var protocol = (JObject)json["protocol"];
            Assert.IsTrue(protocol.ContainsProperty("addressversion"));
            Assert.IsTrue(protocol.ContainsProperty("network"));
            Assert.IsTrue(protocol.ContainsProperty("validatorscount"));
            Assert.IsTrue(protocol.ContainsProperty("msperblock"));
            Assert.IsTrue(protocol.ContainsProperty("maxtraceableblocks"));
            Assert.IsTrue(protocol.ContainsProperty("maxvaliduntilblockincrement"));
            Assert.IsTrue(protocol.ContainsProperty("maxtransactionsperblock"));
            Assert.IsTrue(protocol.ContainsProperty("memorypoolmaxtransactions"));
            Assert.IsTrue(protocol.ContainsProperty("standbycommittee"));
            Assert.IsTrue(protocol.ContainsProperty("seedlist"));
        }

        #region SendRawTransaction Tests

        [TestMethod]
        public void TestSendRawTransaction_Normal()
        {
            var snapshot = _EpicChainSystem.GetSnapshotCache();
            var tx = TestUtils.CreateValidTx(snapshot, _wallet, _walletAccount);
            var txString = Convert.ToBase64String(tx.ToArray());

            var result = _rpcServer.SendRawTransaction(new JArray(txString));
            Assert.IsInstanceOfType(result, typeof(JObject));
            Assert.IsTrue(((JObject)result).ContainsProperty("hash"));
        }

        [TestMethod]
        public void TestSendRawTransaction_InvalidTransactionFormat()
        {
            Assert.ThrowsException<RpcException>(() =>
                _rpcServer.SendRawTransaction(new JArray("invalid_transaction_string")),
                "Should throw RpcException for invalid transaction format");
        }

        [TestMethod]
        public void TestSendRawTransaction_InsufficientBalance()
        {
            var snapshot = _EpicChainSystem.GetSnapshotCache();
            var tx = TestUtils.CreateInvalidTransaction(snapshot, _wallet, _walletAccount, TestUtils.InvalidTransactionType.InsufficientBalance);
            var txString = Convert.ToBase64String(tx.ToArray());

            var exception = Assert.ThrowsException<RpcException>(() =>
                _rpcServer.SendRawTransaction(new JArray(txString)),
                "Should throw RpcException for insufficient balance");
            Assert.AreEqual(RpcError.InsufficientFunds.Code, exception.HResult);
        }

        [TestMethod]
        public void TestSendRawTransaction_InvalidSignature()
        {
            var snapshot = _EpicChainSystem.GetSnapshotCache();
            var tx = TestUtils.CreateInvalidTransaction(snapshot, _wallet, _walletAccount, TestUtils.InvalidTransactionType.InvalidSignature);
            var txString = Convert.ToBase64String(tx.ToArray());

            var exception = Assert.ThrowsException<RpcException>(() =>
                _rpcServer.SendRawTransaction(new JArray(txString)),
                "Should throw RpcException for invalid signature");
            Assert.AreEqual(RpcError.InvalidSignature.Code, exception.HResult);
        }

        [TestMethod]
        public void TestSendRawTransaction_InvalidScript()
        {
            var snapshot = _EpicChainSystem.GetSnapshotCache();
            var tx = TestUtils.CreateInvalidTransaction(snapshot, _wallet, _walletAccount, TestUtils.InvalidTransactionType.InvalidScript);
            var txString = Convert.ToBase64String(tx.ToArray());

            var exception = Assert.ThrowsException<RpcException>(() =>
                _rpcServer.SendRawTransaction(new JArray(txString)),
                "Should throw RpcException for invalid script");
            Assert.AreEqual(RpcError.InvalidScript.Code, exception.HResult);
        }

        [TestMethod]
        public void TestSendRawTransaction_InvalidAttribute()
        {
            var snapshot = _EpicChainSystem.GetSnapshotCache();
            var tx = TestUtils.CreateInvalidTransaction(snapshot, _wallet, _walletAccount, TestUtils.InvalidTransactionType.InvalidAttribute);
            var txString = Convert.ToBase64String(tx.ToArray());

            var exception = Assert.ThrowsException<RpcException>(() =>
                _rpcServer.SendRawTransaction(new JArray(txString)),
                "Should throw RpcException for invalid attribute");
            // Transaction with invalid attribute can not pass the Transaction deserialization
            // and will throw invalid params exception.
            Assert.AreEqual(RpcError.InvalidParams.Code, exception.HResult);
        }

        [TestMethod]
        public void TestSendRawTransaction_Oversized()
        {
            var snapshot = _EpicChainSystem.GetSnapshotCache();
            var tx = TestUtils.CreateInvalidTransaction(snapshot, _wallet, _walletAccount, TestUtils.InvalidTransactionType.Oversized);
            var txString = Convert.ToBase64String(tx.ToArray());

            var exception = Assert.ThrowsException<RpcException>(() =>
                _rpcServer.SendRawTransaction(new JArray(txString)),
                "Should throw RpcException for invalid format transaction");
            // Oversized transaction will not pass the deserialization.
            Assert.AreEqual(RpcError.InvalidParams.Code, exception.HResult);
        }

        [TestMethod]
        public void TestSendRawTransaction_Expired()
        {
            var snapshot = _EpicChainSystem.GetSnapshotCache();
            var tx = TestUtils.CreateInvalidTransaction(snapshot, _wallet, _walletAccount, TestUtils.InvalidTransactionType.Expired);
            var txString = Convert.ToBase64String(tx.ToArray());

            var exception = Assert.ThrowsException<RpcException>(() =>
                _rpcServer.SendRawTransaction(new JArray(txString)),
                "Should throw RpcException for expired transaction");
            Assert.AreEqual(RpcError.ExpiredTransaction.Code, exception.HResult);
        }

        [TestMethod]
        public void TestSendRawTransaction_PolicyFailed()
        {
            var snapshot = _EpicChainSystem.GetSnapshotCache();
            var tx = TestUtils.CreateValidTx(snapshot, _wallet, _walletAccount);
            var txString = Convert.ToBase64String(tx.ToArray());
            NativeContract.Policy.BlockAccount(snapshot, _walletAccount.ScriptHash);
            snapshot.Commit();

            var exception = Assert.ThrowsException<RpcException>(() =>
                _rpcServer.SendRawTransaction(new JArray(txString)),
                "Should throw RpcException for conflicting transaction");
            Assert.AreEqual(RpcError.PolicyFailed.Code, exception.HResult);
        }

        [TestMethod]
        public void TestSendRawTransaction_AlreadyInPool()
        {
            var snapshot = _EpicChainSystem.GetSnapshotCache();
            var tx = TestUtils.CreateValidTx(snapshot, _wallet, _walletAccount);
            _EpicChainSystem.MemPool.TryAdd(tx, snapshot);
            var txString = Convert.ToBase64String(tx.ToArray());

            var exception = Assert.ThrowsException<RpcException>(() =>
                _rpcServer.SendRawTransaction(new JArray(txString)),
                "Should throw RpcException for transaction already in memory pool");
            Assert.AreEqual(RpcError.AlreadyInPool.Code, exception.HResult);
        }

        [TestMethod]
        public void TestSendRawTransaction_AlreadyInBlockchain()
        {
            var snapshot = _EpicChainSystem.GetSnapshotCache();
            var tx = TestUtils.CreateValidTx(snapshot, _wallet, _walletAccount);
            TestUtils.AddTransactionToBlockchain(snapshot, tx);
            snapshot.Commit();
            var txString = Convert.ToBase64String(tx.ToArray());
            var exception = Assert.ThrowsException<RpcException>(() => _rpcServer.SendRawTransaction(new JArray(txString)));
            Assert.AreEqual(RpcError.AlreadyExists.Code, exception.HResult);
        }

        #endregion

        #region SubmitBlock Tests

        [TestMethod]
        public void TestSubmitBlock_Normal()
        {
            var snapshot = _EpicChainSystem.GetSnapshotCache();
            var block = TestUtils.CreateBlockWithValidTransactions(snapshot, _wallet, _walletAccount, 1);
            var blockString = Convert.ToBase64String(block.ToArray());

            var result = _rpcServer.SubmitBlock(new JArray(blockString));
            Assert.IsInstanceOfType(result, typeof(JObject));
            Assert.IsTrue(((JObject)result).ContainsProperty("hash"));
        }

        [TestMethod]
        public void TestSubmitBlock_InvalidBlockFormat()
        {
            string invalidBlockString = TestUtils.CreateInvalidBlockFormat();

            var exception = Assert.ThrowsException<RpcException>(() =>
                _rpcServer.SubmitBlock(new JArray(invalidBlockString)),
                "Should throw RpcException for invalid block format");

            Assert.AreEqual(RpcError.InvalidParams.Code, exception.HResult);
            StringAssert.Contains(exception.Message, "Invalid Block Format");
        }

        [TestMethod]
        public void TestSubmitBlock_AlreadyExists()
        {
            var snapshot = _EpicChainSystem.GetSnapshotCache();
            var block = TestUtils.CreateBlockWithValidTransactions(snapshot, _wallet, _walletAccount, 1);
            TestUtils.BlocksAdd(snapshot, block.Hash, block);
            snapshot.Commit();
            var blockString = Convert.ToBase64String(block.ToArray());

            var exception = Assert.ThrowsException<RpcException>(() =>
                _rpcServer.SubmitBlock(new JArray(blockString)),
                "Should throw RpcException when block already exists");
            Assert.AreEqual(RpcError.AlreadyExists.Code, exception.HResult);
        }

        [TestMethod]
        public void TestSubmitBlock_InvalidBlock()
        {
            var snapshot = _EpicChainSystem.GetSnapshotCache();
            var block = TestUtils.CreateBlockWithValidTransactions(snapshot, _wallet, _walletAccount, 1);
            block.Header.Witness = new Witness();
            var blockString = Convert.ToBase64String(block.ToArray());

            var exception = Assert.ThrowsException<RpcException>(() =>
                _rpcServer.SubmitBlock(new JArray(blockString)),
                "Should throw RpcException for invalid block");
            Assert.AreEqual(RpcError.VerificationFailed.Code, exception.HResult);
        }

        #endregion

        #region Edge Cases and Error Handling

        [TestMethod]
        public void TestSendRawTransaction_NullInput()
        {
            var exception = Assert.ThrowsException<RpcException>(() =>
                _rpcServer.SendRawTransaction(new JArray((string)null)),
                "Should throw RpcException for null input");
            Assert.AreEqual(RpcError.InvalidParams.Code, exception.HResult);
        }

        [TestMethod]
        public void TestSendRawTransaction_EmptyInput()
        {
            var exception = Assert.ThrowsException<RpcException>(() =>
                _rpcServer.SendRawTransaction(new JArray(string.Empty)),
                "Should throw RpcException for empty input");
            Assert.AreEqual(RpcError.InvalidParams.Code, exception.HResult);
        }

        [TestMethod]
        public void TestSubmitBlock_NullInput()
        {
            var exception = Assert.ThrowsException<RpcException>(() =>
                _rpcServer.SubmitBlock(new JArray((string)null)),
                "Should throw RpcException for null input");
            Assert.AreEqual(RpcError.InvalidParams.Code, exception.HResult);
        }

        [TestMethod]
        public void TestSubmitBlock_EmptyInput()
        {
            var exception = Assert.ThrowsException<RpcException>(() =>
                _rpcServer.SubmitBlock(new JArray(string.Empty)),
                "Should throw RpcException for empty input");
            Assert.AreEqual(RpcError.InvalidParams.Code, exception.HResult);
        }

        #endregion
    }
}
