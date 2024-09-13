// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_TransactionState.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.IO;
using EpicChain.Network.P2P.Payloads;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using System;

namespace EpicChain.UnitTests.Ledger
{
    [TestClass]
    public class UT_TransactionState
    {
        TransactionState origin;

        TransactionState originTrimmed;

        [TestInitialize]
        public void Initialize()
        {
            origin = new TransactionState
            {
                BlockIndex = 1,
                Transaction = new Transaction()
                {
                    Attributes = Array.Empty<TransactionAttribute>(),
                    Script = new byte[] { (byte)OpCode.PUSH1 },
                    Signers = new Signer[] { new Signer() { Account = UInt160.Zero } },
                    Witnesses = new Witness[] { new Witness() {
                        InvocationScript=Array.Empty<byte>(),
                        VerificationScript=Array.Empty<byte>()
                    } }
                }
            };
            originTrimmed = new TransactionState
            {
                BlockIndex = 1,
            };
        }

        [TestMethod]
        public void TestDeserialize()
        {
            var data = BinarySerializer.Serialize(((IInteroperable)origin).ToStackItem(null), ExecutionEngineLimits.Default);
            var reader = new MemoryReader(data);

            TransactionState dest = new();
            ((IInteroperable)dest).FromStackItem(BinarySerializer.Deserialize(ref reader, ExecutionEngineLimits.Default, null));

            dest.BlockIndex.Should().Be(origin.BlockIndex);
            dest.Transaction.Hash.Should().Be(origin.Transaction.Hash);
            dest.Transaction.Should().NotBeNull();
        }

        [TestMethod]
        public void TestDeserializeTrimmed()
        {
            var data = BinarySerializer.Serialize(((IInteroperable)originTrimmed).ToStackItem(null), ExecutionEngineLimits.Default);
            var reader = new MemoryReader(data);

            TransactionState dest = new();
            ((IInteroperable)dest).FromStackItem(BinarySerializer.Deserialize(ref reader, ExecutionEngineLimits.Default, null));

            dest.BlockIndex.Should().Be(originTrimmed.BlockIndex);
            dest.Transaction.Should().Be(null);
            dest.Transaction.Should().BeNull();
        }
    }
}
