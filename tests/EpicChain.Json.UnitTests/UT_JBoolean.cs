// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_JBoolean.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Newtonsoft.Json;

namespace EpicChain.Json.UnitTests
{
    [TestClass]
    public class UT_JBoolean
    {
        private JBoolean jFalse;
        private JBoolean jTrue;

        [TestInitialize]
        public void SetUp()
        {
            jFalse = new JBoolean();
            jTrue = new JBoolean(true);
        }

        [TestMethod]
        public void TestAsNumber()
        {
            jFalse.AsNumber().Should().Be(0);
            jTrue.AsNumber().Should().Be(1);
        }

        [TestMethod]
        public void TestDefaultConstructor()
        {
            var defaultJBoolean = new JBoolean();
            defaultJBoolean.AsNumber().Should().Be(0);
        }

        [TestMethod]
        public void TestExplicitFalse()
        {
            var explicitFalse = new JBoolean(false);
            explicitFalse.AsNumber().Should().Be(0);
        }

        [TestMethod]
        public void TestNullJBoolean()
        {
            JBoolean nullJBoolean = null;
            Assert.ThrowsException<NullReferenceException>(() => nullJBoolean.AsNumber());
        }

        [TestMethod]
        public void TestConversionToOtherTypes()
        {
            Assert.AreEqual("true", jTrue.ToString());
            Assert.AreEqual("false", jFalse.ToString());
        }

        [TestMethod]
        public void TestComparisonsWithOtherBooleans()
        {
            Assert.IsTrue(jTrue.Equals(new JBoolean(true)));
            Assert.IsTrue(jFalse.Equals(new JBoolean()));
        }

        [TestMethod]
        public void TestSerializationAndDeserialization()
        {
            string serialized = JsonConvert.SerializeObject(jTrue);
            var deserialized = JsonConvert.DeserializeObject<JBoolean>(serialized);
            Assert.AreEqual(jTrue, deserialized);
        }

        [TestMethod]
        public void TestEqual()
        {
            Assert.IsTrue(jTrue.Equals(new JBoolean(true)));
            Assert.IsTrue(jTrue == new JBoolean(true));
            Assert.IsTrue(jTrue != new JBoolean(false));
            Assert.IsTrue(jFalse.Equals(new JBoolean()));
            Assert.IsTrue(jFalse == new JBoolean());
            Assert.IsTrue(jFalse.GetBoolean().ToString().ToLowerInvariant() == jFalse.ToString());
        }
    }
}
