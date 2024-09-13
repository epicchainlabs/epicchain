// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_WildCardContainer.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Json;
using EpicChain.SmartContract.Manifest;
using System;
using System.Collections;
using System.Collections.Generic;

namespace EpicChain.UnitTests.SmartContract.Manifest
{
    [TestClass]
    public class UT_WildCardContainer
    {
        [TestMethod]
        public void TestFromJson()
        {
            JString jstring = new JString("*");
            WildcardContainer<string> s = WildcardContainer<string>.FromJson(jstring, u => u.AsString());
            s.Should().BeEmpty();

            jstring = new JString("hello world");
            Action action = () => WildcardContainer<string>.FromJson(jstring, u => u.AsString());
            action.Should().Throw<FormatException>();

            JObject alice = new JObject();
            alice["name"] = "alice";
            alice["age"] = 30;
            JArray jarray = new JArray { alice };
            WildcardContainer<string> r = WildcardContainer<string>.FromJson(jarray, u => u.AsString());
            r[0].Should().Be("{\"name\":\"alice\",\"age\":30}");

            JBoolean jbool = new JBoolean();
            action = () => WildcardContainer<string>.FromJson(jbool, u => u.AsString());
            action.Should().Throw<FormatException>();
        }

        [TestMethod]
        public void TestGetCount()
        {
            string[] s = new string[] { "hello", "world" };
            WildcardContainer<string> container = WildcardContainer<string>.Create(s);
            container.Count.Should().Be(2);

            s = null;
            container = WildcardContainer<string>.Create(s);
            container.Count.Should().Be(0);
        }

        [TestMethod]
        public void TestGetItem()
        {
            string[] s = new string[] { "hello", "world" };
            WildcardContainer<string> container = WildcardContainer<string>.Create(s);
            container[0].Should().Be("hello");
            container[1].Should().Be("world");
        }

        [TestMethod]
        public void TestGetEnumerator()
        {
            string[] s = null;
            IReadOnlyList<string> rs = new string[0];
            WildcardContainer<string> container = WildcardContainer<string>.Create(s);
            IEnumerator<string> enumerator = container.GetEnumerator();
            enumerator.Should().Be(rs.GetEnumerator());

            s = new string[] { "hello", "world" };
            container = WildcardContainer<string>.Create(s);
            enumerator = container.GetEnumerator();
            foreach (string _ in s)
            {
                enumerator.MoveNext();
                enumerator.Current.Should().Be(_);
            }
        }

        [TestMethod]
        public void TestIEnumerableGetEnumerator()
        {
            string[] s = new string[] { "hello", "world" };
            WildcardContainer<string> container = WildcardContainer<string>.Create(s);
            IEnumerable enumerable = container;
            var enumerator = enumerable.GetEnumerator();
            foreach (string _ in s)
            {
                enumerator.MoveNext();
                enumerator.Current.Should().Be(_);
            }
        }
    }
}
