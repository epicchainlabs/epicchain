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
// UT_IndexedQueue.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.IO.Caching;
using System;
using System.Linq;

namespace Neo.UnitTests.IO.Caching
{
    [TestClass]
    public class UT_IndexedQueue
    {
        [TestMethod]
        public void TestDefault()
        {
            var queue = new IndexedQueue<int>(10);
            queue.Count.Should().Be(0);

            queue = new IndexedQueue<int>();
            queue.Count.Should().Be(0);
            queue.TrimExcess();
            queue.Count.Should().Be(0);

            queue = new IndexedQueue<int>(Array.Empty<int>());
            queue.Count.Should().Be(0);
            queue.TryPeek(out var a).Should().BeFalse();
            a.Should().Be(0);
            queue.TryDequeue(out a).Should().BeFalse();
            a.Should().Be(0);

            Assert.ThrowsException<InvalidOperationException>(() => queue.Peek());
            Assert.ThrowsException<InvalidOperationException>(() => queue.Dequeue());
            Assert.ThrowsException<IndexOutOfRangeException>(() => _ = queue[-1]);
            Assert.ThrowsException<IndexOutOfRangeException>(() => queue[-1] = 1);
            Assert.ThrowsException<IndexOutOfRangeException>(() => _ = queue[1]);
            Assert.ThrowsException<IndexOutOfRangeException>(() => queue[1] = 1);
            Assert.ThrowsException<ArgumentOutOfRangeException>(() => new IndexedQueue<int>(-1));
        }

        [TestMethod]
        public void TestQueue()
        {
            var queue = new IndexedQueue<int>(new int[] { 1, 2, 3 });
            queue.Count.Should().Be(3);

            queue.Enqueue(4);
            queue.Count.Should().Be(4);
            queue.Peek().Should().Be(1);
            queue.TryPeek(out var a).Should().BeTrue();
            a.Should().Be(1);

            queue[0].Should().Be(1);
            queue[1].Should().Be(2);
            queue[2].Should().Be(3);
            queue.Dequeue().Should().Be(1);
            queue.Dequeue().Should().Be(2);
            queue.Dequeue().Should().Be(3);
            queue[0] = 5;
            queue.TryDequeue(out a).Should().BeTrue();
            a.Should().Be(5);

            queue.Enqueue(4);
            queue.Clear();
            queue.Count.Should().Be(0);
        }

        [TestMethod]
        public void TestEnumerator()
        {
            int[] arr = new int[3] { 1, 2, 3 };
            var queue = new IndexedQueue<int>(arr);

            arr.SequenceEqual(queue).Should().BeTrue();
        }

        [TestMethod]
        public void TestCopyTo()
        {
            int[] arr = new int[3];
            var queue = new IndexedQueue<int>(new int[] { 1, 2, 3 });

            Assert.ThrowsException<ArgumentNullException>(() => queue.CopyTo(null, 0));
            Assert.ThrowsException<ArgumentOutOfRangeException>(() => queue.CopyTo(arr, -1));
            Assert.ThrowsException<ArgumentOutOfRangeException>(() => queue.CopyTo(arr, 2));

            queue.CopyTo(arr, 0);

            arr[0].Should().Be(1);
            arr[1].Should().Be(2);
            arr[2].Should().Be(3);

            arr = queue.ToArray();

            arr[0].Should().Be(1);
            arr[1].Should().Be(2);
            arr[2].Should().Be(3);
        }
    }
}
