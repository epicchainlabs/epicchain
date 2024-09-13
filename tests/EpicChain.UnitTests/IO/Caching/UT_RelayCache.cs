// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_RelayCache.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using Microsoft.VisualStudio.TestTools.UnitTesting;
using EpicChain.IO.Caching;
using EpicChain.Network.P2P.Payloads;
using System;

namespace EpicChain.UnitTests.IO.Caching
{
    [TestClass]
    public class UT_RelayCache
    {
        RelayCache relayCache;

        [TestInitialize]
        public void SetUp()
        {
            relayCache = new RelayCache(10);
        }

        [TestMethod]
        public void TestGetKeyForItem()
        {
            Transaction tx = new Transaction()
            {
                Version = 0,
                Nonce = 1,
                SystemFee = 0,
                NetworkFee = 0,
                ValidUntilBlock = 100,
                Attributes = Array.Empty<TransactionAttribute>(),
                Signers = Array.Empty<Signer>(),
                Script = new byte[] { 0x00, 0x01, 0x02, 0x03, 0x04 },
                Witnesses = Array.Empty<Witness>()
            };
            relayCache.Add(tx);
            relayCache.Contains(tx).Should().BeTrue();
            relayCache.TryGet(tx.Hash, out IInventory tmp).Should().BeTrue();
            (tmp is Transaction).Should().BeTrue();
        }
    }
}
