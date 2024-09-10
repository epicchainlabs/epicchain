// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_DateTimeExtensions.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System;

namespace Neo.Extensions.Tests
{
    [TestClass]
    public class UT_DateTimeExtensions
    {
        private static readonly DateTime unixEpoch = new(1970, 1, 1, 0, 0, 0, DateTimeKind.Utc);

        [TestMethod]
        public void TestToTimestamp()
        {
            var time = DateTime.Now;
            var expected = (uint)(time.ToUniversalTime() - unixEpoch).TotalSeconds;
            var actual = time.ToTimestamp();

            actual.Should().Be(expected);
        }

        [TestMethod]
        public void TestToTimestampMS()
        {
            var time = DateTime.Now;
            var expected = (ulong)(time.ToUniversalTime() - unixEpoch).TotalMilliseconds;
            var actual = time.ToTimestampMS();

            actual.Should().Be(expected);
        }
    }
}
