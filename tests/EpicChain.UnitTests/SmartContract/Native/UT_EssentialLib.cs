// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_EssentialLib.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Extensions;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using EpicChain.VM.Types;
using System.Collections.Generic;
using System.Numerics;

namespace EpicChain.UnitTests.SmartContract.Native
{
    [TestClass]
    public class UT_EssentialLib
    {
        [TestMethod]
        public void TestBinary()
        {
            var data = System.Array.Empty<byte>();

            CollectionAssert.AreEqual(data, EssentialLib.Base64Decode(EssentialLib.Base64Encode(data)));
            CollectionAssert.AreEqual(data, EssentialLib.Base58Decode(EssentialLib.Base58Encode(data)));

            data = new byte[] { 1, 2, 3 };

            CollectionAssert.AreEqual(data, EssentialLib.Base64Decode(EssentialLib.Base64Encode(data)));
            CollectionAssert.AreEqual(data, EssentialLib.Base58Decode(EssentialLib.Base58Encode(data)));
            Assert.AreEqual("AQIDBA==", EssentialLib.Base64Encode(new byte[] { 1, 2, 3, 4 }));
            Assert.AreEqual("2VfUX", EssentialLib.Base58Encode(new byte[] { 1, 2, 3, 4 }));
        }

        [TestMethod]
        public void TestItoaAtoi()
        {
            Assert.AreEqual("1", EssentialLib.Itoa(BigInteger.One, 10));
            Assert.AreEqual("1", EssentialLib.Itoa(BigInteger.One, 16));
            Assert.AreEqual("-1", EssentialLib.Itoa(BigInteger.MinusOne, 10));
            Assert.AreEqual("f", EssentialLib.Itoa(BigInteger.MinusOne, 16));
            Assert.AreEqual("3b9aca00", EssentialLib.Itoa(1_000_000_000, 16));
            Assert.AreEqual(-1, EssentialLib.Atoi("-1", 10));
            Assert.AreEqual(1, EssentialLib.Atoi("+1", 10));
            Assert.AreEqual(-1, EssentialLib.Atoi("ff", 16));
            Assert.AreEqual(-1, EssentialLib.Atoi("FF", 16));
            Assert.ThrowsException<System.FormatException>(() => EssentialLib.Atoi("a", 10));
            Assert.ThrowsException<System.FormatException>(() => EssentialLib.Atoi("g", 16));
            Assert.ThrowsException<System.ArgumentOutOfRangeException>(() => EssentialLib.Atoi("a", 11));

            EssentialLib.Atoi(EssentialLib.Itoa(BigInteger.One, 10)).Should().Be(BigInteger.One);
            EssentialLib.Atoi(EssentialLib.Itoa(BigInteger.MinusOne, 10)).Should().Be(BigInteger.MinusOne);
        }

        [TestMethod]
        public void MemoryCompare()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();

