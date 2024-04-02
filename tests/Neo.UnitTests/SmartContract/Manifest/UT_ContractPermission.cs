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
// UT_ContractPermission.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.Cryptography.ECC;
using Neo.SmartContract;
using Neo.SmartContract.Manifest;
using Neo.VM.Types;
using System;

namespace Neo.UnitTests.SmartContract.Manifest
{
    [TestClass]
    public class UT_ContractPermission
    {
        [TestMethod]
        public void TestDeserialize()
        {
            // null
            ContractPermission contractPermission = ContractPermission.DefaultPermission;
            Struct s = (Struct)contractPermission.ToStackItem(new VM.ReferenceCounter());

            contractPermission = s.ToInteroperable<ContractPermission>();
            Assert.IsTrue(contractPermission.Contract.IsWildcard);
            Assert.IsTrue(contractPermission.Methods.IsWildcard);

            // not null
            contractPermission = new ContractPermission()
            {
                Contract = ContractPermissionDescriptor.Create(UInt160.Zero),
                Methods = WildcardContainer<string>.Create("test")
            };
            s = (Struct)contractPermission.ToStackItem(new VM.ReferenceCounter());

            contractPermission = s.ToInteroperable<ContractPermission>();
            Assert.IsFalse(contractPermission.Contract.IsWildcard);
            Assert.IsFalse(contractPermission.Methods.IsWildcard);
            Assert.AreEqual(UInt160.Zero, contractPermission.Contract.Hash);
            Assert.AreEqual("test", contractPermission.Methods[0]);
        }

        [TestMethod]
        public void TestIsAllowed()
        {
            ContractManifest contractManifest1 = TestUtils.CreateDefaultManifest();
            ContractPermission contractPermission1 = ContractPermission.DefaultPermission;
            contractPermission1.Contract = ContractPermissionDescriptor.Create(UInt160.Zero);
            Assert.AreEqual(true, contractPermission1.IsAllowed(new ContractState() { Hash = UInt160.Zero, Manifest = contractManifest1 }, "AAA"));
            contractPermission1.Contract = ContractPermissionDescriptor.CreateWildcard();

            ContractManifest contractManifest2 = TestUtils.CreateDefaultManifest();
            ContractPermission contractPermission2 = ContractPermission.DefaultPermission;
            contractPermission2.Contract = ContractPermissionDescriptor.Create(UInt160.Parse("0x0000000000000000000000000000000000000001"));
            Assert.AreEqual(false, contractPermission2.IsAllowed(new ContractState() { Hash = UInt160.Zero, Manifest = contractManifest2 }, "AAA"));
            contractPermission2.Contract = ContractPermissionDescriptor.CreateWildcard();

            Random random = new();
            byte[] privateKey3 = new byte[32];
            random.NextBytes(privateKey3);
            ECPoint publicKey3 = ECCurve.Secp256r1.G * privateKey3;
            ContractManifest contractManifest3 = TestUtils.CreateDefaultManifest();
            contractManifest3.Groups = new ContractGroup[] { new ContractGroup() { PubKey = publicKey3 } };
            ContractPermission contractPermission3 = ContractPermission.DefaultPermission;
            contractPermission3.Contract = ContractPermissionDescriptor.Create(publicKey3);
            Assert.AreEqual(true, contractPermission3.IsAllowed(new ContractState() { Hash = UInt160.Zero, Manifest = contractManifest3 }, "AAA"));
            contractPermission3.Contract = ContractPermissionDescriptor.CreateWildcard();

            byte[] privateKey41 = new byte[32];
            random.NextBytes(privateKey41);
            ECPoint publicKey41 = ECCurve.Secp256r1.G * privateKey41;
            byte[] privateKey42 = new byte[32];
            random.NextBytes(privateKey42);
            ECPoint publicKey42 = ECCurve.Secp256r1.G * privateKey42;
            ContractManifest contractManifest4 = TestUtils.CreateDefaultManifest();
            contractManifest4.Groups = new ContractGroup[] { new ContractGroup() { PubKey = publicKey42 } };
            ContractPermission contractPermission4 = ContractPermission.DefaultPermission;
            contractPermission4.Contract = ContractPermissionDescriptor.Create(publicKey41);
            Assert.AreEqual(false, contractPermission4.IsAllowed(new ContractState() { Hash = UInt160.Zero, Manifest = contractManifest4 }, "AAA"));
            contractPermission4.Contract = ContractPermissionDescriptor.CreateWildcard();
        }
    }
}
