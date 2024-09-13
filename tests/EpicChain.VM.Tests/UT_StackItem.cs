// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_StackItem.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.VM;
using EpicChain.VM.Types;
using System.Numerics;

namespace EpicChain.Test
{
    [TestClass]
    public class UT_StackItem
    {
        [TestMethod]
        public void TestHashCode()
        {
            StackItem itemA = "EpicChain";
            StackItem itemB = "EpicChain";
            StackItem itemC = "Next Generation Ecosystem";

            Assert.IsTrue(itemA.GetHashCode() == itemB.GetHashCode());
            Assert.IsTrue(itemA.GetHashCode() != itemC.GetHashCode());

            itemA = new VM.Types.Buffer(1);
            itemB = new VM.Types.Buffer(1);

            Assert.IsTrue(itemA.GetHashCode() != itemB.GetHashCode());

            itemA = true;
            itemB = true;
            itemC = false;

            Assert.IsTrue(itemA.GetHashCode() == itemB.GetHashCode());
            Assert.IsTrue(itemA.GetHashCode() != itemC.GetHashCode());

            itemA = 1;
            itemB = 1;
            itemC = 123;

            Assert.IsTrue(itemA.GetHashCode() == itemB.GetHashCode());
            Assert.IsTrue(itemA.GetHashCode() != itemC.GetHashCode());

            itemA = new Null();
            itemB = new Null();

            Assert.IsTrue(itemA.GetHashCode() == itemB.GetHashCode());

            itemA = new VM.Types.Array();

            Assert.ThrowsException<System.NotSupportedException>(() => itemA.GetHashCode());

            itemA = new Struct();

            Assert.ThrowsException<System.NotSupportedException>(() => itemA.GetHashCode());

            itemA = new Map();

            Assert.ThrowsException<System.NotSupportedException>(() => itemA.GetHashCode());

            itemA = new InteropInterface(123);
            itemB = new InteropInterface(123);

            Assert.IsTrue(itemA.GetHashCode() == itemB.GetHashCode());

            var script = new Script(System.Array.Empty<byte>());
            itemA = new Pointer(script, 123);
            itemB = new Pointer(script, 123);
            itemC = new Pointer(script, 1234);

            Assert.IsTrue(itemA.GetHashCode() == itemB.GetHashCode());
            Assert.IsTrue(itemA.GetHashCode() != itemC.GetHashCode());
        }

        [TestMethod]
        public void TestNull()
        {
            StackItem nullItem = System.Array.Empty<byte>();
            Assert.AreNotEqual(StackItem.Null, nullItem);

            nullItem = new Null();
            Assert.AreEqual(StackItem.Null, nullItem);
        }

        [TestMethod]
        public void TestEqual()
        {
            StackItem itemA = "EpicChain";
            StackItem itemB = "EpicChain";
            StackItem itemC = "Next Generation Ecosystem";
            StackItem itemD = "Next Generation Ecosystem";
            StackItem itemE = "Next Generation Ecosystem";

            Assert.IsTrue(itemA.Equals(itemB));
            Assert.IsFalse(itemA.Equals(itemC));
            Assert.IsFalse(itemC.Equals(itemD));
            Assert.IsFalse(itemD.Equals(itemE));
            Assert.IsFalse(itemA.Equals(new object()));
        }

        [TestMethod]
        public void TestCast()
        {
            // Signed byte

            StackItem item = sbyte.MaxValue;

            Assert.IsInstanceOfType(item, typeof(Integer));
            Assert.AreEqual(new BigInteger(sbyte.MaxValue), ((Integer)item).GetInteger());

            // Unsigned byte

            item = byte.MaxValue;

            Assert.IsInstanceOfType(item, typeof(Integer));
            Assert.AreEqual(new BigInteger(byte.MaxValue), ((Integer)item).GetInteger());

            // Signed short

            item = short.MaxValue;

            Assert.IsInstanceOfType(item, typeof(Integer));
            Assert.AreEqual(new BigInteger(short.MaxValue), ((Integer)item).GetInteger());

            // Unsigned short

            item = ushort.MaxValue;

            Assert.IsInstanceOfType(item, typeof(Integer));
            Assert.AreEqual(new BigInteger(ushort.MaxValue), ((Integer)item).GetInteger());

            // Signed integer

            item = int.MaxValue;

            Assert.IsInstanceOfType(item, typeof(Integer));
            Assert.AreEqual(new BigInteger(int.MaxValue), ((Integer)item).GetInteger());

            // Unsigned integer

            item = uint.MaxValue;

            Assert.IsInstanceOfType(item, typeof(Integer));
            Assert.AreEqual(new BigInteger(uint.MaxValue), ((Integer)item).GetInteger());

            // Signed long

            item = long.MaxValue;

            Assert.IsInstanceOfType(item, typeof(Integer));
            Assert.AreEqual(new BigInteger(long.MaxValue), ((Integer)item).GetInteger());

            // Unsigned long

            item = ulong.MaxValue;

            Assert.IsInstanceOfType(item, typeof(Integer));
            Assert.AreEqual(new BigInteger(ulong.MaxValue), ((Integer)item).GetInteger());

            // BigInteger

            item = BigInteger.MinusOne;

            Assert.IsInstanceOfType(item, typeof(Integer));
            Assert.AreEqual(new BigInteger(-1), ((Integer)item).GetInteger());

            // Boolean

            item = true;

            Assert.IsInstanceOfType(item, typeof(VM.Types.Boolean));
            Assert.IsTrue(item.GetBoolean());

            // ByteString

            item = new byte[] { 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09 };

            Assert.IsInstanceOfType(item, typeof(ByteString));
            CollectionAssert.AreEqual(new byte[] { 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09 }, item.GetSpan().ToArray());
        }

        [TestMethod]
        public void TestDeepCopy()
        {
            Array a = new()
            {
                true,
                1,
                new byte[] { 1 },
                StackItem.Null,
                new Buffer(new byte[] { 1 }),
                new Map { [0] = 1, [2] = 3 },
                new Struct { 1, 2, 3 }
            };
            a.Add(a);
            Array aa = (Array)a.DeepCopy();
            Assert.AreNotEqual(a, aa);
            Assert.AreSame(aa, aa[^1]);
            Assert.IsTrue(a[^2].Equals(aa[^2], ExecutionEngineLimits.Default));
            Assert.AreNotSame(a[^2], aa[^2]);
        }
    }
}
