// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_RpcModels.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Json;
using EpicChain.Network.RPC.Models;
using System;
using System.Linq;
using System.Net.Http;

namespace EpicChain.Network.RPC.Tests
{
    [TestClass()]
    public class UT_RpcModels
    {
        RpcClient rpc;
        Mock<HttpMessageHandler> handlerMock;

        [TestInitialize]
        public void TestSetup()
        {
            handlerMock = new Mock<HttpMessageHandler>(MockBehavior.Strict);

            // use real http client with mocked handler here
            var httpClient = new HttpClient(handlerMock.Object);
            rpc = new RpcClient(httpClient, new Uri("http://mainnet1-seed.epic-chain.org:10112"), null);
        }

        [TestMethod()]
        public void TestRpcAccount()
        {
            JToken json = TestUtils.RpcTestCases.Find(p => p.Name == nameof(RpcClient.ImportPrivKeyAsync).ToLower()).Response.Result;
            var item = RpcAccount.FromJson((JObject)json);
            Assert.AreEqual(json.ToString(), item.ToJson().ToString());
        }

        [TestMethod()]
        public void TestRpcApplicationLog()
        {
            JToken json = TestUtils.RpcTestCases.Find(p => p.Name == nameof(RpcClient.GetApplicationLogAsync).ToLower()).Response.Result;
            var item = RpcApplicationLog.FromJson((JObject)json, rpc.protocolSettings);
            Assert.AreEqual(json.ToString(), item.ToJson().ToString());
        }

        [TestMethod()]
        public void TestRpcBlock()
        {
            JToken json = TestUtils.RpcTestCases.Find(p => p.Name == nameof(RpcClient.GetBlockAsync).ToLower()).Response.Result;
            var item = RpcBlock.FromJson((JObject)json, rpc.protocolSettings);
            Assert.AreEqual(json.ToString(), item.ToJson(rpc.protocolSettings).ToString());
        }

        [TestMethod()]
        public void TestRpcBlockHeader()
        {
            JToken json = TestUtils.RpcTestCases.Find(p => p.Name == nameof(RpcClient.GetBlockHeaderAsync).ToLower()).Response.Result;
            var item = RpcBlockHeader.FromJson((JObject)json, rpc.protocolSettings);
            Assert.AreEqual(json.ToString(), item.ToJson(rpc.protocolSettings).ToString());
        }

        [TestMethod()]
        public void TestGetContractState()
        {
            JToken json = TestUtils.RpcTestCases.Find(p => p.Name == nameof(RpcClient.GetContractStateAsync).ToLower()).Response.Result;
            var item = RpcContractState.FromJson((JObject)json);
            Assert.AreEqual(json.ToString(), item.ToJson().ToString());

            var nef = RpcXefFile.FromJson((JObject)json["nef"]);
            Assert.AreEqual(json["nef"].ToString(), nef.ToJson().ToString());
        }

        [TestMethod()]
        public void TestRpcInvokeResult()
        {
            JToken json = TestUtils.RpcTestCases.Find(p => p.Name == nameof(RpcClient.InvokeFunctionAsync).ToLower()).Response.Result;
            var item = RpcInvokeResult.FromJson((JObject)json);
            Assert.AreEqual(json.ToString(), item.ToJson().ToString());
        }

        [TestMethod()]
        public void TestRpcMethodToken()
        {
            RpcMethodToken.FromJson((JObject)JToken.Parse("{\"hash\": \"0x0e1b9bfaa44e60311f6f3c96cfcd6d12c2fc3add\", \"method\":\"test\",\"paramcount\":\"1\",\"hasreturnvalue\":\"true\",\"callflags\":\"All\"}"));
        }

        [TestMethod()]
        public void TestRpcXep17Balances()
        {
            JToken json = TestUtils.RpcTestCases.Find(p => p.Name == nameof(RpcClient.GetXep17BalancesAsync).ToLower()).Response.Result;
            var item = RpcXep17Balances.FromJson((JObject)json, rpc.protocolSettings);
            Assert.AreEqual(json.ToString(), item.ToJson(rpc.protocolSettings).ToString());
        }

