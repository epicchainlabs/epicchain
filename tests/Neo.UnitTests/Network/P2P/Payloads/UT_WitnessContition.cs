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
// UT_WitnessContition.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.Cryptography.ECC;
using Neo.Json;
using Neo.Network.P2P.Payloads.Conditions;

namespace Neo.UnitTests.Network.P2P.Payloads
{
    [TestClass]
    public class UT_WitnessCondition
    {
        [TestMethod]
        public void TestFromJson1()
        {
            var point = ECPoint.Parse("03b209fd4f53a7170ea4444e0cb0a6bb6a53c2bd016926989cf85f9b0fba17a70c", ECCurve.Secp256r1);
            var hash = UInt160.Zero;
            var condition = new OrCondition
            {
                Expressions = new WitnessCondition[]
                {
                    new CalledByContractCondition { Hash = hash },
                    new CalledByGroupCondition { Group = point }
                }
            };
            var json = condition.ToJson();
            var new_condi = WitnessCondition.FromJson(json, 2);
            Assert.IsTrue(new_condi is OrCondition);
            var or_condi = (OrCondition)new_condi;
            Assert.AreEqual(2, or_condi.Expressions.Length);
            Assert.IsTrue(or_condi.Expressions[0] is CalledByContractCondition);
            var cbcc = (CalledByContractCondition)(or_condi.Expressions[0]);
            Assert.IsTrue(or_condi.Expressions[1] is CalledByGroupCondition);
            var cbgc = (CalledByGroupCondition)(or_condi.Expressions[1]);
            Assert.IsTrue(cbcc.Hash.Equals(hash));
            Assert.IsTrue(cbgc.Group.Equals(point));
        }

        [TestMethod]
        public void TestFromJson2()
        {
            var point = ECPoint.Parse("03b209fd4f53a7170ea4444e0cb0a6bb6a53c2bd016926989cf85f9b0fba17a70c", ECCurve.Secp256r1);
            var hash1 = UInt160.Zero;
            var hash2 = UInt160.Parse("0xd2a4cff31913016155e38e474a2c06d08be276cf");
            var jstr = "{\"type\":\"Or\",\"expressions\":[{\"type\":\"And\",\"expressions\":[{\"type\":\"CalledByContract\",\"hash\":\"0x0000000000000000000000000000000000000000\"},{\"type\":\"ScriptHash\",\"hash\":\"0xd2a4cff31913016155e38e474a2c06d08be276cf\"}]},{\"type\":\"Or\",\"expressions\":[{\"type\":\"CalledByGroup\",\"group\":\"03b209fd4f53a7170ea4444e0cb0a6bb6a53c2bd016926989cf85f9b0fba17a70c\"},{\"type\":\"Boolean\",\"expression\":true}]}]}";
            var json = (JObject)JToken.Parse(jstr);
            var condi = WitnessCondition.FromJson(json, 2);
            var or_condi = (OrCondition)condi;
            Assert.AreEqual(2, or_condi.Expressions.Length);
            var and_condi = (AndCondition)or_condi.Expressions[0];
            var or_condi1 = (OrCondition)or_condi.Expressions[1];
            Assert.AreEqual(2, and_condi.Expressions.Length);
            Assert.AreEqual(2, or_condi1.Expressions.Length);
            var cbcc = (CalledByContractCondition)and_condi.Expressions[0];
            var cbsc = (ScriptHashCondition)and_condi.Expressions[1];
            Assert.IsTrue(cbcc.Hash.Equals(hash1));
            Assert.IsTrue(cbsc.Hash.Equals(hash2));
            var cbgc = (CalledByGroupCondition)or_condi1.Expressions[0];
            var bc = (BooleanCondition)or_condi1.Expressions[1];
            Assert.IsTrue(cbgc.Group.Equals(point));
            Assert.IsTrue(bc.Expression);
        }
    }
}
