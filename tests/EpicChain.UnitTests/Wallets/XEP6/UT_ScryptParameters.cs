// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_ScryptParameters.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Wallets.XEP6;

namespace EpicChain.UnitTests.Wallets.XEP6
{
    [TestClass]
    public class UT_ScryptParameters
    {
        ScryptParameters uut;

        [TestInitialize]
        public void TestSetup()
        {
            uut = ScryptParameters.Default;
        }

        [TestMethod]
        public void Test_Default_ScryptParameters()
        {
            uut.N.Should().Be(16384);
            uut.R.Should().Be(8);
            uut.P.Should().Be(8);
        }

        [TestMethod]
        public void Test_ScryptParameters_Default_ToJson()
        {
            JObject json = ScryptParameters.Default.ToJson();
            json["n"].AsNumber().Should().Be(ScryptParameters.Default.N);
            json["r"].AsNumber().Should().Be(ScryptParameters.Default.R);
            json["p"].AsNumber().Should().Be(ScryptParameters.Default.P);
        }

        [TestMethod]
        public void Test_Default_ScryptParameters_FromJson()
        {
            JObject json = new JObject();
            json["n"] = 16384;
            json["r"] = 8;
            json["p"] = 8;

            ScryptParameters uut2 = ScryptParameters.FromJson(json);
            uut2.N.Should().Be(ScryptParameters.Default.N);
            uut2.R.Should().Be(ScryptParameters.Default.R);
            uut2.P.Should().Be(ScryptParameters.Default.P);
        }

        [TestMethod]
        public void TestScryptParametersConstructor()
        {
            int n = 1, r = 2, p = 3;
            ScryptParameters parameter = new ScryptParameters(n, r, p);
            parameter.N.Should().Be(n);
            parameter.R.Should().Be(r);
            parameter.P.Should().Be(p);
        }
    }
}
