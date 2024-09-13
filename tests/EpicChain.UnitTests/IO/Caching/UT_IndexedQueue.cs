// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_IndexedQueue.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.IO.Caching;
using System;
using System.Linq;

namespace EpicChain.UnitTests.IO.Caching
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
