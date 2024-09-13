// Copyright (C) 2021-2024 EpicChain Labs.

//
// TestWalletAccount.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Moq;
using EpicChain.SmartContract;
using EpicChain.Wallets;
using System;

namespace EpicChain.UnitTests
{
    class TestWalletAccount : WalletAccount
    {
        private static readonly KeyPair key;

        public override bool HasKey => true;
        public override KeyPair GetKey() => key;

        public TestWalletAccount(UInt160 hash)
            : base(hash, TestProtocolSettings.Default)
        {
            var mock = new Mock<Contract>();
            mock.SetupGet(p => p.ScriptHash).Returns(hash);
            mock.Object.Script = Contract.CreateSignatureRedeemScript(key.PublicKey);
            mock.Object.ParameterList = new[] { ContractParameterType.Signature };
            Contract = mock.Object;
        }

        static TestWalletAccount()
        {
            Random random = new();
            byte[] prikey = new byte[32];
            random.NextBytes(prikey);
            key = new KeyPair(prikey);
        }
    }
}
