// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_WitnessContition.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Cryptography.ECC;
using EpicChain.Json;
using EpicChain.Network.P2P.Payloads.Conditions;

namespace EpicChain.UnitTests.Network.P2P.Payloads
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
