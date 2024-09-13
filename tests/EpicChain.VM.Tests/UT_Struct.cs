// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_Struct.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

namespace EpicChain.Test
{
    [TestClass]
    public class UT_Struct
    {
        private readonly Struct @struct;

        public UT_Struct()
        {
            @struct = new Struct { 1 };
            for (int i = 0; i < 20000; i++)
                @struct = new Struct { @struct };
        }

        [TestMethod]
        public void TestClone()
        {
            Struct s1 = new() { 1, new Struct { 2 } };
            Struct s2 = s1.Clone(ExecutionEngineLimits.Default);
            s1[0] = 3;
            Assert.AreEqual(1, s2[0]);
            ((Struct)s1[1])[0] = 3;
            Assert.AreEqual(2, ((Struct)s2[1])[0]);
            Assert.ThrowsException<InvalidOperationException>(() => @struct.Clone(ExecutionEngineLimits.Default));
        }

        [TestMethod]
        public void TestEquals()
        {
            Struct s1 = new() { 1, new Struct { 2 } };
            Struct s2 = new() { 1, new Struct { 2 } };
            Assert.IsTrue(s1.Equals(s2, ExecutionEngineLimits.Default));
            Struct s3 = new() { 1, new Struct { 3 } };
            Assert.IsFalse(s1.Equals(s3, ExecutionEngineLimits.Default));
            Assert.ThrowsException<InvalidOperationException>(() => @struct.Equals(@struct.Clone(ExecutionEngineLimits.Default), ExecutionEngineLimits.Default));
        }

        [TestMethod]
        public void TestEqualsDos()
        {
            string payloadStr = new string('h', 65535);
            Struct s1 = new();
            Struct s2 = new();
            for (int i = 0; i < 2; i++)
            {
                s1.Add(payloadStr);
                s2.Add(payloadStr);
            }
            Assert.ThrowsException<InvalidOperationException>(() => s1.Equals(s2, ExecutionEngineLimits.Default));

            for (int i = 0; i < 1000; i++)
            {
                s1.Add(payloadStr);
                s2.Add(payloadStr);
            }
            Assert.ThrowsException<InvalidOperationException>(() => s1.Equals(s2, ExecutionEngineLimits.Default));
        }
    }
}
