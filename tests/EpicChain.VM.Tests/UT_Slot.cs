// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_Slot.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System;
using System.Collections;
using System.Linq;
using System.Numerics;

namespace EpicChain.Test
{
    [TestClass]
    public class UT_Slot
    {
        private static Slot CreateOrderedSlot(int count)
        {
            var check = new Integer[count];

            for (int x = 1; x <= count; x++)
            {
                check[x - 1] = x;
            }

            var slot = new Slot(check, new ReferenceCounter());

            Assert.AreEqual(count, slot.Count);
            CollectionAssert.AreEqual(check, slot.ToArray());

            return slot;
        }

        public static IEnumerable GetEnumerable(IEnumerator enumerator)
        {
            while (enumerator.MoveNext()) yield return enumerator.Current;
        }

        [TestMethod]
        public void TestGet()
        {
            var slot = CreateOrderedSlot(3);

            Assert.IsTrue(slot[0] is Integer item0 && item0.Equals(1));
            Assert.IsTrue(slot[1] is Integer item1 && item1.Equals(2));
            Assert.IsTrue(slot[2] is Integer item2 && item2.Equals(3));
            Assert.ThrowsException<IndexOutOfRangeException>(() => slot[3] is Integer item3);
        }

        [TestMethod]
        public void TestEnumerable()
        {
            var slot = CreateOrderedSlot(3);

            BigInteger i = 1;
            foreach (Integer item in slot)
            {
                Assert.AreEqual(item.GetInteger(), i);
                i++;
            }

            CollectionAssert.AreEqual(new Integer[] { 1, 2, 3 }, slot.ToArray());

            // Test IEnumerable

            var enumerable = (IEnumerable)slot;
            var enumerator = enumerable.GetEnumerator();

            CollectionAssert.AreEqual(new Integer[] { 1, 2, 3 }, GetEnumerable(enumerator).Cast<Integer>().ToArray());

            Assert.AreEqual(3, slot.Count);

            CollectionAssert.AreEqual(new Integer[] { 1, 2, 3 }, slot.ToArray());

            // Empty

            slot = CreateOrderedSlot(0);

            CollectionAssert.AreEqual(System.Array.Empty<Integer>(), slot.ToArray());

            // Test IEnumerable

            enumerable = slot;
            enumerator = enumerable.GetEnumerator();

            CollectionAssert.AreEqual(System.Array.Empty<Integer>(), GetEnumerable(enumerator).Cast<Integer>().ToArray());

            Assert.AreEqual(0, slot.Count);

            CollectionAssert.AreEqual(System.Array.Empty<Integer>(), slot.ToArray());
        }
    }
}
