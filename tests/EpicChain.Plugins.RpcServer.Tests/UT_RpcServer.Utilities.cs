// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_RpcServer.Wallet.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using Microsoft.AspNetCore.Http;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using EpicChain.IO;
using EpicChain.Json;
using EpicChain.Network.P2P.Payloads;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.UnitTests;
using EpicChain.UnitTests.Extensions;
using System;
using System.IO;
using System.Linq;

namespace EpicChain.Plugins.RpcServer.Tests;

public partial class UT_RpcServer
{
    [TestMethod]
    public void TestListPlugins()
    {
        JArray resp = (JArray)_rpcServer.ListPlugins([]);
        Assert.AreEqual(resp.Count, 0);
        Plugins.Plugin.Plugins.Add(new RpcServerPlugin());
        resp = (JArray)_rpcServer.ListPlugins([]);
        Assert.AreEqual(resp.Count, 2);
        foreach (JObject p in resp)
            Assert.AreEqual(p["name"], nameof(RpcServer));
    }

    [TestMethod]
    public void TestValidateAddress()
    {
        string validAddr = "XgsJXcjSB4sLRMQgW6CGc9887Lc87MmsUu";
        JObject resp = (JObject)_rpcServer.ValidateAddress([validAddr]);
        Assert.AreEqual(resp["address"], validAddr);
        Assert.AreEqual(resp["isvalid"], true);
        string invalidAddr = "AEpicChain2toEpicChain3MigrationAddressxwPB2Hz";
        resp = (JObject)_rpcServer.ValidateAddress([invalidAddr]);
        Assert.AreEqual(resp["address"], invalidAddr);
        Assert.AreEqual(resp["isvalid"], false);
    }
}
