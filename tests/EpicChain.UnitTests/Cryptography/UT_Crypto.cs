// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_Crypto.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Cryptography;
using EpicChain.Extensions;
using EpicChain.Wallets;
using System;
using System.Security.Cryptography;

namespace EpicChain.UnitTests.Cryptography
{
    [TestClass]
    public class UT_Crypto
    {
        private KeyPair key = null;

        public static KeyPair GenerateKey(int privateKeyLength)
        {
            byte[] privateKey = new byte[privateKeyLength];
            using (RandomNumberGenerator rng = RandomNumberGenerator.Create())
            {
                rng.GetBytes(privateKey);
            }
            return new KeyPair(privateKey);
        }

        public static KeyPair GenerateCertainKey(int privateKeyLength)
        {
            byte[] privateKey = new byte[privateKeyLength];
            for (int i = 0; i < privateKeyLength; i++)
            {
                privateKey[i] = (byte)((byte)i % byte.MaxValue);
            }
            return new KeyPair(privateKey);
        }

        [TestInitialize]
        public void TestSetup()
        {
            key = GenerateKey(32);
        }

        [TestMethod]
        public void TestVerifySignature()
        {
            byte[] message = System.Text.Encoding.Default.GetBytes("HelloWorld");
            byte[] signature = Crypto.Sign(message, key.PrivateKey);
            Crypto.VerifySignature(message, signature, key.PublicKey).Should().BeTrue();

            byte[] wrongKey = new byte[33];
            wrongKey[0] = 0x02;
            Crypto.VerifySignature(message, signature, wrongKey, EpicChain.Cryptography.ECC.ECCurve.Secp256r1).Should().BeFalse();

            wrongKey[0] = 0x03;
            for (int i = 1; i < 33; i++) wrongKey[i] = byte.MaxValue;
            Action action = () => Crypto.VerifySignature(message, signature, wrongKey, EpicChain.Cryptography.ECC.ECCurve.Secp256r1);
            action.Should().Throw<ArgumentException>();

            wrongKey = new byte[36];
            action = () => Crypto.VerifySignature(message, signature, wrongKey, EpicChain.Cryptography.ECC.ECCurve.Secp256r1);
            action.Should().Throw<FormatException>();
        }

        [TestMethod]
        public void TestSecp256k1()
        {
            byte[] privkey = "7177f0d04c79fa0b8c91fe90c1cf1d44772d1fba6e5eb9b281a22cd3aafb51fe".HexToBytes();
            byte[] message = "2d46a712699bae19a634563d74d04cc2da497b841456da270dccb75ac2f7c4e7".HexToBytes();
            var signature = Crypto.Sign(message, privkey, EpicChain.Cryptography.ECC.ECCurve.Secp256k1);

            byte[] pubKey = "04fd0a8c1ce5ae5570fdd46e7599c16b175bf0ebdfe9c178f1ab848fb16dac74a5d301b0534c7bcf1b3760881f0c420d17084907edd771e1c9c8e941bbf6ff9108".HexToBytes();
            Crypto.VerifySignature(message, signature, pubKey, EpicChain.Cryptography.ECC.ECCurve.Secp256k1)
                .Should().BeTrue();

            message = System.Text.Encoding.Default.GetBytes("world");
            signature = Crypto.Sign(message, privkey, EpicChain.Cryptography.ECC.ECCurve.Secp256k1);

            Crypto.VerifySignature(message, signature, pubKey, EpicChain.Cryptography.ECC.ECCurve.Secp256k1)
                .Should().BeTrue();

            message = System.Text.Encoding.Default.GetBytes("中文");
            signature = "b8cba1ff42304d74d083e87706058f59cdd4f755b995926d2cd80a734c5a3c37e4583bfd4339ac762c1c91eee3782660a6baf62cd29e407eccd3da3e9de55a02".HexToBytes();
            pubKey = "03661b86d54eb3a8e7ea2399e0db36ab65753f95fff661da53ae0121278b881ad0".HexToBytes();

            Crypto.VerifySignature(message, signature, pubKey, EpicChain.Cryptography.ECC.ECCurve.Secp256k1)
                .Should().BeTrue();
        }
    }
}
