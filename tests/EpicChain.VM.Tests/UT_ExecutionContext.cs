// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_ExecutionContext.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System;
using System.Collections.Generic;

namespace EpicChain.Test
{
    [TestClass]
    public class UT_ExecutionContext
    {
        class TestState
        {
            public bool Flag = false;
        }

        [TestMethod]
        public void TestStateTest()
        {
            var context = new ExecutionContext(Array.Empty<byte>(), -1, new ReferenceCounter());

            // Test factory

            var flag = context.GetState(() => new TestState() { Flag = true });
            Assert.IsTrue(flag.Flag);

            flag.Flag = false;

            flag = context.GetState(() => new TestState() { Flag = true });
            Assert.IsFalse(flag.Flag);

            // Test new

            var stack = context.GetState<Stack<int>>();
            Assert.AreEqual(0, stack.Count);
            stack.Push(100);
            stack = context.GetState<Stack<int>>();
            Assert.AreEqual(100, stack.Pop());
            stack.Push(100);

            // Test clone

            var copy = context.Clone();
            var copyStack = copy.GetState<Stack<int>>();
            Assert.AreEqual(1, copyStack.Count);
            copyStack.Push(200);
            copyStack = context.GetState<Stack<int>>();
            Assert.AreEqual(200, copyStack.Pop());
            Assert.AreEqual(100, copyStack.Pop());
            copyStack.Push(200);

            stack = context.GetState<Stack<int>>();
            Assert.AreEqual(200, stack.Pop());
        }
    }
}
