// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_JObject.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


namespace EpicChain.Json.UnitTests
{
    [TestClass]
    public class UT_JObject
    {
        private JObject alice;
        private JObject bob;

        [TestInitialize]
        public void SetUp()
        {
            alice = new JObject();
            alice["name"] = "alice";
            alice["age"] = 30;
            alice["score"] = 100.001;
            alice["gender"] = Foo.female;
            alice["isMarried"] = true;
            var pet1 = new JObject();
            pet1["name"] = "Tom";
            pet1["type"] = "cat";
            alice["pet"] = pet1;

            bob = new JObject();
            bob["name"] = "bob";
            bob["age"] = 100000;
            bob["score"] = 0.001;
            bob["gender"] = Foo.male;
            bob["isMarried"] = false;
            var pet2 = new JObject();
            pet2["name"] = "Paul";
            pet2["type"] = "dog";
            bob["pet"] = pet2;
        }

        [TestMethod]
        public void TestAsBoolean()
        {
            alice.AsBoolean().Should().BeTrue();
        }

        [TestMethod]
        public void TestAsNumber()
        {
            alice.AsNumber().Should().Be(double.NaN);
        }

        [TestMethod]
        public void TestParse()
        {
            Assert.ThrowsException<ArgumentOutOfRangeException>(() => JObject.Parse("", -1));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("aaa"));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("hello world"));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("100.a"));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("100.+"));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("\"\\s\""));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("\"a"));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("{\"k1\":\"v1\",\"k1\":\"v2\"}"));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("{\"k1\",\"k1\"}"));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("{\"k1\":\"v1\""));
            Assert.ThrowsException<FormatException>(() => JObject.Parse(new byte[] { 0x22, 0x01, 0x22 }));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("{\"color\":\"red\",\"\\uDBFF\\u0DFFF\":\"#f00\"}"));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("{\"color\":\"\\uDBFF\\u0DFFF\"}"));
            Assert.ThrowsException<FormatException>(() => JObject.Parse("\"\\uDBFF\\u0DFFF\""));

            JObject.Parse("null").Should().BeNull();
            JObject.Parse("true").AsBoolean().Should().BeTrue();
            JObject.Parse("false").AsBoolean().Should().BeFalse();
            JObject.Parse("\"hello world\"").AsString().Should().Be("hello world");
            JObject.Parse("\"\\\"\\\\\\/\\b\\f\\n\\r\\t\"").AsString().Should().Be("\"\\/\b\f\n\r\t");
            JObject.Parse("\"\\u0030\"").AsString().Should().Be("0");
            JObject.Parse("{\"k1\":\"v1\"}", 100).ToString().Should().Be("{\"k1\":\"v1\"}");
        }

        [TestMethod]
        public void TestGetEnum()
        {
            alice.AsEnum<Woo>().Should().Be(Woo.Tom);

            Action action = () => alice.GetEnum<Woo>();
            action.Should().Throw<InvalidCastException>();
        }

        [TestMethod]
        public void TestOpImplicitEnum()
        {
            JToken obj = Woo.Tom;
            obj.AsString().Should().Be("Tom");
        }

        [TestMethod]
        public void TestOpImplicitString()
        {
            JToken obj = null;
            obj.Should().BeNull();

            obj = "{\"aaa\":\"111\"}";
            obj.AsString().Should().Be("{\"aaa\":\"111\"}");
        }

        [TestMethod]
        public void TestGetNull()
        {
            JToken.Null.Should().BeNull();
        }

        [TestMethod]
        public void TestClone()
        {
            var bobClone = (JObject)bob.Clone();
            bobClone.Should().NotBeSameAs(bob);
            foreach (var key in bobClone.Properties.Keys)
            {
                switch (bob[key])
                {
                    case JToken.Null:
                        bobClone[key].Should().BeNull();
                        break;
                    case JObject obj:
                        ((JObject)bobClone[key]).Properties.Should().BeEquivalentTo(obj.Properties);
                        break;
                    default:
                        bobClone[key].Should().BeEquivalentTo(bob[key]);
                        break;
                }
            }
        }
    }
}
