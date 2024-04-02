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
// UT_NEP6Contract.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.Json;
using Neo.SmartContract;
using Neo.Wallets.NEP6;
using System;

namespace Neo.UnitTests.Wallets.NEP6
{
    [TestClass]
    public class UT_NEP6Contract
    {
        [TestMethod]
        public void TestFromNullJson()
        {
            NEP6Contract nep6Contract = NEP6Contract.FromJson(null);
            nep6Contract.Should().BeNull();
        }

        [TestMethod]
        public void TestFromJson()
        {
            string json = "{\"script\":\"IQPviR30wLfu+5N9IeoPuIzejg2Cp/8RhytecEeWna+062h0dHaq\"," +
                "\"parameters\":[{\"name\":\"signature\",\"type\":\"Signature\"}],\"deployed\":false}";
            JObject @object = (JObject)JToken.Parse(json);

            NEP6Contract nep6Contract = NEP6Contract.FromJson(@object);
            nep6Contract.Script.Should().BeEquivalentTo("2103ef891df4c0b7eefb937d21ea0fb88cde8e0d82a7ff11872b5e7047969dafb4eb68747476aa".HexToBytes());
            nep6Contract.ParameterList.Length.Should().Be(1);
            nep6Contract.ParameterList[0].Should().Be(ContractParameterType.Signature);
            nep6Contract.ParameterNames.Length.Should().Be(1);
            nep6Contract.ParameterNames[0].Should().Be("signature");
            nep6Contract.Deployed.Should().BeFalse();
        }

        [TestMethod]
        public void TestToJson()
        {
            NEP6Contract nep6Contract = new()
            {
                Script = new byte[] { 0x00, 0x01 },
                ParameterList = new ContractParameterType[] { ContractParameterType.Boolean, ContractParameterType.Integer },
                ParameterNames = new string[] { "param1", "param2" },
                Deployed = false
            };

            JObject @object = nep6Contract.ToJson();
            JString jString = (JString)@object["script"];
            jString.Value.Should().Be(Convert.ToBase64String(nep6Contract.Script, Base64FormattingOptions.None));

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
