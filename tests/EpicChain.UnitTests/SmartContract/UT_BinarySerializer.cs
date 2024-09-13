// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_BinarySerializer.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.SmartContract;
using EpicChain.VM;
using EpicChain.VM.Types;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace EpicChain.UnitTests.SmartContract
{
    [TestClass]
    public class UT_BinarySerializer
    {
        [TestMethod]
        public void TestSerialize()
        {
            byte[] result1 = BinarySerializer.Serialize(new byte[5], ExecutionEngineLimits.Default);
            byte[] expectedArray1 = new byte[] {
                        0x28, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00
                    };
            Assert.AreEqual(Encoding.Default.GetString(expectedArray1), Encoding.Default.GetString(result1));

            byte[] result2 = BinarySerializer.Serialize(true, ExecutionEngineLimits.Default);
            byte[] expectedArray2 = new byte[] {
                        0x20, 0x01
                    };
            Assert.AreEqual(Encoding.Default.GetString(expectedArray2), Encoding.Default.GetString(result2));

            byte[] result3 = BinarySerializer.Serialize(1, ExecutionEngineLimits.Default);
            byte[] expectedArray3 = new byte[] {
                        0x21, 0x01, 0x01
                    };
            Assert.AreEqual(Encoding.Default.GetString(expectedArray3), Encoding.Default.GetString(result3));

            StackItem stackItem4 = new InteropInterface(new object());
            Action action4 = () => BinarySerializer.Serialize(stackItem4, ExecutionEngineLimits.Default);
            action4.Should().Throw<NotSupportedException>();

            List<StackItem> list6 = new List<StackItem> { 1 };
            StackItem stackItem62 = new VM.Types.Array(list6);
            byte[] result6 = BinarySerializer.Serialize(stackItem62, ExecutionEngineLimits.Default);
            byte[] expectedArray6 = new byte[] {
                        0x40,0x01,0x21,0x01,0x01
                    };
            Assert.AreEqual(Encoding.Default.GetString(expectedArray6), Encoding.Default.GetString(result6));

            List<StackItem> list7 = new List<StackItem> { 1 };
            StackItem stackItem72 = new Struct(list7);
            byte[] result7 = BinarySerializer.Serialize(stackItem72, ExecutionEngineLimits.Default);
            byte[] expectedArray7 = new byte[] {
                        0x41,0x01,0x21,0x01,0x01
                    };
            Assert.AreEqual(Encoding.Default.GetString(expectedArray7), Encoding.Default.GetString(result7));

            StackItem stackItem82 = new Map { [2] = 1 };
            byte[] result8 = BinarySerializer.Serialize(stackItem82, ExecutionEngineLimits.Default);
            byte[] expectedArray8 = new byte[] {
                        0x48,0x01,0x21,0x01,0x02,0x21,0x01,0x01
                    };
            Assert.AreEqual(Encoding.Default.GetString(expectedArray8), Encoding.Default.GetString(result8));

            Map stackItem91 = new Map();
            stackItem91[1] = stackItem91;
            Action action9 = () => BinarySerializer.Serialize(stackItem91, ExecutionEngineLimits.Default);
            action9.Should().Throw<NotSupportedException>();

            VM.Types.Array stackItem10 = new VM.Types.Array();
            stackItem10.Add(stackItem10);
            Action action10 = () => BinarySerializer.Serialize(stackItem10, ExecutionEngineLimits.Default);
            action10.Should().Throw<NotSupportedException>();
        }

        [TestMethod]
        public void TestDeserializeStackItem()
        {
            StackItem stackItem1 = new ByteString(new byte[5]);
            byte[] byteArray1 = BinarySerializer.Serialize(stackItem1, ExecutionEngineLimits.Default);
            StackItem result1 = BinarySerializer.Deserialize(byteArray1, ExecutionEngineLimits.Default);
            Assert.AreEqual(stackItem1, result1);

            StackItem stackItem2 = StackItem.True;
            byte[] byteArray2 = BinarySerializer.Serialize(stackItem2, ExecutionEngineLimits.Default);
            StackItem result2 = BinarySerializer.Deserialize(byteArray2, ExecutionEngineLimits.Default);
            Assert.AreEqual(stackItem2, result2);

            StackItem stackItem3 = new Integer(1);
            byte[] byteArray3 = BinarySerializer.Serialize(stackItem3, ExecutionEngineLimits.Default);
            StackItem result3 = BinarySerializer.Deserialize(byteArray3, ExecutionEngineLimits.Default);
            Assert.AreEqual(stackItem3, result3);

            byte[] byteArray4 = BinarySerializer.Serialize(1, ExecutionEngineLimits.Default);
            byteArray4[0] = 0x40;
            Action action4 = () => BinarySerializer.Deserialize(byteArray4, ExecutionEngineLimits.Default);
            action4.Should().Throw<FormatException>();

            List<StackItem> list5 = new List<StackItem> { 1 };
            StackItem stackItem52 = new VM.Types.Array(list5);
            byte[] byteArray5 = BinarySerializer.Serialize(stackItem52, ExecutionEngineLimits.Default);
            StackItem result5 = BinarySerializer.Deserialize(byteArray5, ExecutionEngineLimits.Default);
            Assert.AreEqual(((VM.Types.Array)stackItem52).Count, ((VM.Types.Array)result5).Count);
            Assert.AreEqual(((VM.Types.Array)stackItem52).GetEnumerator().Current, ((VM.Types.Array)result5).GetEnumerator().Current);

            List<StackItem> list6 = new List<StackItem> { 1 };
            StackItem stackItem62 = new Struct(list6);
            byte[] byteArray6 = BinarySerializer.Serialize(stackItem62, ExecutionEngineLimits.Default);
            StackItem result6 = BinarySerializer.Deserialize(byteArray6, ExecutionEngineLimits.Default);
            Assert.AreEqual(((Struct)stackItem62).Count, ((Struct)result6).Count);
            Assert.AreEqual(((Struct)stackItem62).GetEnumerator().Current, ((Struct)result6).GetEnumerator().Current);

            StackItem stackItem72 = new Map { [2] = 1 };
            byte[] byteArray7 = BinarySerializer.Serialize(stackItem72, ExecutionEngineLimits.Default);
            StackItem result7 = BinarySerializer.Deserialize(byteArray7, ExecutionEngineLimits.Default);
            Assert.AreEqual(((Map)stackItem72).Count, ((Map)result7).Count);
            CollectionAssert.AreEqual(((Map)stackItem72).Keys.ToArray(), ((Map)result7).Keys.ToArray());
            CollectionAssert.AreEqual(((Map)stackItem72).Values.ToArray(), ((Map)result7).Values.ToArray());
        }
    }
}
