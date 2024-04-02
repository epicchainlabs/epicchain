// EpicChain Copyright Project (2021-2024)
// 
// Copyright (c) 2021-2024 EpicChain
// 
// EpicChain is an innovative blockchain network developed and maintained by xmoohad. This copyright project outlines the rights and responsibilities associated with the EpicChain software and its related components.
// 
// 1. Copyright Holder:
//    - xmoohad
// 
// 2. Project Name:
//    - EpicChain
// 
// 3. Project Description:
//    - EpicChain is a decentralized blockchain network that aims to revolutionize the way digital assets are managed, traded, and secured. With its innovative features and robust architecture, EpicChain provides a secure and efficient platform for various decentralized applications (dApps) and digital asset management.
// 
// 4. Copyright Period:
//    - The copyright for the EpicChain software and its related components is valid from 2021 to 2024.
// 
// 5. Copyright Statement:
//    - All rights reserved. No part of the EpicChain software or its related components may be reproduced, distributed, or transmitted in any form or by any means, without the prior written permission of the copyright holder, except in the case of brief quotations embodied in critical reviews and certain other noncommercial uses permitted by copyright law.
// 
// 6. License:
//    - The EpicChain software is licensed under the EpicChain Software License, a custom license that governs the use, distribution, and modification of the software. The EpicChain Software License is designed to promote the free and open development of the EpicChain network while protecting the interests of the copyright holder.
// 
// 7. Open Source:
//    - EpicChain is an open-source project, and its source code is available to the public under the terms of the EpicChain Software License. Developers are encouraged to contribute to the development of EpicChain and create innovative applications on top of the EpicChain network.
// 
// 8. Disclaimer:
//    - The EpicChain software and its related components are provided "as is," without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall the copyright holder or contributors be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the EpicChain software or its related components.
// 
// 9. Contact Information:
//    - For inquiries regarding the EpicChain copyright project, please contact xmoohad at [email address].
// 
// 10. Updates:
//     - This copyright project may be updated or modified from time to time to reflect changes in the EpicChain project or to address new legal or regulatory requirements. Users and developers are encouraged to check the latest version of the copyright project periodically.

//
// UT_InteropPrices.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.SmartContract;
using Neo.UnitTests.Extensions;
using Neo.VM;

namespace Neo.UnitTests.SmartContract
{
    [TestClass]
    public class UT_InteropPrices
    {
        [TestMethod]
        public void ApplicationEngineFixedPrices()
        {
            // System.Runtime.CheckWitness: f827ec8c (price is 200)
            byte[] SyscallSystemRuntimeCheckWitnessHash = new byte[] { 0x68, 0xf8, 0x27, 0xec, 0x8c };
            using (ApplicationEngine ae = ApplicationEngine.Create(TriggerType.Application, null, null, gas: 0))
            {
                ae.LoadScript(SyscallSystemRuntimeCheckWitnessHash);
                ApplicationEngine.System_Runtime_CheckWitness.FixedPrice.Should().Be(0_00001024L);
            }

            // System.Storage.GetContext: 9bf667ce (price is 1)
            byte[] SyscallSystemStorageGetContextHash = new byte[] { 0x68, 0x9b, 0xf6, 0x67, 0xce };
            using (ApplicationEngine ae = ApplicationEngine.Create(TriggerType.Application, null, null, gas: 0))
            {
                ae.LoadScript(SyscallSystemStorageGetContextHash);
                ApplicationEngine.System_Storage_GetContext.FixedPrice.Should().Be(0_00000016L);
            }

            // System.Storage.Get: 925de831 (price is 100)
            byte[] SyscallSystemStorageGetHash = new byte[] { 0x68, 0x92, 0x5d, 0xe8, 0x31 };
            using (ApplicationEngine ae = ApplicationEngine.Create(TriggerType.Application, null, null, gas: 0))
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

            var snapshot = TestBlockchain.GetTestSnapshot();
            snapshot.Add(skey, sItem);
            snapshot.AddContract(script.ToScriptHash(), contractState);

            using ApplicationEngine ae = ApplicationEngine.Create(TriggerType.Application, null, snapshot);
            Debugger debugger = new(ae);
            ae.LoadScript(script);
            debugger.StepInto();
            debugger.StepInto();
            debugger.StepInto();
            var setupPrice = ae.GasConsumed;
            debugger.Execute();
            (ae.GasConsumed - setupPrice).Should().Be(ae.StoragePrice * value.Length + (1 << 15) * 30);
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

            var snapshot = TestBlockchain.GetTestSnapshot();
            snapshot.Add(skey, sItem);
            snapshot.AddContract(script.ToScriptHash(), contractState);

            using ApplicationEngine applicationEngine = ApplicationEngine.Create(TriggerType.Application, null, snapshot);
            Debugger debugger = new(applicationEngine);
            applicationEngine.LoadScript(script);
            debugger.StepInto();
            debugger.StepInto();
            debugger.StepInto();
            var setupPrice = applicationEngine.GasConsumed;
            debugger.Execute();
            (applicationEngine.GasConsumed - setupPrice).Should().Be(1 * applicationEngine.StoragePrice + (1 << 15) * 30);
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

            var snapshot = TestBlockchain.GetTestSnapshot();
            snapshot.Add(skey, sItem);
            snapshot.AddContract(script.ToScriptHash(), contractState);

            using ApplicationEngine ae = ApplicationEngine.Create(TriggerType.Application, null, snapshot);
            Debugger debugger = new(ae);
            ae.LoadScript(script);
            debugger.StepInto();
            debugger.StepInto();
            debugger.StepInto();
            var setupPrice = ae.GasConsumed;
            debugger.StepInto();
            debugger.StepInto();
            (ae.GasConsumed - setupPrice).Should().Be((1 + (oldValue.Length / 4) + value.Length - oldValue.Length) * ae.StoragePrice + (1 << 15) * 30);
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

            var snapshot = TestBlockchain.GetTestSnapshot();
            snapshot.Add(skey, sItem);
            snapshot.AddContract(script.ToScriptHash(), contractState);

            using ApplicationEngine ae = ApplicationEngine.Create(TriggerType.Application, null, snapshot);
            Debugger debugger = new(ae);
            ae.LoadScript(script);
            debugger.StepInto(); //push value
            debugger.StepInto(); //push key
            debugger.StepInto(); //syscall Storage.GetContext
            debugger.StepInto(); //syscall Storage.Put
            debugger.StepInto(); //push value
            debugger.StepInto(); //push key
            debugger.StepInto(); //syscall Storage.GetContext
            var setupPrice = ae.GasConsumed;
            debugger.StepInto(); //syscall Storage.Put
            (ae.GasConsumed - setupPrice).Should().Be((sItem.Value.Length / 4 + 1) * ae.StoragePrice + (1 << 15) * 30); // = PUT basic fee
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
