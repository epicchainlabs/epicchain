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
// UT_JArray.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using System.Collections;

namespace Neo.Json.UnitTests
{
    enum Foo
    {
        male,
        female
    }

    [TestClass]
    public class UT_JArray
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
        public void TestAdd()
        {
            var jArray = new JArray
            {
                alice,
                bob
            };
            var jAlice = jArray[0];
            var jBob = jArray[1];
            jAlice["name"].ToString().Should().Be(alice["name"].ToString());
            jAlice["age"].ToString().Should().Be(alice["age"].ToString());
            jAlice["score"].ToString().Should().Be(alice["score"].ToString());
            jAlice["gender"].ToString().Should().Be(alice["gender"].ToString());
            jAlice["isMarried"].ToString().Should().Be(alice["isMarried"].ToString());
            jAlice["pet"].ToString().Should().Be(alice["pet"].ToString());
            jBob["name"].ToString().Should().Be(bob["name"].ToString());
            jBob["age"].ToString().Should().Be(bob["age"].ToString());
            jBob["score"].ToString().Should().Be(bob["score"].ToString());
            jBob["gender"].ToString().Should().Be(bob["gender"].ToString());
            jBob["isMarried"].ToString().Should().Be(bob["isMarried"].ToString());
            jBob["pet"].ToString().Should().Be(bob["pet"].ToString());
        }

        [TestMethod]
        public void TestSetItem()
        {
            var jArray = new JArray
            {
                alice
            };
            jArray[0] = bob;
            Assert.AreEqual(jArray[0], bob);

            Action action = () => jArray[1] = alice;
            action.Should().Throw<ArgumentOutOfRangeException>();
        }

        [TestMethod]
        public void TestClear()
        {
            var jArray = new JArray
            {
                alice
            };
            var jAlice = jArray[0];
            jAlice["name"].ToString().Should().Be(alice["name"].ToString());
            jAlice["age"].ToString().Should().Be(alice["age"].ToString());
            jAlice["score"].ToString().Should().Be(alice["score"].ToString());
            jAlice["gender"].ToString().Should().Be(alice["gender"].ToString());
            jAlice["isMarried"].ToString().Should().Be(alice["isMarried"].ToString());
            jAlice["pet"].ToString().Should().Be(alice["pet"].ToString());

            jArray.Clear();
            Action action = () => jArray[0].ToString();
            action.Should().Throw<ArgumentOutOfRangeException>();
        }

        [TestMethod]
        public void TestContains()
        {
            var jArray = new JArray
            {
                alice
            };
            jArray.Contains(alice).Should().BeTrue();
            jArray.Contains(bob).Should().BeFalse();
        }

        [TestMethod]
        public void TestCopyTo()
        {
            var jArray = new JArray
            {
                alice,
                bob
            };

            JObject[] jObjects1 = new JObject[2];
            jArray.CopyTo(jObjects1, 0);
            var jAlice1 = jObjects1[0];
            var jBob1 = jObjects1[1];
            Assert.AreEqual(alice, jAlice1);
            Assert.AreEqual(bob, jBob1);

            JObject[] jObjects2 = new JObject[4];
            jArray.CopyTo(jObjects2, 2);
            var jAlice2 = jObjects2[2];
            var jBob2 = jObjects2[3];
            jObjects2[0].Should().BeNull();
            jObjects2[1].Should().BeNull();
            Assert.AreEqual(alice, jAlice2);
            Assert.AreEqual(bob, jBob2);
        }

        [TestMethod]
        public void TestInsert()
        {
            var jArray = new JArray
            {
                alice,
                alice,
                alice,
                alice
            };

            jArray.Insert(1, bob);
            jArray.Count().Should().Be(5);
            jArray[0].Should().Be(alice);
            jArray[1].Should().Be(bob);
            jArray[2].Should().Be(alice);

            jArray.Insert(5, bob);
            jArray.Count().Should().Be(6);
            jArray[5].Should().Be(bob);
        }

        [TestMethod]
        public void TestIndexOf()
        {
            var jArray = new JArray();
            jArray.IndexOf(alice).Should().Be(-1);

            jArray.Add(alice);
            jArray.Add(alice);
            jArray.Add(alice);
            jArray.Add(alice);
            jArray.IndexOf(alice).Should().Be(0);

            jArray.Insert(1, bob);
            jArray.IndexOf(bob).Should().Be(1);
        }

        [TestMethod]
        public void TestIsReadOnly()
        {
            var jArray = new JArray();
            jArray.IsReadOnly.Should().BeFalse();
        }

        [TestMethod]
        public void TestRemove()
        {
            var jArray = new JArray
            {
                alice
            };
            jArray.Count().Should().Be(1);
            jArray.Remove(alice);
            jArray.Count().Should().Be(0);

            jArray.Add(alice);
            jArray.Add(alice);
            jArray.Count().Should().Be(2);
            jArray.Remove(alice);
            jArray.Count().Should().Be(1);
        }

        [TestMethod]
        public void TestRemoveAt()
        {
            var jArray = new JArray
            {
                alice,
                bob,
                alice
            };
            jArray.RemoveAt(1);
            jArray.Count().Should().Be(2);
            jArray.Contains(bob).Should().BeFalse();
        }

        [TestMethod]
        public void TestGetEnumerator()
        {
            var jArray = new JArray
            {
                alice,
                bob,
                alice,
                bob
            };
            int i = 0;
            foreach (var item in jArray)
            {
                if (i % 2 == 0) item.Should().Be(alice);
                if (i % 2 != 0) item.Should().Be(bob);
                i++;
            }
            Assert.IsNotNull(((IEnumerable)jArray).GetEnumerator());
        }

        [TestMethod]
        public void TestAsString()
        {
            var jArray = new JArray
            {
                alice,
                bob,
            };
            var s = jArray.AsString();
            Assert.AreEqual(s, "{\"name\":\"alice\",\"age\":30,\"score\":100.001,\"gender\":\"female\",\"isMarried\":true,\"pet\":{\"name\":\"Tom\",\"type\":\"cat\"}},{\"name\":\"bob\",\"age\":100000,\"score\":0.001,\"gender\":\"male\",\"isMarried\":false,\"pet\":{\"name\":\"Paul\",\"type\":\"dog\"}}");
        }
    }
}
