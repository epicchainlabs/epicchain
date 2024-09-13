// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_XEP6Contract.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Extensions;
using EpicChain.Json;
using EpicChain.SmartContract;
using EpicChain.Wallets.XEP6;
using System;

namespace EpicChain.UnitTests.Wallets.XEP6
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
