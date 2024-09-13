// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_Cryptography_Helper.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.IO;
using EpicChain.Network.P2P.Payloads;
using EpicChain.SmartContract;
using EpicChain.Wallets;
using EpicChain.Wallets.XEP6;
using System;
using System.Linq;
using System.Text;

namespace EpicChain.UnitTests.Cryptography
{
    [TestClass]
    public class UT_Cryptography_Helper
    {
        [TestMethod]
        public void TestBase58CheckDecode()
        {
            string input = "3vQB7B6MrGQZaxCuFg4oh";
            byte[] result = input.Base58CheckDecode();
            byte[] helloWorld = { 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100 };
            result.Should().Equal(helloWorld);

            input = "3v";
            Action action = () => input.Base58CheckDecode();
            action.Should().Throw<FormatException>();

            input = "3vQB7B6MrGQZaxCuFg4og";
            action = () => input.Base58CheckDecode();
            action.Should().Throw<FormatException>();

            Assert.ThrowsException<FormatException>(() => string.Empty.Base58CheckDecode());
        }

        [TestMethod]
        public void TestMurmurReadOnlySpan()
        {
            ReadOnlySpan<byte> input = "Hello, world!"u8;
            byte[] input2 = input.ToArray();
            input.Murmur32(0).Should().Be(input2.Murmur32(0));
            input.Murmur128(0).Should().Equal(input2.Murmur128(0));
        }

        [TestMethod]
        public void TestSha256()
        {
            byte[] value = Encoding.ASCII.GetBytes("hello world");
            byte[] result = value.Sha256(0, value.Length);
            string resultStr = result.ToHexString();
            resultStr.Should().Be("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
            value.Sha256().Should().Equal(result);
            ((Span<byte>)value).Sha256().Should().Equal(result);
            ((ReadOnlySpan<byte>)value).Sha256().Should().Equal(result);
        }

        [TestMethod]
        public void TestKeccak256()
        {
            var input = "Hello, world!"u8.ToArray();
            var result = input.Keccak256();
            result.ToHexString().Should().Be("b6e16d27ac5ab427a7f68900ac5559ce272dc6c37c82b3e052246c82244c50e4");
            ((Span<byte>)input).Keccak256().Should().Equal(result);
            ((ReadOnlySpan<byte>)input).Keccak256().Should().Equal(result);
        }

        [TestMethod]
        public void TestRIPEMD160()
        {
            ReadOnlySpan<byte> value = Encoding.ASCII.GetBytes("hello world");
            byte[] result = value.RIPEMD160();
            string resultStr = result.ToHexString();
            resultStr.Should().Be("98c615784ccb5fe5936fbc0cbe9dfdb408d92f0f");
        }

        [TestMethod]
        public void TestAESEncryptAndDecrypt()
        {
            XEP6Wallet wallet = new XEP6Wallet("", "1", TestProtocolSettings.Default);
            wallet.CreateAccount();
            WalletAccount account = wallet.GetAccounts().ToArray()[0];
            KeyPair key = account.GetKey();
            Random random = new Random();
            byte[] nonce = new byte[12];
            random.NextBytes(nonce);
            var cypher = EpicChain.Cryptography.Helper.AES256Encrypt(Encoding.UTF8.GetBytes("hello world"), key.PrivateKey, nonce);
            var m = EpicChain.Cryptography.Helper.AES256Decrypt(cypher, key.PrivateKey);
            var message2 = Encoding.UTF8.GetString(m);
            Assert.AreEqual("hello world", message2);
        }

        [TestMethod]
        public void TestEcdhEncryptAndDecrypt()
        {
            XEP6Wallet wallet = new XEP6Wallet("", "1", ProtocolSettings.Default);
            wallet.CreateAccount();
            wallet.CreateAccount();
            WalletAccount account1 = wallet.GetAccounts().ToArray()[0];
            KeyPair key1 = account1.GetKey();
            WalletAccount account2 = wallet.GetAccounts().ToArray()[1];
            KeyPair key2 = account2.GetKey();
            Console.WriteLine($"Account:{1},privatekey:{key1.PrivateKey.ToHexString()},publicKey:{key1.PublicKey.ToArray().ToHexString()}");
            Console.WriteLine($"Account:{2},privatekey:{key2.PrivateKey.ToHexString()},publicKey:{key2.PublicKey.ToArray().ToHexString()}");
            var secret1 = EpicChain.Cryptography.Helper.ECDHDeriveKey(key1, key2.PublicKey);
            var secret2 = EpicChain.Cryptography.Helper.ECDHDeriveKey(key2, key1.PublicKey);
            Assert.AreEqual(secret1.ToHexString(), secret2.ToHexString());
            var message = Encoding.ASCII.GetBytes("hello world");
            Random random = new Random();
            byte[] nonce = new byte[12];
            random.NextBytes(nonce);
            var cypher = message.AES256Encrypt(secret1, nonce);
            cypher.AES256Decrypt(secret2);
            Assert.AreEqual("hello world", Encoding.ASCII.GetString(cypher.AES256Decrypt(secret2)));
        }

        [TestMethod]
        public void TestTest()
        {
            int m = 7, n = 10;
            uint nTweak = 123456;
            BloomFilter filter = new(m, n, nTweak);

            Transaction tx = new()
            {
                Script = TestUtils.GetByteArray(32, 0x42),
                SystemFee = 4200000000,
                Signers = new Signer[] { new Signer() { Account = (Array.Empty<byte>()).ToScriptHash() } },
                Attributes = Array.Empty<TransactionAttribute>(),
                Witnesses = new[]
                {
                    new Witness
                    {
                        InvocationScript = Array.Empty<byte>(),
                        VerificationScript = Array.Empty<byte>()
                    }
                }
            };
            filter.Test(tx).Should().BeFalse();
            filter.Add(tx.Witnesses[0].ScriptHash.ToArray());
            filter.Test(tx).Should().BeTrue();
            filter.Add(tx.Hash.ToArray());
            filter.Test(tx).Should().BeTrue();
        }
    }
}
