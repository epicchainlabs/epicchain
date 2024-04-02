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
// UT_EvaluationStack.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.Test.Extensions;
using Neo.Test.Helpers;
using Neo.VM;
using Neo.VM.Types;
using System;
using System.Collections;
using System.Linq;

namespace Neo.Test
{
    [TestClass]
    public class UT_EvaluationStack
    {
        private static EvaluationStack CreateOrderedStack(int count)
        {
            var check = new Integer[count];
            var stack = new EvaluationStack(new ReferenceCounter());

            for (int x = 1; x <= count; x++)
            {
                stack.Push(x);
                check[x - 1] = x;
            }

            Assert.AreEqual(count, stack.Count);
            CollectionAssert.AreEqual(check, stack.ToArray());

            return stack;
        }

        public static IEnumerable GetEnumerable(IEnumerator enumerator)
        {
            while (enumerator.MoveNext()) yield return enumerator.Current;
        }

        [TestMethod]
        public void TestClear()
        {
            var stack = CreateOrderedStack(3);
            stack.Clear();
            Assert.AreEqual(0, stack.Count);
        }

        [TestMethod]
        public void TestCopyTo()
        {
            var stack = CreateOrderedStack(3);
            var copy = new EvaluationStack(new ReferenceCounter());

            Assert.ThrowsException<ArgumentOutOfRangeException>(() => stack.CopyTo(copy, -2));
            Assert.ThrowsException<ArgumentOutOfRangeException>(() => stack.CopyTo(copy, 4));

            stack.CopyTo(copy, 0);

            Assert.AreEqual(3, stack.Count);
            Assert.AreEqual(0, copy.Count);
            CollectionAssert.AreEqual(new Integer[] { 1, 2, 3 }, stack.ToArray());

            stack.CopyTo(copy, -1);

            Assert.AreEqual(3, stack.Count);
            Assert.AreEqual(3, copy.Count);
            CollectionAssert.AreEqual(new Integer[] { 1, 2, 3 }, stack.ToArray());

            // Test IEnumerable

            var enumerable = (IEnumerable)copy;
            var enumerator = enumerable.GetEnumerator();

            CollectionAssert.AreEqual(new Integer[] { 1, 2, 3 }, GetEnumerable(enumerator).Cast<Integer>().ToArray());

            copy.CopyTo(stack, 2);

            Assert.AreEqual(5, stack.Count);
            Assert.AreEqual(3, copy.Count);

            CollectionAssert.AreEqual(new Integer[] { 1, 2, 3, 2, 3 }, stack.ToArray());
            CollectionAssert.AreEqual(new Integer[] { 1, 2, 3 }, copy.ToArray());
        }

        [TestMethod]
        public void TestMoveTo()
        {
            var stack = CreateOrderedStack(3);
            var other = new EvaluationStack(new ReferenceCounter());

            stack.MoveTo(other, 0);

            Assert.AreEqual(3, stack.Count);
            Assert.AreEqual(0, other.Count);
            CollectionAssert.AreEqual(new Integer[] { 1, 2, 3 }, stack.ToArray());

            stack.MoveTo(other, -1);

            Assert.AreEqual(0, stack.Count);
            Assert.AreEqual(3, other.Count);
            CollectionAssert.AreEqual(new Integer[] { 1, 2, 3 }, other.ToArray());

            // Test IEnumerable

            var enumerable = (IEnumerable)other;
            var enumerator = enumerable.GetEnumerator();

            CollectionAssert.AreEqual(new Integer[] { 1, 2, 3 }, GetEnumerable(enumerator).Cast<Integer>().ToArray());

            other.MoveTo(stack, 2);

            Assert.AreEqual(2, stack.Count);
            Assert.AreEqual(1, other.Count);

            CollectionAssert.AreEqual(new Integer[] { 2, 3 }, stack.ToArray());
            CollectionAssert.AreEqual(new Integer[] { 1 }, other.ToArray());
        }

