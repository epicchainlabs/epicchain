// Copyright (C) 2015-2024 The Neo Project.
//
// UT_XEP6Account.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.Cryptography;
using Neo.Json;
using Neo.SmartContract;
using Neo.Wallets;
using Neo.Wallets.XEP6;

namespace Neo.UnitTests.Wallets.XEP6
{
    [TestClass]
    public class UT_XEP6Account
    {
        XEP6Account _account;
        UInt160 _hash;
        XEP6Wallet _wallet;
        private static string _nep2;
        private static KeyPair _keyPair;

        [ClassInitialize]
        public static void ClassSetup(TestContext ctx)
        {
            byte[] privateKey = { 0x01,0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01};
            _keyPair = new KeyPair(privateKey);
            _nep2 = _keyPair.Export("Satoshi", TestProtocolSettings.Default.AddressVersion, 2, 1, 1);
        }

        [TestInitialize]
        public void TestSetup()
        {
            _wallet = TestUtils.GenerateTestWallet("Satoshi");
            byte[] array1 = { 0x01 };
            _hash = new UInt160(Crypto.Hash160(array1));
            _account = new XEP6Account(_wallet, _hash);
        }

        [TestMethod]
        public void TestChangePassword()
        {
            _account = new XEP6Account(_wallet, _hash, _nep2);
            _account.ChangePasswordPrepare("b", "Satoshi").Should().BeTrue();
            _account.ChangePasswordCommit();
            _account.Contract = new Contract();
            _account.ChangePasswordPrepare("b", "Satoshi").Should().BeFalse();
            _account.ChangePasswordPrepare("Satoshi", "b").Should().BeTrue();
            _account.ChangePasswordCommit();
            _account.VerifyPassword("b").Should().BeTrue();
            _account.ChangePasswordPrepare("b", "Satoshi").Should().BeTrue();
            _account.ChangePasswordCommit();
            _account.ChangePasswordPrepare("Satoshi", "b").Should().BeTrue();
            _account.ChangePasswordRollback();
            _account.VerifyPassword("Satoshi").Should().BeTrue();
        }

        [TestMethod]
        public void TestConstructorWithNep2Key()
        {
            _account.ScriptHash.Should().Be(_hash);
            _account.Decrypted.Should().BeTrue();
            _account.HasKey.Should().BeFalse();
        }

        [TestMethod]
        public void TestConstructorWithKeyPair()
        {
            string password = "hello world";
            var wallet = TestUtils.GenerateTestWallet(password);
            byte[] array1 = { 0x01 };
            var hash = new UInt160(Crypto.Hash160(array1));
            XEP6Account account = new(wallet, hash, _keyPair, password);
            account.ScriptHash.Should().Be(hash);
            account.Decrypted.Should().BeTrue();
            account.HasKey.Should().BeTrue();
        }

        [TestMethod]
        public void TestFromJson()
        {
            JObject json = new();
            json["address"] = "NdtB8RXRmJ7Nhw1FPTm7E6HoDZGnDw37nf";
            json["key"] = null;
            json["label"] = null;
            json["isDefault"] = true;
            json["lock"] = false;
            json["contract"] = null;
            json["extra"] = null;
            XEP6Account account = XEP6Account.FromJson(json, _wallet);
            account.ScriptHash.Should().Be("NdtB8RXRmJ7Nhw1FPTm7E6HoDZGnDw37nf".ToScriptHash(TestProtocolSettings.Default.AddressVersion));
            account.Label.Should().BeNull();
            account.IsDefault.Should().BeTrue();
            account.Lock.Should().BeFalse();
            account.Contract.Should().BeNull();
            account.Extra.Should().BeNull();
            account.GetKey().Should().BeNull();

            json["key"] = "6PYRjVE1gAbCRyv81FTiFz62cxuPGw91vMjN4yPa68bnoqJtioreTznezn";
            json["label"] = "label";
            account = XEP6Account.FromJson(json, _wallet);
            account.Label.Should().Be("label");
            account.HasKey.Should().BeTrue();
        }

        [TestMethod]
        public void TestGetKey()
        {
            _account.GetKey().Should().BeNull();
            _account = new XEP6Account(_wallet, _hash, _nep2);
            _account.GetKey().Should().Be(_keyPair);
        }

        [TestMethod]
        public void TestGetKeyWithString()
        {
            _account.GetKey("Satoshi").Should().BeNull();
            _account = new XEP6Account(_wallet, _hash, _nep2);
            _account.GetKey("Satoshi").Should().Be(_keyPair);
        }

        [TestMethod]
        public void TestToJson()
        {
            JObject xep6contract = new();
            xep6contract["script"] = "IQNgPziA63rqCtRQCJOSXkpC/qSKRO5viYoQs8fOBdKiZ6w=";
            JObject parameters = new();
            parameters["type"] = 0x00;
            parameters["name"] = "Sig";
            JArray array = new()
            {
                parameters
            };
            xep6contract["parameters"] = array;
            xep6contract["deployed"] = false;
            _account.Contract = XEP6Contract.FromJson(xep6contract);
            JObject json = _account.ToJson();
            json["address"].AsString().Should().Be("NdtB8RXRmJ7Nhw1FPTm7E6HoDZGnDw37nf");
            json["label"].Should().BeNull();
            json["isDefault"].ToString().Should().Be("false");
            json["lock"].ToString().Should().Be("false");
            json["key"].Should().BeNull();
            json["contract"]["script"].ToString().Should().Be(@"""IQNgPziA63rqCtRQCJOSXkpC/qSKRO5viYoQs8fOBdKiZ6w=""");
            json["extra"].Should().BeNull();

            _account.Contract = null;
            json = _account.ToJson();
            json["contract"].Should().BeNull();
        }

        [TestMethod]
        public void TestVerifyPassword()
        {
            _account = new XEP6Account(_wallet, _hash, _nep2);
            _account.VerifyPassword("Satoshi").Should().BeTrue();
            _account.VerifyPassword("b").Should().BeFalse();
        }
    }
}