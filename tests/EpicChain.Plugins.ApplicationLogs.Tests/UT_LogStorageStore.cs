// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_LogStorageStore.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Microsoft.AspNetCore.Authorization;
using EpicChain.Persistence;
using EpicChain.Plugins.ApplicationLogs.Store;
using EpicChain.Plugins.ApplicationLogs.Store.States;
using EpicChain.Plugins.ApplicationsLogs.Tests.Setup;
using EpicChain.SmartContract;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Xunit;

namespace EpicChain.Plugins.ApplicationsLogs.Tests
{
    public class UT_LogStorageStore
    {
        [Theory]
        [InlineData(TriggerType.OnPersist, "0x0000000000000000000000000000000000000000000000000000000000000001")]
        [InlineData(TriggerType.Application, "0x0000000000000000000000000000000000000000000000000000000000000002")]
        [InlineData(TriggerType.PostPersist, "0x0000000000000000000000000000000000000000000000000000000000000003")]
        public void Test_Put_Get_BlockState_Storage(TriggerType expectedAppTrigger, string expectedBlockHashString)
        {
            var expectedGuid = Guid.NewGuid();
            var expectedHash = UInt256.Parse(expectedBlockHashString);

            using (var snapshot = TestStorage.Store.GetSnapshot())
            {
                using (var lss = new LogStorageStore(snapshot))
                {
                    // Put Block States in Storage for each Trigger
                    lss.PutBlockState(expectedHash, expectedAppTrigger, BlockLogState.Create([expectedGuid]));
                    // Commit Data to "Store" Storage for Lookup
                    snapshot.Commit();
                }
            }

            // The Current way that ISnapshot Works we need to Create New Instance of LogStorageStore
            using (var lss = new LogStorageStore(TestStorage.Store.GetSnapshot()))
            {
                // Get OnPersist Block State from Storage
                var actualFound = lss.TryGetBlockState(expectedHash, expectedAppTrigger, out var actualState);

                Assert.True(actualFound);
                Assert.NotNull(actualState);
                Assert.NotNull(actualState.NotifyLogIds);
                Assert.Single(actualState.NotifyLogIds);
                Assert.Equal(expectedGuid, actualState.NotifyLogIds[0]);
            }
        }

        [Theory]
        [InlineData("00000000-0000-0000-0000-000000000000", "0x0000000000000000000000000000000000000000000000000000000000000000")]
        [InlineData("00000000-0000-0000-0000-000000000001", "0x0000000000000000000000000000000000000000000000000000000000000001")]
        public void Test_Put_Get_TransactionEngineState_Storage(string expectedLogId, string expectedHashString)
        {
            var expectedGuid = Guid.Parse(expectedLogId);
            var expectedTxHash = UInt256.Parse(expectedHashString);

            using (var snapshot = TestStorage.Store.GetSnapshot())
            {
                using (var lss = new LogStorageStore(snapshot))
                {
                    // Put Block States in Storage for each Trigger
                    lss.PutTransactionEngineState(expectedTxHash, TransactionEngineLogState.Create([expectedGuid]));
                    // Commit Data to "Store" Storage for Lookup
                    snapshot.Commit();
                }
            }

            using (var lss = new LogStorageStore(TestStorage.Store.GetSnapshot()))
            {
                // Get OnPersist Block State from Storage
                var actualFound = lss.TryGetTransactionEngineState(expectedTxHash, out var actualState);

                Assert.True(actualFound);
                Assert.NotNull(actualState);
                Assert.NotNull(actualState.LogIds);
                Assert.Single(actualState.LogIds);
                Assert.Equal(expectedGuid, actualState.LogIds[0]);
            }
        }

        [Theory]
        [InlineData("0x0000000000000000000000000000000000000000", "Hello World")]
        [InlineData("0x0000000000000000000000000000000000000001", "Hello Again")]
        public void Test_Put_Get_EngineState_Storage(string expectedScriptHashString, string expectedMessage)
        {
            var expectedScriptHash = UInt160.Parse(expectedScriptHashString);
            var expectedGuid = Guid.Empty;

            using (var snapshot = TestStorage.Store.GetSnapshot())
            {
                using (var lss = new LogStorageStore(snapshot))
                {
                    expectedGuid = lss.PutEngineState(EngineLogState.Create(expectedScriptHash, expectedMessage));
                    snapshot.Commit();
                }
            }

            using (var lss = new LogStorageStore(TestStorage.Store.GetSnapshot()))
            {
                var actualFound = lss.TryGetEngineState(expectedGuid, out var actualState);

                Assert.True(actualFound);
                Assert.NotNull(actualState);
                Assert.Equal(expectedScriptHash, actualState.ScriptHash);
                Assert.Equal(expectedMessage, actualState.Message);
            }
        }

        [Theory]
        [InlineData("0x0000000000000000000000000000000000000000", "SayHello", "00000000-0000-0000-0000-000000000000")]
        [InlineData("0x0000000000000000000000000000000000000001", "SayGoodBye", "00000000-0000-0000-0000-000000000001")]
        public void Test_Put_Get_NotifyState_Storage(string expectedScriptHashString, string expectedEventName, string expectedItemGuidString)
        {
            var expectedScriptHash = UInt160.Parse(expectedScriptHashString);
            var expectedNotifyEventArgs = new NotifyEventArgs(null, expectedScriptHash, expectedEventName, []);
            var expectedItemGuid = Guid.Parse(expectedItemGuidString);
            var expectedGuid = Guid.Empty;

            using (var snapshot = TestStorage.Store.GetSnapshot())
            {
                using (var lss = new LogStorageStore(snapshot))
                {
                    expectedGuid = lss.PutNotifyState(NotifyLogState.Create(expectedNotifyEventArgs, [expectedItemGuid]));
                    snapshot.Commit();
                }
            }

            using (var lss = new LogStorageStore(TestStorage.Store.GetSnapshot()))
            {
                var actualFound = lss.TryGetNotifyState(expectedGuid, out var actualState);

                Assert.True(actualFound);
                Assert.NotNull(actualState);
                Assert.Equal(expectedScriptHash, actualState.ScriptHash);
                Assert.Equal(expectedEventName, actualState.EventName);
                Assert.NotNull(actualState.StackItemIds);
                Assert.Single(actualState.StackItemIds);
                Assert.Equal(expectedItemGuid, actualState.StackItemIds[0]);
            }
        }
    }
}