        [TestMethod]
        public void TestInsertPeek()
        {
            var stack = new EvaluationStack(new ReferenceCounter());

            stack.Insert(0, 3);
            stack.Insert(1, 1);
            stack.Insert(1, 2);

            Assert.ThrowsException<InvalidOperationException>(() => stack.Insert(4, 2));

            Assert.AreEqual(3, stack.Count);
            CollectionAssert.AreEqual(new Integer[] { 1, 2, 3 }, stack.ToArray());

            Assert.AreEqual(3, stack.Peek(0));
            Assert.AreEqual(2, stack.Peek(1));
            Assert.AreEqual(1, stack.Peek(-1));

            Assert.ThrowsException<InvalidOperationException>(() => stack.Peek(-4));
        }

        [TestMethod]
        public void TestPopPush()
        {
            var stack = CreateOrderedStack(3);

            Assert.AreEqual(3, stack.Pop());
            Assert.AreEqual(2, stack.Pop());
            Assert.AreEqual(1, stack.Pop());

            Assert.ThrowsException<ArgumentOutOfRangeException>(() => stack.Pop());

            stack = CreateOrderedStack(3);

            Assert.IsTrue(stack.Pop<Integer>().Equals(3));
            Assert.IsTrue(stack.Pop<Integer>().Equals(2));
            Assert.IsTrue(stack.Pop<Integer>().Equals(1));

            Assert.ThrowsException<ArgumentOutOfRangeException>(() => stack.Pop<Integer>());
        }

        [TestMethod]
        public void TestRemove()
        {
            var stack = CreateOrderedStack(3);

            Assert.IsTrue(stack.Remove<Integer>(0).Equals(3));
            Assert.IsTrue(stack.Remove<Integer>(0).Equals(2));
            Assert.IsTrue(stack.Remove<Integer>(-1).Equals(1));

            Assert.ThrowsException<ArgumentOutOfRangeException>(() => stack.Remove<Integer>(0));
            Assert.ThrowsException<ArgumentOutOfRangeException>(() => stack.Remove<Integer>(-1));
        }

        [TestMethod]
        public void TestReverse()
        {
            var stack = CreateOrderedStack(3);

            stack.Reverse(3);
            Assert.IsTrue(stack.Pop<Integer>().Equals(1));
            Assert.IsTrue(stack.Pop<Integer>().Equals(2));
            Assert.IsTrue(stack.Pop<Integer>().Equals(3));
            Assert.ThrowsException<ArgumentOutOfRangeException>(() => stack.Pop<Integer>().Equals(0));

            stack = CreateOrderedStack(3);

            Assert.ThrowsException<ArgumentOutOfRangeException>(() => stack.Reverse(-1));
            Assert.ThrowsException<ArgumentOutOfRangeException>(() => stack.Reverse(4));

            stack.Reverse(1);
            Assert.IsTrue(stack.Pop<Integer>().Equals(3));
            Assert.IsTrue(stack.Pop<Integer>().Equals(2));
            Assert.IsTrue(stack.Pop<Integer>().Equals(1));
            Assert.ThrowsException<ArgumentOutOfRangeException>(() => stack.Pop<Integer>().Equals(0));
        }

        [TestMethod]
        public void TestEvaluationStackPrint()
        {
            var stack = new EvaluationStack(new ReferenceCounter());

            stack.Insert(0, 3);
            stack.Insert(1, 1);
            stack.Insert(2, "test");
            stack.Insert(3, true);

            Assert.AreEqual("[Boolean(True), ByteString(\"test\"), Integer(1), Integer(3)]", stack.ToString());
        }

        [TestMethod]
        public void TestPrintInvalidUTF8()
        {
            var stack = new EvaluationStack(new ReferenceCounter());
            stack.Insert(0, "4CC95219999D421243C8161E3FC0F4290C067845".FromHexString());
            Assert.AreEqual("[ByteString(\"Base64: TMlSGZmdQhJDyBYeP8D0KQwGeEU=\")]", stack.ToString());
        }
    }
}
