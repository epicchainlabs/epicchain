// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_HashSetExtensions.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.IO.Caching;
using System.Collections.Generic;
using System.Linq;

namespace EpicChain.Extensions.Tests.Collections
{
    [TestClass]
    public class UT_HashSetExtensions
    {
        [TestMethod]
        public void TestRemoveHashsetDictionary()
        {
            var a = new HashSet<int>
            {
                1,
                2,
                3
            };

            var b = new Dictionary<int, object>
            {
                [2] = null
            };

            a.Remove(b);

            CollectionAssert.AreEqual(new int[] { 1, 3 }, a.ToArray());

            b[4] = null;
            b[5] = null;
            b[1] = null;
            a.Remove(b);

            CollectionAssert.AreEqual(new int[] { 3 }, a.ToArray());
        }

        [TestMethod]
        public void TestRemoveHashsetSet()
        {
            var a = new HashSet<int>
            {
                1,
                2,
                3
            };

            var b = new SortedSet<int>()
            {
                2
            };

            a.Remove(b);

            CollectionAssert.AreEqual(new int[] { 1, 3 }, a.ToArray());

            b.Add(4);
            b.Add(5);
            b.Add(1);
            a.Remove(b);

            CollectionAssert.AreEqual(new int[] { 3 }, a.ToArray());
        }
    }
}
