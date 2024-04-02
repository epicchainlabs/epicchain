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
// UT_ContractEventAttribute.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.SmartContract;
using Neo.SmartContract.Native;

namespace Neo.UnitTests.SmartContract.Native
{
    [TestClass]
    public class UT_ContractEventAttribute
    {
        [TestMethod]
        public void TestConstructorOneArg()
        {
            var arg = new ContractEventAttribute(Hardfork.HF_Basilisk, 0, "1", "a1", ContractParameterType.String);

            Assert.AreEqual(Hardfork.HF_Basilisk, arg.ActiveIn);
            Assert.AreEqual(0, arg.Order);
            Assert.AreEqual("1", arg.Descriptor.Name);
            Assert.AreEqual(1, arg.Descriptor.Parameters.Length);
            Assert.AreEqual("a1", arg.Descriptor.Parameters[0].Name);
            Assert.AreEqual(ContractParameterType.String, arg.Descriptor.Parameters[0].Type);

            arg = new ContractEventAttribute(1, "1", "a1", ContractParameterType.String);

            Assert.IsNull(arg.ActiveIn);
            Assert.AreEqual(1, arg.Order);
            Assert.AreEqual("1", arg.Descriptor.Name);
            Assert.AreEqual(1, arg.Descriptor.Parameters.Length);
            Assert.AreEqual("a1", arg.Descriptor.Parameters[0].Name);
            Assert.AreEqual(ContractParameterType.String, arg.Descriptor.Parameters[0].Type);
        }

        [TestMethod]
        public void TestConstructorTwoArg()
        {
            var arg = new ContractEventAttribute(Hardfork.HF_Basilisk, 0, "2",
                "a1", ContractParameterType.String,
                "a2", ContractParameterType.Integer);

            Assert.AreEqual(Hardfork.HF_Basilisk, arg.ActiveIn);
            Assert.AreEqual(0, arg.Order);
            Assert.AreEqual("2", arg.Descriptor.Name);
            Assert.AreEqual(2, arg.Descriptor.Parameters.Length);
            Assert.AreEqual("a1", arg.Descriptor.Parameters[0].Name);
            Assert.AreEqual(ContractParameterType.String, arg.Descriptor.Parameters[0].Type);
            Assert.AreEqual("a2", arg.Descriptor.Parameters[1].Name);
            Assert.AreEqual(ContractParameterType.Integer, arg.Descriptor.Parameters[1].Type);

            arg = new ContractEventAttribute(1, "2",
                "a1", ContractParameterType.String,
                "a2", ContractParameterType.Integer);

            Assert.IsNull(arg.ActiveIn);
            Assert.AreEqual(1, arg.Order);
            Assert.AreEqual("2", arg.Descriptor.Name);
            Assert.AreEqual(2, arg.Descriptor.Parameters.Length);
            Assert.AreEqual("a1", arg.Descriptor.Parameters[0].Name);
            Assert.AreEqual(ContractParameterType.String, arg.Descriptor.Parameters[0].Type);
            Assert.AreEqual("a2", arg.Descriptor.Parameters[1].Name);
            Assert.AreEqual(ContractParameterType.Integer, arg.Descriptor.Parameters[1].Type);
        }

