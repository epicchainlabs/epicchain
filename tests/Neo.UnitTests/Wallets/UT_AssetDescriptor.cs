// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_AssetDescriptor.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using Neo.SmartContract.Native;
using System;

namespace Neo.UnitTests.Wallets
{
    [TestClass]
    public class UT_AssetDescriptor
    {
        [TestMethod]
        public void TestConstructorWithNonexistAssetId()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            Action action = () =>
            {
                var descriptor = new Neo.Wallets.AssetDescriptor(snapshotCache, TestProtocolSettings.Default, UInt160.Parse("01ff00ff00ff00ff00ff00ff00ff00ff00ff00a4"));
            };
            action.Should().Throw<ArgumentException>();
        }

        [TestMethod]
        public void Check_EpicPulse()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            var descriptor = new Neo.Wallets.AssetDescriptor(snapshotCache, TestProtocolSettings.Default, NativeContract.EpicPulse.Hash);
            descriptor.AssetId.Should().Be(NativeContract.EpicPulse.Hash);
            descriptor.AssetName.Should().Be(nameof(EpicPulse));
            descriptor.ToString().Should().Be(nameof(EpicPulse));
            descriptor.Symbol.Should().Be("XPP");
            descriptor.Decimals.Should().Be(8);
        }

        [TestMethod]
        public void Check_EPICCHAIN()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            var descriptor = new Neo.Wallets.AssetDescriptor(snapshotCache, TestProtocolSettings.Default, NativeContract.NEO.Hash);
            descriptor.AssetId.Should().Be(NativeContract.NEO.Hash);
            descriptor.AssetName.Should().Be(nameof(EpicChain));
            descriptor.ToString().Should().Be(nameof(EpicChain));
            descriptor.Symbol.Should().Be("XPR");
            descriptor.Decimals.Should().Be(0);
        }
    }
}
