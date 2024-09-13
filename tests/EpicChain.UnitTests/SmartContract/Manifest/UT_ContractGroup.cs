// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_ContractGroup.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Cryptography;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Manifest;
using EpicChain.Wallets;
using System;

namespace EpicChain.UnitTests.SmartContract.Manifest
{
    [TestClass]
    public class UT_ContractGroup
    {
        [TestMethod]
        public void TestClone()
        {
            Random random = new();
            byte[] privateKey = new byte[32];
            random.NextBytes(privateKey);
            KeyPair keyPair = new(privateKey);
            ContractGroup contractGroup = new()
            {
                PubKey = keyPair.PublicKey,
                Signature = new byte[20]
            };

            ContractGroup clone = new();
            ((IInteroperable)clone).FromStackItem(contractGroup.ToStackItem(null));
            Assert.AreEqual(clone.ToJson().ToString(), contractGroup.ToJson().ToString());
        }

        [TestMethod]
        public void TestIsValid()
        {
            Random random = new();
            var privateKey = new byte[32];
            random.NextBytes(privateKey);
            KeyPair keyPair = new(privateKey);
            ContractGroup contractGroup = new()
            {
                PubKey = keyPair.PublicKey,
                Signature = new byte[20]
            };
            Assert.AreEqual(false, contractGroup.IsValid(UInt160.Zero));


            var message = new byte[] {  0x01,0x01,0x01,0x01,0x01,
                                           0x01,0x01,0x01,0x01,0x01,
                                           0x01,0x01,0x01,0x01,0x01,
                                           0x01,0x01,0x01,0x01,0x01 };
            var signature = Crypto.Sign(message, keyPair.PrivateKey);
            contractGroup = new ContractGroup
            {
                PubKey = keyPair.PublicKey,
                Signature = signature
            };
            Assert.AreEqual(true, contractGroup.IsValid(new UInt160(message)));
        }
    }
}
