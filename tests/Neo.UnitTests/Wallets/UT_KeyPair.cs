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
// UT_KeyPair.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.Cryptography;
using Neo.Cryptography.ECC;
using Neo.Wallets;
using System;
using System.Linq;

namespace Neo.UnitTests.Wallets
{
    [TestClass]
    public class UT_KeyPair
    {
        [TestMethod]
        public void TestConstructor()
        {
            Random random = new Random();
            byte[] privateKey = new byte[32];
            for (int i = 0; i < privateKey.Length; i++)
                privateKey[i] = (byte)random.Next(256);
            KeyPair keyPair = new KeyPair(privateKey);
            ECPoint publicKey = ECCurve.Secp256r1.G * privateKey;
            keyPair.PrivateKey.Should().BeEquivalentTo(privateKey);
            keyPair.PublicKey.Should().Be(publicKey);

            byte[] privateKey96 = new byte[96];
            for (int i = 0; i < privateKey96.Length; i++)
                privateKey96[i] = (byte)random.Next(256);
            keyPair = new KeyPair(privateKey96);
            publicKey = ECPoint.DecodePoint(new byte[] { 0x04 }.Concat(privateKey96.Skip(privateKey96.Length - 96).Take(64)).ToArray(), ECCurve.Secp256r1);
            keyPair.PrivateKey.Should().BeEquivalentTo(privateKey96.Skip(64).Take(32));
            keyPair.PublicKey.Should().Be(publicKey);

            byte[] privateKey31 = new byte[31];
            for (int i = 0; i < privateKey31.Length; i++)
                privateKey31[i] = (byte)random.Next(256);
            Action action = () => new KeyPair(privateKey31);
            action.Should().Throw<ArgumentException>();
        }

        [TestMethod]
        public void TestEquals()
        {
            Random random = new Random();
            byte[] privateKey = new byte[32];
            for (int i = 0; i < privateKey.Length; i++)
                privateKey[i] = (byte)random.Next(256);
            KeyPair keyPair = new KeyPair(privateKey);
            KeyPair keyPair2 = keyPair;
            keyPair.Equals(keyPair2).Should().BeTrue();

            KeyPair keyPair3 = null;
            keyPair.Equals(keyPair3).Should().BeFalse();

            byte[] privateKey1 = { 0x01,0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01};
            byte[] privateKey2 = { 0x01,0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x02};
            KeyPair keyPair4 = new KeyPair(privateKey1);
            KeyPair keyPair5 = new KeyPair(privateKey2);
            keyPair4.Equals(keyPair5).Should().BeFalse();
        }

        [TestMethod]
        public void TestEqualsWithObj()
        {
            Random random = new Random();
            byte[] privateKey = new byte[32];
            for (int i = 0; i < privateKey.Length; i++)
                privateKey[i] = (byte)random.Next(256);
            KeyPair keyPair = new KeyPair(privateKey);
            Object keyPair2 = keyPair;
            keyPair.Equals(keyPair2).Should().BeTrue();
        }

        [TestMethod]
        public void TestExport()
        {
            byte[] privateKey = { 0x01,0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01};
            byte[] data = { 0x80, 0x01,0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01};
            KeyPair keyPair = new KeyPair(privateKey);
            keyPair.Export().Should().Be(Base58.Base58CheckEncode(data));
        }

        [TestMethod]
        public void TestGetPublicKeyHash()
        {
            byte[] privateKey = { 0x01,0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01};
            KeyPair keyPair = new KeyPair(privateKey);
            keyPair.PublicKeyHash.ToString().Should().Be("0x4ab3d6ac3a0609e87af84599c93d57c2d0890406");
        }

        [TestMethod]
        public void TestGetHashCode()
        {
            byte[] privateKey = { 0x01,0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01};
            KeyPair keyPair1 = new KeyPair(privateKey);
            KeyPair keyPair2 = new KeyPair(privateKey);
            keyPair1.GetHashCode().Should().Be(keyPair2.GetHashCode());
        }

        [TestMethod]
        public void TestToString()
        {
            byte[] privateKey = { 0x01,0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01};
            KeyPair keyPair = new KeyPair(privateKey);
            keyPair.ToString().Should().Be("026ff03b949241ce1dadd43519e6960e0a85b41a69a05c328103aa2bce1594ca16");
        }
    }
}