        [TestMethod]
        public void TestConstructorThreeArg()
        {
            var arg = new ContractEventAttribute(Hardfork.HF_Basilisk, 0, "3",
                "a1", ContractParameterType.String,
                "a2", ContractParameterType.Integer,
                "a3", ContractParameterType.Boolean);

            Assert.AreEqual(Hardfork.HF_Basilisk, arg.ActiveIn);
            Assert.AreEqual(0, arg.Order);
            Assert.AreEqual("3", arg.Descriptor.Name);
            Assert.AreEqual(3, arg.Descriptor.Parameters.Length);
            Assert.AreEqual("a1", arg.Descriptor.Parameters[0].Name);
            Assert.AreEqual(ContractParameterType.String, arg.Descriptor.Parameters[0].Type);
            Assert.AreEqual("a2", arg.Descriptor.Parameters[1].Name);
            Assert.AreEqual(ContractParameterType.Integer, arg.Descriptor.Parameters[1].Type);
            Assert.AreEqual("a3", arg.Descriptor.Parameters[2].Name);
            Assert.AreEqual(ContractParameterType.Boolean, arg.Descriptor.Parameters[2].Type);

            arg = new ContractEventAttribute(1, "3",
                "a1", ContractParameterType.String,
                "a2", ContractParameterType.Integer,
                "a3", ContractParameterType.Boolean);

            Assert.IsNull(arg.ActiveIn);
            Assert.AreEqual(1, arg.Order);
            Assert.AreEqual("3", arg.Descriptor.Name);
            Assert.AreEqual(3, arg.Descriptor.Parameters.Length);
            Assert.AreEqual("a1", arg.Descriptor.Parameters[0].Name);
            Assert.AreEqual(ContractParameterType.String, arg.Descriptor.Parameters[0].Type);
            Assert.AreEqual("a2", arg.Descriptor.Parameters[1].Name);
            Assert.AreEqual(ContractParameterType.Integer, arg.Descriptor.Parameters[1].Type);
            Assert.AreEqual("a3", arg.Descriptor.Parameters[2].Name);
            Assert.AreEqual(ContractParameterType.Boolean, arg.Descriptor.Parameters[2].Type);
        }

        [TestMethod]
        public void TestConstructorFourArg()
        {
            var arg = new ContractEventAttribute(Hardfork.HF_Basilisk, 0, "4",
                "a1", ContractParameterType.String,
                "a2", ContractParameterType.Integer,
                "a3", ContractParameterType.Boolean,
                "a4", ContractParameterType.Array);

            Assert.AreEqual(Hardfork.HF_Basilisk, arg.ActiveIn);
            Assert.AreEqual(0, arg.Order);
            Assert.AreEqual("4", arg.Descriptor.Name);
            Assert.AreEqual(4, arg.Descriptor.Parameters.Length);
            Assert.AreEqual("a1", arg.Descriptor.Parameters[0].Name);
            Assert.AreEqual(ContractParameterType.String, arg.Descriptor.Parameters[0].Type);
            Assert.AreEqual("a2", arg.Descriptor.Parameters[1].Name);
            Assert.AreEqual(ContractParameterType.Integer, arg.Descriptor.Parameters[1].Type);
            Assert.AreEqual("a3", arg.Descriptor.Parameters[2].Name);
            Assert.AreEqual(ContractParameterType.Boolean, arg.Descriptor.Parameters[2].Type);
            Assert.AreEqual("a4", arg.Descriptor.Parameters[3].Name);
            Assert.AreEqual(ContractParameterType.Array, arg.Descriptor.Parameters[3].Type);

            arg = new ContractEventAttribute(1, "4",
                "a1", ContractParameterType.String,
                "a2", ContractParameterType.Integer,
                "a3", ContractParameterType.Boolean,
                "a4", ContractParameterType.Array);

            Assert.IsNull(arg.ActiveIn);
            Assert.AreEqual(1, arg.Order);
            Assert.AreEqual("4", arg.Descriptor.Name);
            Assert.AreEqual(4, arg.Descriptor.Parameters.Length);
            Assert.AreEqual("a1", arg.Descriptor.Parameters[0].Name);
            Assert.AreEqual(ContractParameterType.String, arg.Descriptor.Parameters[0].Type);
            Assert.AreEqual("a2", arg.Descriptor.Parameters[1].Name);
            Assert.AreEqual(ContractParameterType.Integer, arg.Descriptor.Parameters[1].Type);
            Assert.AreEqual("a3", arg.Descriptor.Parameters[2].Name);
            Assert.AreEqual(ContractParameterType.Boolean, arg.Descriptor.Parameters[2].Type);
            Assert.AreEqual("a4", arg.Descriptor.Parameters[3].Name);
            Assert.AreEqual(ContractParameterType.Array, arg.Descriptor.Parameters[3].Type);
        }
    }
}
