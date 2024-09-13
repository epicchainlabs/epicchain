// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_InteropPrices.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.SmartContract;
using EpicChain.UnitTests.Extensions;
using EpicChain.VM;

namespace EpicChain.UnitTests.SmartContract
{
    [TestClass]
    public class UT_InteropPrices
    {
        [TestMethod]
        public void ApplicationEngineFixedPrices()
        {
            // System.Runtime.CheckWitness: f827ec8c (price is 200)
            byte[] SyscallSystemRuntimeCheckWitnessHash = new byte[] { 0x68, 0xf8, 0x27, 0xec, 0x8c };
            using (ApplicationEngine ae = ApplicationEngine.Create(TriggerType.Application, null, null, epicpulse: 0))
            {
                ae.LoadScript(SyscallSystemRuntimeCheckWitnessHash);
                ApplicationEngine.System_Runtime_CheckWitness.FixedPrice.Should().Be(0_00001024L);
            }

            // System.Storage.GetContext: 9bf667ce (price is 1)
            byte[] SyscallSystemStorageGetContextHash = new byte[] { 0x68, 0x9b, 0xf6, 0x67, 0xce };
            using (ApplicationEngine ae = ApplicationEngine.Create(TriggerType.Application, null, null, epicpulse: 0))
            {
                ae.LoadScript(SyscallSystemStorageGetContextHash);
                ApplicationEngine.System_Storage_GetContext.FixedPrice.Should().Be(0_00000016L);
            }

            // System.Storage.Get: 925de831 (price is 100)
            byte[] SyscallSystemStorageGetHash = new byte[] { 0x68, 0x92, 0x5d, 0xe8, 0x31 };
            using (ApplicationEngine ae = ApplicationEngine.Create(TriggerType.Application, null, null, epicpulse: 0))
            {
                ae.LoadScript(SyscallSystemStorageGetHash);
                ApplicationEngine.System_Storage_Get.FixedPrice.Should().Be(32768L);
            }
        }

        /// <summary>
        /// Put without previous content (should charge per byte used)
        /// </summary>
        [TestMethod]
        public void ApplicationEngineRegularPut()
        {
            var key = new byte[] { (byte)OpCode.PUSH1 };
            var value = new byte[] { (byte)OpCode.PUSH1 };

            byte[] script = CreatePutScript(key, value);

            ContractState contractState = TestUtils.GetContract(script);

            StorageKey skey = TestUtils.GetStorageKey(contractState.Id, key);
            StorageItem sItem = TestUtils.GetStorageItem(System.Array.Empty<byte>());

            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            snapshotCache.Add(skey, sItem);
            snapshotCache.AddContract(script.ToScriptHash(), contractState);

            using ApplicationEngine ae = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache);
            Debugger debugger = new(ae);
            ae.LoadScript(script);
            debugger.StepInto();
            debugger.StepInto();
            debugger.StepInto();
            var setupPrice = ae.FeeConsumed;
            debugger.Execute();
            (ae.FeeConsumed - setupPrice).Should().Be(ae.StoragePrice * value.Length + (1 << 15) * 30);
        }

