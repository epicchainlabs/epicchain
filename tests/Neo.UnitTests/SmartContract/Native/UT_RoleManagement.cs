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
// UT_RoleManagement.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.Cryptography.ECC;
using Neo.IO;
using Neo.Network.P2P.Payloads;
using Neo.Persistence;
using Neo.SmartContract;
using Neo.SmartContract.Native;
using Neo.UnitTests.Extensions;
using Neo.Wallets;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;

namespace Neo.UnitTests.SmartContract.Native
{
    [TestClass]
    public class UT_RoleManagement
    {
        private DataCache _snapshot;

        [TestInitialize]
        public void TestSetup()
        {
            _snapshot = TestBlockchain.GetTestSnapshot();
        }

        [TestCleanup]
        public void Clean()
        {
            TestBlockchain.ResetStore();
        }

        [TestMethod]
        public void TestSetAndGet()
        {
            byte[] privateKey1 = new byte[32];
            var rng1 = System.Security.Cryptography.RandomNumberGenerator.Create();
            rng1.GetBytes(privateKey1);
            KeyPair key1 = new KeyPair(privateKey1);
            byte[] privateKey2 = new byte[32];
            var rng2 = System.Security.Cryptography.RandomNumberGenerator.Create();
            rng2.GetBytes(privateKey2);
            KeyPair key2 = new KeyPair(privateKey2);
            ECPoint[] publicKeys = new ECPoint[2];
            publicKeys[0] = key1.PublicKey;
            publicKeys[1] = key2.PublicKey;
            publicKeys = publicKeys.OrderBy(p => p).ToArray();

            List<Role> roles = new List<Role>() { Role.StateValidator, Role.Oracle, Role.NeoFSAlphabetNode, Role.P2PNotary };
            foreach (var role in roles)
            {
                var snapshot1 = _snapshot.CreateSnapshot();
                UInt160 committeeMultiSigAddr = NativeContract.NEO.GetCommitteeAddress(snapshot1);
                List<NotifyEventArgs> notifications = new List<NotifyEventArgs>();
                EventHandler<NotifyEventArgs> ev = (o, e) => notifications.Add(e);
                ApplicationEngine.Notify += ev;
                var ret = NativeContract.RoleManagement.Call(
                    snapshot1,
                    new Nep17NativeContractExtensions.ManualWitness(committeeMultiSigAddr),
                    new Block { Header = new Header() },
                    "designateAsRole",
                    new ContractParameter(ContractParameterType.Integer) { Value = new BigInteger((int)role) },
                    new ContractParameter(ContractParameterType.Array) { Value = publicKeys.Select(p => new ContractParameter(ContractParameterType.ByteArray) { Value = p.ToArray() }).ToList() }
                );
                snapshot1.Commit();
                ApplicationEngine.Notify -= ev;
                notifications.Count.Should().Be(1);
                notifications[0].EventName.Should().Be("Designation");
                var snapshot2 = _snapshot.CreateSnapshot();
                ret = NativeContract.RoleManagement.Call(
                    snapshot2,
                    "getDesignatedByRole",
                    new ContractParameter(ContractParameterType.Integer) { Value = new BigInteger((int)role) },
                    new ContractParameter(ContractParameterType.Integer) { Value = new BigInteger(1u) }
                );
                ret.Should().BeOfType<VM.Types.Array>();
                (ret as VM.Types.Array).Count.Should().Be(2);
                (ret as VM.Types.Array)[0].GetSpan().ToHexString().Should().Be(publicKeys[0].ToArray().ToHexString());
                (ret as VM.Types.Array)[1].GetSpan().ToHexString().Should().Be(publicKeys[1].ToArray().ToHexString());

                ret = NativeContract.RoleManagement.Call(
                    snapshot2,
                    "getDesignatedByRole",
                    new ContractParameter(ContractParameterType.Integer) { Value = new BigInteger((int)role) },
                    new ContractParameter(ContractParameterType.Integer) { Value = new BigInteger(0) }
                );
                ret.Should().BeOfType<VM.Types.Array>();
                (ret as VM.Types.Array).Count.Should().Be(0);
            }
        }

        private void ApplicationEngine_Notify(object sender, NotifyEventArgs e)
        {
            throw new System.NotImplementedException();
        }
    }
}