        [TestMethod()]
        public void TestRpcXep17Transfers()
        {
            JToken json = TestUtils.RpcTestCases.Find(p => p.Name == nameof(RpcClient.GetXep17TransfersAsync).ToLower()).Response.Result;
            var item = RpcXep17Transfers.FromJson((JObject)json, rpc.protocolSettings);
            Assert.AreEqual(json.ToString(), item.ToJson(rpc.protocolSettings).ToString());
        }

        [TestMethod()]
        public void TestRpcPeers()
        {
            JToken json = TestUtils.RpcTestCases.Find(p => p.Name == nameof(RpcClient.GetPeersAsync).ToLower()).Response.Result;
            var item = RpcPeers.FromJson((JObject)json);
            Assert.AreEqual(json.ToString(), item.ToJson().ToString());
        }

        [TestMethod()]
        public void TestRpcPlugin()
        {
            JToken json = TestUtils.RpcTestCases.Find(p => p.Name == nameof(RpcClient.ListPluginsAsync).ToLower()).Response.Result;
            var item = ((JArray)json).Select(p => RpcPlugin.FromJson((JObject)p));
            Assert.AreEqual(json.ToString(), ((JArray)item.Select(p => p.ToJson()).ToArray()).ToString());
        }

        [TestMethod()]
        public void TestRpcRawMemPool()
        {
            JToken json = TestUtils.RpcTestCases.Find(p => p.Name == nameof(RpcClient.GetRawMempoolBothAsync).ToLower()).Response.Result;
            var item = RpcRawMemPool.FromJson((JObject)json);
            Assert.AreEqual(json.ToString(), item.ToJson().ToString());
        }

        [TestMethod()]
        public void TestRpcTransaction()
        {
            JToken json = TestUtils.RpcTestCases.Find(p => p.Name == nameof(RpcClient.GetRawTransactionAsync).ToLower()).Response.Result;
            var item = RpcTransaction.FromJson((JObject)json, rpc.protocolSettings);
            Assert.AreEqual(json.ToString(), item.ToJson(rpc.protocolSettings).ToString());
        }

        [TestMethod()]
        public void TestRpcTransferOut()
        {
            JToken json = TestUtils.RpcTestCases.Find(p => p.Name == nameof(RpcClient.SendManyAsync).ToLower()).Request.Params[1];
            var item = ((JArray)json).Select(p => RpcTransferOut.FromJson((JObject)p, rpc.protocolSettings));
            Assert.AreEqual(json.ToString(), ((JArray)item.Select(p => p.ToJson(rpc.protocolSettings)).ToArray()).ToString());
        }

        [TestMethod()]
        public void TestRpcValidateAddressResult()
        {
            JToken json = TestUtils.RpcTestCases.Find(p => p.Name == nameof(RpcClient.ValidateAddressAsync).ToLower()).Response.Result;
            var item = RpcValidateAddressResult.FromJson((JObject)json);
            Assert.AreEqual(json.ToString(), item.ToJson().ToString());
        }

        [TestMethod()]
        public void TestRpcValidator()
        {
            JToken json = TestUtils.RpcTestCases.Find(p => p.Name == nameof(RpcClient.GetNextBlockValidatorsAsync).ToLower()).Response.Result;
            var item = ((JArray)json).Select(p => RpcValidator.FromJson((JObject)p));
            Assert.AreEqual(json.ToString(), ((JArray)item.Select(p => p.ToJson()).ToArray()).ToString());
        }

        [TestMethod()]
        public void TestRpcVersion()
        {
            JToken json = TestUtils.RpcTestCases.Find(p => p.Name == nameof(RpcClient.GetVersionAsync).ToLower()).Response.Result;
            var item = RpcVersion.FromJson((JObject)json);
            Assert.AreEqual(json.ToString(), item.ToJson().ToString());
        }
    }
}
