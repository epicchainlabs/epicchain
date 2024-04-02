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
// UT_PoolItem.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Moq;
using Neo.Ledger;
using Neo.Network.P2P.Payloads;
using System;

namespace Neo.UnitTests.Ledger
{
    [TestClass]
    public class UT_PoolItem
    {
        private static readonly Random TestRandom = new Random(1337); // use fixed seed for guaranteed determinism

        [TestInitialize]
        public void TestSetup()
        {
            var timeValues = new[] {
                new DateTime(1968, 06, 01, 0, 0, 1, DateTimeKind.Utc),
            };

            var timeMock = new Mock<TimeProvider>();
            timeMock.SetupGet(tp => tp.UtcNow).Returns(() => timeValues[0])
                                              .Callback(() => timeValues[0] = timeValues[0].Add(TimeSpan.FromSeconds(1)));
            TimeProvider.Current = timeMock.Object;
        }

        [TestCleanup]
        public void TestCleanup()
        {
            // important to leave TimeProvider correct
            TimeProvider.ResetToDefault();
        }

        [TestMethod]
        public void PoolItem_CompareTo_Fee()
        {
            int size1 = 51;
            int netFeeSatoshi1 = 1;
            var tx1 = GenerateTx(netFeeSatoshi1, size1);
            int size2 = 51;
            int netFeeSatoshi2 = 2;
            var tx2 = GenerateTx(netFeeSatoshi2, size2);

            PoolItem pitem1 = new PoolItem(tx1);
            PoolItem pitem2 = new PoolItem(tx2);

            Console.WriteLine($"item1 time {pitem1.Timestamp} item2 time {pitem2.Timestamp}");
            // pitem1 < pitem2 (fee) => -1
            pitem1.CompareTo(pitem2).Should().Be(-1);
            // pitem2 > pitem1 (fee) => 1
            pitem2.CompareTo(pitem1).Should().Be(1);
        }

        [TestMethod]
        public void PoolItem_CompareTo_Hash()
        {
            int sizeFixed = 51;
            int netFeeSatoshiFixed = 1;

            var tx1 = GenerateTxWithFirstByteOfHashGreaterThanOrEqualTo(0x80, netFeeSatoshiFixed, sizeFixed);
            var tx2 = GenerateTxWithFirstByteOfHashLessThanOrEqualTo(0x79, netFeeSatoshiFixed, sizeFixed);

            tx1.Attributes = new TransactionAttribute[] { new HighPriorityAttribute() };

            PoolItem pitem1 = new PoolItem(tx1);
            PoolItem pitem2 = new PoolItem(tx2);

            // Different priority
            pitem2.CompareTo(pitem1).Should().Be(-1);

            // Bulk test
            for (int testRuns = 0; testRuns < 30; testRuns++)
            {
                tx1 = GenerateTxWithFirstByteOfHashGreaterThanOrEqualTo(0x80, netFeeSatoshiFixed, sizeFixed);
                tx2 = GenerateTxWithFirstByteOfHashLessThanOrEqualTo(0x79, netFeeSatoshiFixed, sizeFixed);

                pitem1 = new PoolItem(tx1);
                pitem2 = new PoolItem(tx2);

                pitem2.CompareTo((Transaction)null).Should().Be(1);

                // pitem2.tx.Hash < pitem1.tx.Hash => 1 descending order
                pitem2.CompareTo(pitem1).Should().Be(1);

                // pitem2.tx.Hash > pitem1.tx.Hash => -1 descending order
                pitem1.CompareTo(pitem2).Should().Be(-1);
            }
        }

        [TestMethod]
        public void PoolItem_CompareTo_Equals()
        {
            int sizeFixed = 500;
            int netFeeSatoshiFixed = 10;
            var tx = GenerateTx(netFeeSatoshiFixed, sizeFixed, new byte[] { 0x13, 0x37 });

            PoolItem pitem1 = new PoolItem(tx);
            PoolItem pitem2 = new PoolItem(tx);

            // pitem1 == pitem2 (fee) => 0
            pitem1.CompareTo(pitem2).Should().Be(0);
            pitem2.CompareTo(pitem1).Should().Be(0);
            pitem2.CompareTo((PoolItem)null).Should().Be(1);
        }

        public Transaction GenerateTxWithFirstByteOfHashGreaterThanOrEqualTo(byte firstHashByte, long networkFee, int size)
        {
            Transaction tx;
            do
            {
                tx = GenerateTx(networkFee, size);
            } while (tx.Hash < new UInt256(TestUtils.GetByteArray(32, firstHashByte)));

            return tx;
        }

        public Transaction GenerateTxWithFirstByteOfHashLessThanOrEqualTo(byte firstHashByte, long networkFee, int size)
        {
            Transaction tx;
            do
            {
                tx = GenerateTx(networkFee, size);
            } while (tx.Hash > new UInt256(TestUtils.GetByteArray(32, firstHashByte)));

            return tx;
        }

        // Generate Transaction with different sizes and prices
        public static Transaction GenerateTx(long networkFee, int size, byte[] overrideScriptBytes = null)
        {
            Transaction tx = new Transaction
            {
                Nonce = (uint)TestRandom.Next(),
                Script = overrideScriptBytes ?? new byte[0],
                NetworkFee = networkFee,
                Attributes = Array.Empty<TransactionAttribute>(),
                Signers = Array.Empty<Signer>(),
                Witnesses = new[]
                {
                    new Witness
                    {
                        InvocationScript = new byte[0],
                        VerificationScript = new byte[0]
                    }
                }
            };

            tx.Attributes.Length.Should().Be(0);
            tx.Signers.Length.Should().Be(0);

            int diff = size - tx.Size;
            if (diff < 0) throw new ArgumentException();
            if (diff > 0)
                tx.Witnesses[0].VerificationScript = new byte[diff];
            return tx;
        }
    }
}
