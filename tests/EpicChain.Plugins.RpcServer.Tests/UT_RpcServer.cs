// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_RpcServer.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Microsoft.AspNetCore.Http;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using EpicChain.IO;
using EpicChain.Ledger;
using EpicChain.Persistence;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.UnitTests;
using EpicChain.Wallets;
using EpicChain.Wallets.XEP6;
using System;
using System.Linq;
using System.Net;
using System.Numerics;
using System.Text;

namespace EpicChain.Plugins.RpcServer.Tests
{
    [TestClass]
    public partial class UT_RpcServer
    {
        private EpicChainSystem _EpicChainSystem;
        private RpcServerSettings _rpcServerSettings;
        private RpcServer _rpcServer;
        private TestMemoryStoreProvider _memoryStoreProvider;
        private MemoryStore _memoryStore;
        private readonly XEP6Wallet _wallet = TestUtils.GenerateTestWallet("123");
        private WalletAccount _walletAccount;

        const byte NativePrefixAccount = 20;
        const byte NativePrefixTotalSupply = 11;

        [TestInitialize]
        public void TestSetup()
        {
            _memoryStore = new MemoryStore();
            _memoryStoreProvider = new TestMemoryStoreProvider(_memoryStore);
            _EpicChainSystem = new EpicChainSystem(TestProtocolSettings.SoleNode, _memoryStoreProvider);
            _rpcServerSettings = RpcServerSettings.Default with
            {
                SessionEnabled = true,
                SessionExpirationTime = TimeSpan.FromSeconds(0.3),
                maxEpicPulseInvoke = 1500_0000_0000,
                Network = TestProtocolSettings.SoleNode.Network,
            };
            _rpcServer = new RpcServer(_EpicChainSystem, _rpcServerSettings);
            _walletAccount = _wallet.Import("KxuRSsHgJMb3AMSN6B9P3JHNGMFtxmuimqgR9MmXPcv3CLLfusTd");
            var key = new KeyBuilder(NativeContract.EpicPulse.Id, 20).Add(_walletAccount.ScriptHash);
            var snapshot = _EpicChainSystem.GetSnapshotCache();
            var entry = snapshot.GetAndChange(key, () => new StorageItem(new AccountState()));
            entry.GetInteroperable<AccountState>().Balance = 100_000_000 * NativeContract.EpicPulse.Factor;
            snapshot.Commit();
        }

        [TestCleanup]
        public void TestCleanup()
        {
            // Please build and test in debug mode
            _EpicChainSystem.MemPool.Clear();
            _memoryStore.Reset();
            var snapshot = _EpicChainSystem.GetSnapshotCache();
            var key = new KeyBuilder(NativeContract.EpicPulse.Id, 20).Add(_walletAccount.ScriptHash);
            var entry = snapshot.GetAndChange(key, () => new StorageItem(new AccountState()));
            entry.GetInteroperable<AccountState>().Balance = 100_000_000 * NativeContract.EpicPulse.Factor;
            snapshot.Commit();
        }

        [TestMethod]
        public void TestCheckAuth_ValidCredentials_ReturnsTrue()
        {
            // Arrange
            var context = new DefaultHttpContext();
            context.Request.Headers["Authorization"] = "Basic " + Convert.ToBase64String(Encoding.UTF8.GetBytes("testuser:testpass"));
            // Act
            var result = _rpcServer.CheckAuth(context);
            // Assert
            Assert.IsTrue(result);
        }
    }
}
