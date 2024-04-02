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
// UT_Conflicts.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.IO;
using Neo.Network.P2P.Payloads;
using Neo.SmartContract;
using Neo.SmartContract.Native;
using Neo.VM;
using System;

namespace Neo.UnitTests.Network.P2P.Payloads
{
    [TestClass]
    public class UT_Conflicts
    {
        private const byte Prefix_Transaction = 11;
        private static readonly UInt256 _u = new UInt256(new byte[32] {
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01
            });

        [TestMethod]
        public void Size_Get()
        {
            var test = new Conflicts() { Hash = _u };
            test.Size.Should().Be(1 + 32);
        }

        [TestMethod]
        public void ToJson()
        {
            var test = new Conflicts() { Hash = _u };
            var json = test.ToJson().ToString();
            Assert.AreEqual(@"{""type"":""Conflicts"",""hash"":""0x0101010101010101010101010101010101010101010101010101010101010101""}", json);
        }

        [TestMethod]
        public void DeserializeAndSerialize()
        {
            var test = new Conflicts() { Hash = _u };

            var clone = test.ToArray().AsSerializable<Conflicts>();
            Assert.AreEqual(clone.Type, test.Type);

            // As transactionAttribute
            byte[] buffer = test.ToArray();
            var reader = new MemoryReader(buffer);
            clone = TransactionAttribute.DeserializeFrom(ref reader) as Conflicts;
            Assert.AreEqual(clone.Type, test.Type);

            // Wrong type
            buffer[0] = 0xff;
            Assert.ThrowsException<FormatException>(() =>
            {
                var reader = new MemoryReader(buffer);
                TransactionAttribute.DeserializeFrom(ref reader);
            });
        }

        [TestMethod]
        public void Verify()
        {
            var test = new Conflicts() { Hash = _u };
            var snapshot = TestBlockchain.GetTestSnapshot();
            var key = Ledger.UT_MemoryPool.CreateStorageKey(NativeContract.Ledger.Id, Prefix_Transaction, _u.ToArray());

            // Conflicting transaction is in the Conflicts attribute of some other on-chain transaction.
            var conflict = new TransactionState();
            snapshot.Add(key, new StorageItem(conflict));
            Assert.IsTrue(test.Verify(snapshot, new Transaction()));

            // Conflicting transaction is on-chain.
            snapshot.Delete(key);
            conflict = new TransactionState
            {
                BlockIndex = 123,
                Transaction = new Transaction(),
                State = VMState.NONE
            };
            snapshot.Add(key, new StorageItem(conflict));
            Assert.IsFalse(test.Verify(snapshot, new Transaction()));

            // There's no conflicting transaction at all.
            snapshot.Delete(key);
            Assert.IsTrue(test.Verify(snapshot, new Transaction()));
        }
    }
}
