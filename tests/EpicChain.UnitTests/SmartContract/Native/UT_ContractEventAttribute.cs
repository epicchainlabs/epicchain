// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_ContractEventAttribute.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;

namespace EpicChain.UnitTests.SmartContract.Native
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