        /// <summary>
        /// Reuses the same amount of storage. Should cost 0.
        /// </summary>
        [TestMethod]
        public void ApplicationEngineReusedStorage_FullReuse()
        {
            var key = new byte[] { (byte)OpCode.PUSH1 };
            var value = new byte[] { (byte)OpCode.PUSH1 };

            byte[] script = CreatePutScript(key, value);

            ContractState contractState = TestUtils.GetContract(script);

            StorageKey skey = TestUtils.GetStorageKey(contractState.Id, key);
            StorageItem sItem = TestUtils.GetStorageItem(value);

            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            snapshotCache.Add(skey, sItem);
            snapshotCache.AddContract(script.ToScriptHash(), contractState);

            using ApplicationEngine applicationEngine = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache);
            Debugger debugger = new(applicationEngine);
            applicationEngine.LoadScript(script);
            debugger.StepInto();
            debugger.StepInto();
            debugger.StepInto();
            var setupPrice = applicationEngine.FeeConsumed;
            debugger.Execute();
            (applicationEngine.FeeConsumed - setupPrice).Should().Be(1 * applicationEngine.StoragePrice + (1 << 15) * 30);
        }

        /// <summary>
        /// Reuses one byte and allocates a new one
        /// It should only pay for the second byte.
        /// </summary>
        [TestMethod]
        public void ApplicationEngineReusedStorage_PartialReuse()
        {
            var key = new byte[] { (byte)OpCode.PUSH1 };
            var oldValue = new byte[] { (byte)OpCode.PUSH1 };
            var value = new byte[] { (byte)OpCode.PUSH1, (byte)OpCode.PUSH1 };

            byte[] script = CreatePutScript(key, value);

            ContractState contractState = TestUtils.GetContract(script);

            StorageKey skey = TestUtils.GetStorageKey(contractState.Id, key);
            StorageItem sItem = TestUtils.GetStorageItem(oldValue);

            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            snapshotCache.Add(skey, sItem);
            snapshotCache.AddContract(script.ToScriptHash(), contractState);

            using ApplicationEngine ae = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache);
            Debugger debugger = new(ae);
            ae.LoadScript(script);
            debugger.StepInto();
            debugger.StepInto();
            debugger.StepInto();
            var setupPrice = ae.FeeConsumed;
            debugger.StepInto();
            debugger.StepInto();
            (ae.FeeConsumed - setupPrice).Should().Be((1 + (oldValue.Length / 4) + value.Length - oldValue.Length) * ae.StoragePrice + (1 << 15) * 30);
        }

        /// <summary>
        /// Use put for the same key twice.
        /// Pays for 1 extra byte for the first Put and 1 byte for the second basic fee (as value2.length == value1.length).
        /// </summary>
        [TestMethod]
        public void ApplicationEngineReusedStorage_PartialReuseTwice()
        {
            var key = new byte[] { (byte)OpCode.PUSH1 };
            var oldValue = new byte[] { (byte)OpCode.PUSH1 };
            var value = new byte[] { (byte)OpCode.PUSH1, (byte)OpCode.PUSH1 };

            byte[] script = CreateMultiplePutScript(key, value);

            ContractState contractState = TestUtils.GetContract(script);

            StorageKey skey = TestUtils.GetStorageKey(contractState.Id, key);
            StorageItem sItem = TestUtils.GetStorageItem(oldValue);

            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            snapshotCache.Add(skey, sItem);
            snapshotCache.AddContract(script.ToScriptHash(), contractState);

            using ApplicationEngine ae = ApplicationEngine.Create(TriggerType.Application, null, snapshotCache);
            Debugger debugger = new(ae);
            ae.LoadScript(script);
            debugger.StepInto(); //push value
            debugger.StepInto(); //push key
            debugger.StepInto(); //syscall Storage.GetContext
            debugger.StepInto(); //syscall Storage.Put
            debugger.StepInto(); //push value
            debugger.StepInto(); //push key
            debugger.StepInto(); //syscall Storage.GetContext
            var setupPrice = ae.FeeConsumed;
            debugger.StepInto(); //syscall Storage.Put
            (ae.FeeConsumed - setupPrice).Should().Be((sItem.Value.Length / 4 + 1) * ae.StoragePrice + (1 << 15) * 30); // = PUT basic fee
        }

        private static byte[] CreateMultiplePutScript(byte[] key, byte[] value, int times = 2)
        {
            var scriptBuilder = new ScriptBuilder();

            for (int i = 0; i < times; i++)
            {
                scriptBuilder.EmitPush(value);
                scriptBuilder.EmitPush(key);
                scriptBuilder.EmitSysCall(ApplicationEngine.System_Storage_GetContext);
                scriptBuilder.EmitSysCall(ApplicationEngine.System_Storage_Put);
            }

            return scriptBuilder.ToArray();
        }

        private static byte[] CreatePutScript(byte[] key, byte[] value)
        {
            var scriptBuilder = new ScriptBuilder();
            scriptBuilder.EmitPush(value);
            scriptBuilder.EmitPush(key);
            scriptBuilder.EmitSysCall(ApplicationEngine.System_Storage_GetContext);
            scriptBuilder.EmitSysCall(ApplicationEngine.System_Storage_Put);
            return scriptBuilder.ToArray();
        }
    }
}
