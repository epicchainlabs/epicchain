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
// UT_JPath.cs file belongs to the neo project and is free
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
    public class UT_JPath
    {
        private static readonly JObject json = new()
        {
            ["store"] = new JObject
            {
                ["book"] = new JArray
                {
                    new JObject
                    {
                        ["category"] = "reference",
                        ["author"] = "Nigel Rees",
                        ["title"] = "Sayings of the Century",
                        ["price"] = 8.95
                    },
                    new JObject
                    {
                        ["category"] = "fiction",
                        ["author"] = "Evelyn Waugh",
                        ["title"] = "Sword of Honour",
                        ["price"] = 12.99
                    },
                    new JObject
                    {
                        ["category"] = "fiction",
                        ["author"] = "Herman Melville",
                        ["title"] = "Moby Dick",
                        ["isbn"] = "0-553-21311-3",
                        ["price"] = 8.99
                    },
                    new JObject
                    {
                        ["category"] = "fiction",
                        ["author"] = "J. R. R. Tolkien",
                        ["title"] = "The Lord of the Rings",
                        ["isbn"] = "0-395-19395-8",
                        ["price"] = null
                    }
                },
                ["bicycle"] = new JObject
                {
                    ["color"] = "red",
                    ["price"] = 19.95
                }
            },
            ["expensive"] = 10,
            ["data"] = null,
        };

        [TestMethod]
        public void TestOOM()
        {
            var filter = "$" + string.Concat(Enumerable.Repeat("[0" + string.Concat(Enumerable.Repeat(",0", 64)) + "]", 6));
            Assert.ThrowsException<InvalidOperationException>(() => JObject.Parse("[[[[[[{}]]]]]]")!.JsonPath(filter));
        }

        [TestMethod]
        public void TestJsonPath()
        {
            Assert.AreEqual(@"[""Nigel Rees"",""Evelyn Waugh"",""Herman Melville"",""J. R. R. Tolkien""]", json.JsonPath("$.store.book[*].author").ToString());
            Assert.AreEqual(@"[""Nigel Rees"",""Evelyn Waugh"",""Herman Melville"",""J. R. R. Tolkien""]", json.JsonPath("$..author").ToString());
            Assert.AreEqual(@"[[{""category"":""reference"",""author"":""Nigel Rees"",""title"":""Sayings of the Century"",""price"":8.95},{""category"":""fiction"",""author"":""Evelyn Waugh"",""title"":""Sword of Honour"",""price"":12.99},{""category"":""fiction"",""author"":""Herman Melville"",""title"":""Moby Dick"",""isbn"":""0-553-21311-3"",""price"":8.99},{""category"":""fiction"",""author"":""J. R. R. Tolkien"",""title"":""The Lord of the Rings"",""isbn"":""0-395-19395-8"",""price"":null}],{""color"":""red"",""price"":19.95}]", json.JsonPath("$.store.*").ToString());
            Assert.AreEqual(@"[19.95,8.95,12.99,8.99,null]", json.JsonPath("$.store..price").ToString());
            Assert.AreEqual(@"[{""category"":""fiction"",""author"":""Herman Melville"",""title"":""Moby Dick"",""isbn"":""0-553-21311-3"",""price"":8.99}]", json.JsonPath("$..book[2]").ToString());
            Assert.AreEqual(@"[{""category"":""fiction"",""author"":""Herman Melville"",""title"":""Moby Dick"",""isbn"":""0-553-21311-3"",""price"":8.99}]", json.JsonPath("$..book[-2]").ToString());
            Assert.AreEqual(@"[{""category"":""reference"",""author"":""Nigel Rees"",""title"":""Sayings of the Century"",""price"":8.95},{""category"":""fiction"",""author"":""Evelyn Waugh"",""title"":""Sword of Honour"",""price"":12.99}]", json.JsonPath("$..book[0,1]").ToString());
            Assert.AreEqual(@"[{""category"":""reference"",""author"":""Nigel Rees"",""title"":""Sayings of the Century"",""price"":8.95},{""category"":""fiction"",""author"":""Evelyn Waugh"",""title"":""Sword of Honour"",""price"":12.99}]", json.JsonPath("$..book[:2]").ToString());
            Assert.AreEqual(@"[{""category"":""fiction"",""author"":""Evelyn Waugh"",""title"":""Sword of Honour"",""price"":12.99}]", json.JsonPath("$..book[1:2]").ToString());
            Assert.AreEqual(@"[{""category"":""fiction"",""author"":""Herman Melville"",""title"":""Moby Dick"",""isbn"":""0-553-21311-3"",""price"":8.99},{""category"":""fiction"",""author"":""J. R. R. Tolkien"",""title"":""The Lord of the Rings"",""isbn"":""0-395-19395-8"",""price"":null}]", json.JsonPath("$..book[-2:]").ToString());
            Assert.AreEqual(@"[{""category"":""fiction"",""author"":""Herman Melville"",""title"":""Moby Dick"",""isbn"":""0-553-21311-3"",""price"":8.99},{""category"":""fiction"",""author"":""J. R. R. Tolkien"",""title"":""The Lord of the Rings"",""isbn"":""0-395-19395-8"",""price"":null}]", json.JsonPath("$..book[2:]").ToString());
            Assert.AreEqual(@"[{""store"":{""book"":[{""category"":""reference"",""author"":""Nigel Rees"",""title"":""Sayings of the Century"",""price"":8.95},{""category"":""fiction"",""author"":""Evelyn Waugh"",""title"":""Sword of Honour"",""price"":12.99},{""category"":""fiction"",""author"":""Herman Melville"",""title"":""Moby Dick"",""isbn"":""0-553-21311-3"",""price"":8.99},{""category"":""fiction"",""author"":""J. R. R. Tolkien"",""title"":""The Lord of the Rings"",""isbn"":""0-395-19395-8"",""price"":null}],""bicycle"":{""color"":""red"",""price"":19.95}},""expensive"":10,""data"":null}]", json.JsonPath("").ToString());
            Assert.AreEqual(@"[{""book"":[{""category"":""reference"",""author"":""Nigel Rees"",""title"":""Sayings of the Century"",""price"":8.95},{""category"":""fiction"",""author"":""Evelyn Waugh"",""title"":""Sword of Honour"",""price"":12.99},{""category"":""fiction"",""author"":""Herman Melville"",""title"":""Moby Dick"",""isbn"":""0-553-21311-3"",""price"":8.99},{""category"":""fiction"",""author"":""J. R. R. Tolkien"",""title"":""The Lord of the Rings"",""isbn"":""0-395-19395-8"",""price"":null}],""bicycle"":{""color"":""red"",""price"":19.95}},10,null]", json.JsonPath("$.*").ToString());
            Assert.AreEqual(@"[]", json.JsonPath("$..invalidfield").ToString());
        }

        [TestMethod]
        public void TestMaxDepth()
        {
            Assert.ThrowsException<InvalidOperationException>(() => json.JsonPath("$..book[*].author"));
        }

        [TestMethod]
        public void TestInvalidFormat()
        {
            Assert.ThrowsException<FormatException>(() => json.JsonPath("$..*"));
            Assert.ThrowsException<FormatException>(() => json.JsonPath("..book"));
            Assert.ThrowsException<FormatException>(() => json.JsonPath("$.."));
        }
    }
}
