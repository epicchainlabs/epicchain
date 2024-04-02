// EpicChain Copyright Project (2021-2024)
// 
// Copyright (c) 2021-2024 EpicChain
// 
// EpicChain is an innovative blockchain network developed and maintained by xmoohad. This copyright project outlines the rights and responsibilities associated with the EpicChain software and its related components.
// 
// 1. Copyright Holder:
//    - xmoohad
// 
// 2. Project Name:
//    - EpicChain
// 
// 3. Project Description:
//    - EpicChain is a decentralized blockchain network that aims to revolutionize the way digital assets are managed, traded, and secured. With its innovative features and robust architecture, EpicChain provides a secure and efficient platform for various decentralized applications (dApps) and digital asset management.
// 
// 4. Copyright Period:
//    - The copyright for the EpicChain software and its related components is valid from 2021 to 2024.
// 
// 5. Copyright Statement:
//    - All rights reserved. No part of the EpicChain software or its related components may be reproduced, distributed, or transmitted in any form or by any means, without the prior written permission of the copyright holder, except in the case of brief quotations embodied in critical reviews and certain other noncommercial uses permitted by copyright law.
// 
// 6. License:
//    - The EpicChain software is licensed under the EpicChain Software License, a custom license that governs the use, distribution, and modification of the software. The EpicChain Software License is designed to promote the free and open development of the EpicChain network while protecting the interests of the copyright holder.
// 
// 7. Open Source:
//    - EpicChain is an open-source project, and its source code is available to the public under the terms of the EpicChain Software License. Developers are encouraged to contribute to the development of EpicChain and create innovative applications on top of the EpicChain network.
// 
// 8. Disclaimer:
//    - The EpicChain software and its related components are provided "as is," without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall the copyright holder or contributors be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the EpicChain software or its related components.
// 
// 9. Contact Information:
//    - For inquiries regarding the EpicChain copyright project, please contact xmoohad at [email address].
// 
// 10. Updates:
//     - This copyright project may be updated or modified from time to time to reflect changes in the EpicChain project or to address new legal or regulatory requirements. Users and developers are encouraged to check the latest version of the copyright project periodically.

//
// UT_NEP6Account.cs file belongs to the neo project and is free
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
using Neo.Wallets.NEP6;

namespace Neo.UnitTests.Wallets.NEP6
{
    [TestClass]
    public class UT_NEP6Account
    {
        NEP6Account _account;
        UInt160 _hash;
        NEP6Wallet _wallet;
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
            _account = new NEP6Account(_wallet, _hash);
        }

        [TestMethod]
        public void TestChangePassword()
        {
            _account = new NEP6Account(_wallet, _hash, _nep2);
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
            _account.ChangePasswordRoolback();
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
            NEP6Account account = new(wallet, hash, _keyPair, password);
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
            NEP6Account account = NEP6Account.FromJson(json, _wallet);
            account.ScriptHash.Should().Be("NdtB8RXRmJ7Nhw1FPTm7E6HoDZGnDw37nf".ToScriptHash(TestProtocolSettings.Default.AddressVersion));
            account.Label.Should().BeNull();
            account.IsDefault.Should().BeTrue();
            account.Lock.Should().BeFalse();
            account.Contract.Should().BeNull();
            account.Extra.Should().BeNull();
            account.GetKey().Should().BeNull();

            json["key"] = "6PYRjVE1gAbCRyv81FTiFz62cxuPGw91vMjN4yPa68bnoqJtioreTznezn";
            json["label"] = "label";
            account = NEP6Account.FromJson(json, _wallet);
            account.Label.Should().Be("label");
            account.HasKey.Should().BeTrue();
        }

        [TestMethod]
        public void TestGetKey()
        {
            _account.GetKey().Should().BeNull();
            _account = new NEP6Account(_wallet, _hash, _nep2);
            _account.GetKey().Should().Be(_keyPair);
        }

        [TestMethod]
        public void TestGetKeyWithString()
        {
            _account.GetKey("Satoshi").Should().BeNull();
            _account = new NEP6Account(_wallet, _hash, _nep2);
            _account.GetKey("Satoshi").Should().Be(_keyPair);
        }

        [TestMethod]
        public void TestToJson()
        {
            JObject nep6contract = new();
            nep6contract["script"] = "IQNgPziA63rqCtRQCJOSXkpC/qSKRO5viYoQs8fOBdKiZ6w=";
            JObject parameters = new();
            parameters["type"] = 0x00;
            parameters["name"] = "Sig";
            JArray array = new()
            {
                parameters
            };
            nep6contract["parameters"] = array;
            nep6contract["deployed"] = false;
            _account.Contract = NEP6Contract.FromJson(nep6contract);
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
            _account = new NEP6Account(_wallet, _hash, _nep2);
            _account.VerifyPassword("Satoshi").Should().BeTrue();
            _account.VerifyPassword("b").Should().BeFalse();
        }
    }
}
