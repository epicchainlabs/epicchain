// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_VMJson.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Test.Extensions;
using EpicChain.Test.Types;
using System;
using System.IO;
using System.Text;

namespace EpicChain.Test
{
    [TestClass]
    public class UT_VMJson : VMJsonTestBase
    {
        [TestMethod]
        public void TestOthers() => TestJson("./Tests/Others");

        [TestMethod]
        public void TestOpCodesArrays() => TestJson("./Tests/OpCodes/Arrays");

        [TestMethod]
        public void TestOpCodesStack() => TestJson("./Tests/OpCodes/Stack");

        [TestMethod]
        public void TestOpCodesSlot() => TestJson("./Tests/OpCodes/Slot");

        [TestMethod]
        public void TestOpCodesSplice() => TestJson("./Tests/OpCodes/Splice");

        [TestMethod]
        public void TestOpCodesControl() => TestJson("./Tests/OpCodes/Control");

        [TestMethod]
        public void TestOpCodesPush() => TestJson("./Tests/OpCodes/Push");

        [TestMethod]
        public void TestOpCodesArithmetic() => TestJson("./Tests/OpCodes/Arithmetic");

        [TestMethod]
        public void TestOpCodesBitwiseLogic() => TestJson("./Tests/OpCodes/BitwiseLogic");

        [TestMethod]
        public void TestOpCodesTypes() => TestJson("./Tests/OpCodes/Types");

        private void TestJson(string path)
        {
            foreach (var file in Directory.GetFiles(path, "*.json", SearchOption.AllDirectories))
            {
                Console.WriteLine($"Processing file '{file}'");

                var realFile = Path.GetFullPath(file);
                var json = File.ReadAllText(realFile, Encoding.UTF8);
                var ut = json.DeserializeJson<VMUT>();

                Assert.IsFalse(string.IsNullOrEmpty(ut.Name), "Name is required");

                if (json != ut.ToJson().Replace("\r\n", "\n"))
                {
                    // Format json

                    Console.WriteLine($"The file '{realFile}' was optimized");
                    //File.WriteAllText(realFile, ut.ToJson().Replace("\r\n", "\n"), Encoding.UTF8);
                }

                try
                {
                    ExecuteTest(ut);
                }
                catch (Exception ex)
                {
                    throw new AggregateException("Error in file: " + realFile, ex);
                }
            }
        }
    }
}
