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
// UT_JObject.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

namespace Neo.Json.UnitTests
{
    [TestClass]
    public class UT_JObject
    {
        private JObject alice;
        private JObject bob;

        [TestInitialize]
        public void SetUp()
        {
            alice = new JObject();
            alice["name"] = "alice";
            alice["age"] = 30;
            alice["score"] = 100.001;
            alice["gender"] = Foo.female;
            alice["isMarried"] = true;
            var pet1 = new JObject();
            pet1["name"] = "Tom";
            pet1["type"] = "cat";
            alice["pet"] = pet1;

            bob = new JObject();
            bob["name"] = "bob";
            bob["age"] = 100000;
            bob["score"] = 0.001;
            bob["gender"] = Foo.male;
            bob["isMarried"] = false;
            var pet2 = new JObject();
            pet2["name"] = "Paul";
            pet2["type"] = "dog";
            bob["pet"] = pet2;
        }

        [TestMethod]
        public void TestAsBoolean()
        {
            alice.AsBoolean().Should().BeTrue();
        }

        [TestMethod]
        public void TestAsNumber()
        {
            alice.AsNumber().Should().Be(double.NaN);
        }

        [TestMethod]
        public void TestParse()
        {
            Assert.ThrowsException<ArgumentOutOfRangeException>(() => JObject.Parse("", -1));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("aaa"));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("hello world"));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("100.a"));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("100.+"));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("\"\\s\""));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("\"a"));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("{\"k1\":\"v1\",\"k1\":\"v2\"}"));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("{\"k1\",\"k1\"}"));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("{\"k1\":\"v1\""));
            Assert.ThrowsException<FormatException>(() => JObject.Parse(new byte[] { 0x22, 0x01, 0x22 }));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("{\"color\":\"red\",\"\\uDBFF\\u0DFFF\":\"#f00\"}"));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("{\"color\":\"\\uDBFF\\u0DFFF\"}"));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("\"\\uDBFF\\u0DFFF\""));

            JObject.Parse("null").Should().BeNull();
            JObject.Parse("true").AsBoolean().Should().BeTrue();
            JObject.Parse("false").AsBoolean().Should().BeFalse();
            JObject.Parse("\"hello world\"").AsString().Should().Be("hello world");
            JObject.Parse("\"\\\"\\\\\\/\\b\\f\\n\\r\\t\"").AsString().Should().Be("\"\\/\b\f\n\r\t");
            JObject.Parse("\"\\u0030\"").AsString().Should().Be("0");
            JObject.Parse("{\"k1\":\"v1\"}", 100).ToString().Should().Be("{\"k1\":\"v1\"}");
        }

        [TestMethod]
        public void TestGetEnum()
        {
            alice.AsEnum<Woo>().Should().Be(Woo.Tom);

            Action action = () => alice.GetEnum<Woo>();
            action.Should().Throw<InvalidCastException>();
        }

        [TestMethod]
        public void TestOpImplicitEnum()
        {
            JToken obj = Woo.Tom;
            obj.AsString().Should().Be("Tom");
        }

        [TestMethod]
        public void TestOpImplicitString()
        {
            JToken obj = null;
            obj.Should().BeNull();

            obj = "{\"aaa\":\"111\"}";
            obj.AsString().Should().Be("{\"aaa\":\"111\"}");
        }

        [TestMethod]
        public void TestGetNull()
        {
            JToken.Null.Should().BeNull();
        }

        [TestMethod]
        public void TestClone()
        {
            var bobClone = (JObject)bob.Clone();
            bobClone.Should().NotBeSameAs(bob);
            foreach (var key in bobClone.Properties.Keys)
            {
                switch (bob[key])
                {
                    case JToken.Null:
                        bobClone[key].Should().BeNull();
                        break;
                    case JObject obj:
                        ((JObject)bobClone[key]).Properties.Should().BeEquivalentTo(obj.Properties);
                        break;
                    default:
                        bobClone[key].Should().BeEquivalentTo(bob[key]);
                        break;
                }
            }
        }
    }
}