            using (var script = new ScriptBuilder())
            {
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "memoryCompare", "abc", "c");
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "memoryCompare", "abc", "d");
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "memoryCompare", "abc", "abc");
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "memoryCompare", "abc", "abcd");

                using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
                engine.LoadScript(script.ToArray());

                Assert.AreEqual(engine.Execute(), VMState.HALT);
                Assert.AreEqual(4, engine.ResultStack.Count);

                Assert.AreEqual(-1, engine.ResultStack.Pop<Integer>().GetInteger());
                Assert.AreEqual(0, engine.ResultStack.Pop<Integer>().GetInteger());
                Assert.AreEqual(-1, engine.ResultStack.Pop<Integer>().GetInteger());
                Assert.AreEqual(-1, engine.ResultStack.Pop<Integer>().GetInteger());
            }
        }

        [TestMethod]
        public void CheckDecodeEncode()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();

            using (ScriptBuilder script = new())
            {
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "base58CheckEncode", new byte[] { 1, 2, 3 });

                using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
                engine.LoadScript(script.ToArray());

                Assert.AreEqual(engine.Execute(), VMState.HALT);
                Assert.AreEqual(1, engine.ResultStack.Count);

                Assert.AreEqual("3DUz7ncyT", engine.ResultStack.Pop<ByteString>().GetString());
            }

            using (ScriptBuilder script = new())
            {
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "base58CheckDecode", "3DUz7ncyT");

                using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
                engine.LoadScript(script.ToArray());

                Assert.AreEqual(engine.Execute(), VMState.HALT);
                Assert.AreEqual(1, engine.ResultStack.Count);

                CollectionAssert.AreEqual(new byte[] { 1, 2, 3 }, engine.ResultStack.Pop<ByteString>().GetSpan().ToArray());
            }

            // Error

            using (ScriptBuilder script = new())
            {
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "base58CheckDecode", "AA");

                using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
                engine.LoadScript(script.ToArray());

                Assert.AreEqual(engine.Execute(), VMState.FAULT);
            }

            using (ScriptBuilder script = new())
            {
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "base58CheckDecode", null);

                using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
                engine.LoadScript(script.ToArray());

                Assert.AreEqual(engine.Execute(), VMState.FAULT);
            }
        }

        [TestMethod]
        public void MemorySearch()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();

            using (var script = new ScriptBuilder())
            {
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "memorySearch", "abc", "c", 0);
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "memorySearch", "abc", "c", 1);
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "memorySearch", "abc", "c", 2);
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "memorySearch", "abc", "c", 3);
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "memorySearch", "abc", "d", 0);

                using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
                engine.LoadScript(script.ToArray());

                Assert.AreEqual(engine.Execute(), VMState.HALT);
                Assert.AreEqual(5, engine.ResultStack.Count);

                Assert.AreEqual(-1, engine.ResultStack.Pop<Integer>().GetInteger());
                Assert.AreEqual(-1, engine.ResultStack.Pop<Integer>().GetInteger());
                Assert.AreEqual(2, engine.ResultStack.Pop<Integer>().GetInteger());
                Assert.AreEqual(2, engine.ResultStack.Pop<Integer>().GetInteger());
            }

            using (var script = new ScriptBuilder())
            {
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "memorySearch", "abc", "c", 0, false);
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "memorySearch", "abc", "c", 1, false);
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "memorySearch", "abc", "c", 2, false);
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "memorySearch", "abc", "c", 3, false);
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "memorySearch", "abc", "d", 0, false);

                using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
                engine.LoadScript(script.ToArray());

                Assert.AreEqual(engine.Execute(), VMState.HALT);
                Assert.AreEqual(5, engine.ResultStack.Count);

                Assert.AreEqual(-1, engine.ResultStack.Pop<Integer>().GetInteger());
                Assert.AreEqual(-1, engine.ResultStack.Pop<Integer>().GetInteger());
                Assert.AreEqual(2, engine.ResultStack.Pop<Integer>().GetInteger());
                Assert.AreEqual(2, engine.ResultStack.Pop<Integer>().GetInteger());
            }

            using (var script = new ScriptBuilder())
            {
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "memorySearch", "abc", "c", 0, true);
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "memorySearch", "abc", "c", 1, true);
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "memorySearch", "abc", "c", 2, true);
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "memorySearch", "abc", "c", 3, true);
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "memorySearch", "abc", "d", 0, true);

                using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
                engine.LoadScript(script.ToArray());

                Assert.AreEqual(engine.Execute(), VMState.HALT);
                Assert.AreEqual(5, engine.ResultStack.Count);

                Assert.AreEqual(-1, engine.ResultStack.Pop<Integer>().GetInteger());
                Assert.AreEqual(2, engine.ResultStack.Pop<Integer>().GetInteger());
                Assert.AreEqual(-1, engine.ResultStack.Pop<Integer>().GetInteger());
                Assert.AreEqual(-1, engine.ResultStack.Pop<Integer>().GetInteger());
            }
        }

        [TestMethod]
        public void StringSplit()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();

            using var script = new ScriptBuilder();
            script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "stringSplit", "a,b", ",");

            using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
            engine.LoadScript(script.ToArray());

            Assert.AreEqual(engine.Execute(), VMState.HALT);
            Assert.AreEqual(1, engine.ResultStack.Count);

            var arr = engine.ResultStack.Pop<Array>();
            Assert.AreEqual(2, arr.Count);
            Assert.AreEqual("a", arr[0].GetString());
            Assert.AreEqual("b", arr[1].GetString());
        }

        [TestMethod]
        public void StringElementLength()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();

            using var script = new ScriptBuilder();
            script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "strLen", "🦆");
            script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "strLen", "ã");
            script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "strLen", "a");

            using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
            engine.LoadScript(script.ToArray());

            Assert.AreEqual(engine.Execute(), VMState.HALT);
            Assert.AreEqual(3, engine.ResultStack.Count);
            Assert.AreEqual(1, engine.ResultStack.Pop().GetInteger());
            Assert.AreEqual(1, engine.ResultStack.Pop().GetInteger());
            Assert.AreEqual(1, engine.ResultStack.Pop().GetInteger());
        }

        [TestMethod]
        public void TestInvalidUtf8Sequence()
        {
            // Simulating invalid UTF-8 byte (0xff) decoded as a UTF-16 char
            const char badChar = (char)0xff;
            var badStr = badChar.ToString();
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();

            using var script = new ScriptBuilder();
            script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "strLen", badStr);
            script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "strLen", badStr + "ab");

            using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
            engine.LoadScript(script.ToArray());

            Assert.AreEqual(engine.Execute(), VMState.HALT);
            Assert.AreEqual(2, engine.ResultStack.Count);
            Assert.AreEqual(3, engine.ResultStack.Pop().GetInteger());
            Assert.AreEqual(1, engine.ResultStack.Pop().GetInteger());
        }

        [TestMethod]
        public void Json_Deserialize()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();

            // Good

            using (var script = new ScriptBuilder())
            {
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "jsonDeserialize", "123");
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "jsonDeserialize", "null");

                using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
                engine.LoadScript(script.ToArray());

                Assert.AreEqual(engine.Execute(), VMState.HALT);
                Assert.AreEqual(2, engine.ResultStack.Count);

                engine.ResultStack.Pop<Null>();
                Assert.IsTrue(engine.ResultStack.Pop().GetInteger() == 123);
            }

            // Error 1 - Wrong Json

            using (ScriptBuilder script = new())
            {
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "jsonDeserialize", "***");

                using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
                engine.LoadScript(script.ToArray());

                Assert.AreEqual(engine.Execute(), VMState.FAULT);
                Assert.AreEqual(0, engine.ResultStack.Count);
            }

            // Error 2 - No decimals

            using (var script = new ScriptBuilder())
            {
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "jsonDeserialize", "123.45");

                using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
                engine.LoadScript(script.ToArray());

                Assert.AreEqual(engine.Execute(), VMState.FAULT);
                Assert.AreEqual(0, engine.ResultStack.Count);
            }
        }

        [TestMethod]
        public void Json_Serialize()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();

            // Good

            using (var script = new ScriptBuilder())
            {
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "jsonSerialize", 5);
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "jsonSerialize", true);
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "jsonSerialize", "test");
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "jsonSerialize", new object[] { null });
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "jsonSerialize", new ContractParameter(ContractParameterType.Map)
                {
                    Value = new List<KeyValuePair<ContractParameter, ContractParameter>>() {
                        { new KeyValuePair<ContractParameter, ContractParameter>(
                            new ContractParameter(ContractParameterType.String){ Value="key" },
                            new ContractParameter(ContractParameterType.String){ Value= "value" })
                        }
                    }
                });

                using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
                engine.LoadScript(script.ToArray());

                Assert.AreEqual(engine.Execute(), VMState.HALT);
                Assert.AreEqual(5, engine.ResultStack.Count);

                Assert.IsTrue(engine.ResultStack.Pop<ByteString>().GetString() == "{\"key\":\"value\"}");
                Assert.IsTrue(engine.ResultStack.Pop<ByteString>().GetString() == "null");
                Assert.IsTrue(engine.ResultStack.Pop<ByteString>().GetString() == "\"test\"");
                Assert.IsTrue(engine.ResultStack.Pop<ByteString>().GetString() == "true");
                Assert.IsTrue(engine.ResultStack.Pop<ByteString>().GetString() == "5");
            }

            // Error

            using (var script = new ScriptBuilder())
            {
                script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "jsonSerialize");

                using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
                engine.LoadScript(script.ToArray());

                Assert.AreEqual(engine.Execute(), VMState.FAULT);
                Assert.AreEqual(0, engine.ResultStack.Count);
            }
        }

        [TestMethod]
        public void TestRuntime_Serialize()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();

            // Good

            using ScriptBuilder script = new();
            script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "serialize", 100);
            script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "serialize", "test");

            using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
            engine.LoadScript(script.ToArray());

            Assert.AreEqual(engine.Execute(), VMState.HALT);
            Assert.AreEqual(2, engine.ResultStack.Count);

            Assert.AreEqual(engine.ResultStack.Pop<ByteString>().GetSpan().ToHexString(), "280474657374");
            Assert.AreEqual(engine.ResultStack.Pop<ByteString>().GetSpan().ToHexString(), "210164");
        }

        [TestMethod]
        public void TestRuntime_Deserialize()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();

            // Good

            using ScriptBuilder script = new();
            script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "deserialize", "280474657374".HexToBytes());
            script.EmitDynamicCall(NativeContract.EssentialLib.Hash, "deserialize", "210164".HexToBytes());

            using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings);
            engine.LoadScript(script.ToArray());

            Assert.AreEqual(engine.Execute(), VMState.HALT);
            Assert.AreEqual(2, engine.ResultStack.Count);

            Assert.AreEqual(engine.ResultStack.Pop<Integer>().GetInteger(), 100);
            Assert.AreEqual(engine.ResultStack.Pop<ByteString>().GetString(), "test");
        }
    }
}
