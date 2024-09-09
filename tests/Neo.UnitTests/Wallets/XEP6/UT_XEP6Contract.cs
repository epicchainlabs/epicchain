// Copyright (C) 2015-2024 The Neo Project.
//
// UT_XEP6Contract.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.Extensions;
using Neo.Json;
using Neo.SmartContract;
using Neo.Wallets.XEP6;
using System;

namespace Neo.UnitTests.Wallets.XEP6
{
    [TestClass]
    public class UT_XEP6Contract
    {
        [TestMethod]
        public void TestFromNullJson()
        {
            XEP6Contract XEP6Contract = XEP6Contract.FromJson(null);
            XEP6Contract.Should().BeNull();
        }

        [TestMethod]
        public void TestFromJson()
        {
            string json = "{\"script\":\"IQPviR30wLfu+5N9IeoPuIzejg2Cp/8RhytecEeWna+062h0dHaq\"," +
                "\"parameters\":[{\"name\":\"signature\",\"type\":\"Signature\"}],\"deployed\":false}";
            JObject @object = (JObject)JToken.Parse(json);

            XEP6Contract XEP6Contract = XEP6Contract.FromJson(@object);
            XEP6Contract.Script.Should().BeEquivalentTo("2103ef891df4c0b7eefb937d21ea0fb88cde8e0d82a7ff11872b5e7047969dafb4eb68747476aa".HexToBytes());
            XEP6Contract.ParameterList.Length.Should().Be(1);
            XEP6Contract.ParameterList[0].Should().Be(ContractParameterType.Signature);
            XEP6Contract.ParameterNames.Length.Should().Be(1);
            XEP6Contract.ParameterNames[0].Should().Be("signature");
            XEP6Contract.Deployed.Should().BeFalse();
        }

        [TestMethod]
        public void TestToJson()
        {
            XEP6Contract XEP6Contract = new()
            {
                Script = new byte[] { 0x00, 0x01 },
                ParameterList = new ContractParameterType[] { ContractParameterType.Boolean, ContractParameterType.Integer },
                ParameterNames = new string[] { "param1", "param2" },
                Deployed = false
            };

            JObject @object = XEP6Contract.ToJson();
            JString jString = (JString)@object["script"];
            jString.Value.Should().Be(Convert.ToBase64String(XEP6Contract.Script, Base64FormattingOptions.None));

            JBoolean jBoolean = (JBoolean)@object["deployed"];
            jBoolean.Value.Should().BeFalse();

            JArray parameters = (JArray)@object["parameters"];
            parameters.Count.Should().Be(2);

            jString = (JString)parameters[0]["name"];
            jString.Value.Should().Be("param1");
            jString = (JString)parameters[0]["type"];
            jString.Value.Should().Be(ContractParameterType.Boolean.ToString());

            jString = (JString)parameters[1]["name"];
            jString.Value.Should().Be("param2");
            jString = (JString)parameters[1]["type"];
            jString.Value.Should().Be(ContractParameterType.Integer.ToString());
        }
    }
}
