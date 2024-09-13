// Copyright (C) 2021-2024 EpicChain Labs.

//
// CommandTokenTest.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Linq;

namespace EpicChain.ConsoleService.Tests
{
    [TestClass]
    public class CommandTokenTest
    {
        [TestMethod]
        public void Test1()
        {
            var cmd = " ";
            var args = CommandToken.Parse(cmd).ToArray();

            AreEqual(args, new CommandSpaceToken(0, 1));
            Assert.AreEqual(cmd, CommandToken.ToString(args));
        }

        [TestMethod]
        public void Test2()
        {
            var cmd = "show  state";
            var args = CommandToken.Parse(cmd).ToArray();

            AreEqual(args, new CommandStringToken(0, "show"), new CommandSpaceToken(4, 2), new CommandStringToken(6, "state"));
            Assert.AreEqual(cmd, CommandToken.ToString(args));
        }

        [TestMethod]
        public void Test3()
        {
            var cmd = "show \"hello world\"";
            var args = CommandToken.Parse(cmd).ToArray();

            AreEqual(args,
                new CommandStringToken(0, "show"),
                new CommandSpaceToken(4, 1),
                new CommandQuoteToken(5, '"'),
                new CommandStringToken(6, "hello world"),
                new CommandQuoteToken(17, '"')
                );
            Assert.AreEqual(cmd, CommandToken.ToString(args));
        }

        [TestMethod]
        public void Test4()
        {
            var cmd = "show \"'\"";
            var args = CommandToken.Parse(cmd).ToArray();

            AreEqual(args,
                new CommandStringToken(0, "show"),
                new CommandSpaceToken(4, 1),
                new CommandQuoteToken(5, '"'),
                new CommandStringToken(6, "'"),
                new CommandQuoteToken(7, '"')
                );
            Assert.AreEqual(cmd, CommandToken.ToString(args));
        }

        [TestMethod]
        public void Test5()
        {
            var cmd = "show \"123\\\"456\"";
            var args = CommandToken.Parse(cmd).ToArray();

            AreEqual(args,
                new CommandStringToken(0, "show"),
                new CommandSpaceToken(4, 1),
                new CommandQuoteToken(5, '"'),
                new CommandStringToken(6, "123\\\"456"),
                new CommandQuoteToken(14, '"')
                );
            Assert.AreEqual(cmd, CommandToken.ToString(args));
        }

        private void AreEqual(CommandToken[] args, params CommandToken[] compare)
        {
            Assert.AreEqual(compare.Length, args.Length);

            for (int x = 0; x < args.Length; x++)
            {
                var a = args[x];
                var b = compare[x];

                Assert.AreEqual(a.Type, b.Type);
                Assert.AreEqual(a.Value, b.Value);
                Assert.AreEqual(a.Offset, b.Offset);
            }
        }
    }
}
