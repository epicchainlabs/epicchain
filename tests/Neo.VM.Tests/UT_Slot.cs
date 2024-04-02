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
// UT_Slot.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.VM;
using Neo.VM.Types;
using System;
using System.Collections;
using System.Linq;
using System.Numerics;

namespace Neo.Test
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
