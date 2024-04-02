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
// UT_Signers.cs file belongs to the neo project and is free
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
using Neo.Network.P2P.Payloads.Conditions;
using System;

namespace Neo.UnitTests.Network.P2P.Payloads
{
    [TestClass]
    public class UT_Signers
    {
        [TestMethod]
        public void Serialize_Deserialize_Global()
        {
            var attr = new Signer()
            {
                Scopes = WitnessScope.Global,
                Account = UInt160.Zero
            };

            var hex = "000000000000000000000000000000000000000080";
            attr.ToArray().ToHexString().Should().Be(hex);

            var copy = hex.HexToBytes().AsSerializable<Signer>();

            Assert.AreEqual(attr.Scopes, copy.Scopes);
            Assert.AreEqual(attr.Account, copy.Account);
        }

        [TestMethod]
        public void Serialize_Deserialize_CalledByEntry()
        {
            var attr = new Signer()
            {
                Scopes = WitnessScope.CalledByEntry,
                Account = UInt160.Zero
            };

            var hex = "000000000000000000000000000000000000000001";
            attr.ToArray().ToHexString().Should().Be(hex);

            var copy = hex.HexToBytes().AsSerializable<Signer>();

            Assert.AreEqual(attr.Scopes, copy.Scopes);
            Assert.AreEqual(attr.Account, copy.Account);
        }

        [TestMethod]
        public void Serialize_Deserialize_MaxNested_And()
        {
            var attr = new Signer()
            {
                Scopes = WitnessScope.WitnessRules,
                Account = UInt160.Zero,
                Rules = new WitnessRule[]{ new WitnessRule()
                {
                    Action = WitnessRuleAction.Allow,
                    Condition = new AndCondition()
                    {
                        Expressions = new WitnessCondition[]
                        {
                            new AndCondition()
                            {
                                Expressions = new WitnessCondition[]
                                {
                                    new AndCondition()
                                    {
                                        Expressions = new WitnessCondition[]
                                        {
                                            new BooleanCondition() { Expression=true }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }}
            };

            var hex = "00000000000000000000000000000000000000004001010201020102010001";
            attr.ToArray().ToHexString().Should().Be(hex);

            Assert.ThrowsException<FormatException>(() => hex.HexToBytes().AsSerializable<Signer>());
        }

        [TestMethod]
        public void Serialize_Deserialize_MaxNested_Or()
        {
            var attr = new Signer()
            {
                Scopes = WitnessScope.WitnessRules,
                Account = UInt160.Zero,
                Rules = new WitnessRule[]{ new WitnessRule()
                {
                    Action = WitnessRuleAction.Allow,
                    Condition = new OrCondition()
                    {
                        Expressions = new WitnessCondition[]
                        {
                            new OrCondition()
                            {
                                Expressions = new WitnessCondition[]
                                {
                                    new OrCondition()
                                    {
                                        Expressions = new WitnessCondition[]
                                        {
                                            new BooleanCondition() { Expression=true }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }}
            };

            var hex = "00000000000000000000000000000000000000004001010301030103010001";
            attr.ToArray().ToHexString().Should().Be(hex);

            Assert.ThrowsException<FormatException>(() => hex.HexToBytes().AsSerializable<Signer>());
        }

        [TestMethod]
        public void Serialize_Deserialize_CustomContracts()
        {
            var attr = new Signer()
            {
                Scopes = WitnessScope.CustomContracts,
                AllowedContracts = new[] { UInt160.Zero },
                Account = UInt160.Zero
            };

            var hex = "000000000000000000000000000000000000000010010000000000000000000000000000000000000000";
            attr.ToArray().ToHexString().Should().Be(hex);

            var copy = hex.HexToBytes().AsSerializable<Signer>();

            Assert.AreEqual(attr.Scopes, copy.Scopes);
            CollectionAssert.AreEqual(attr.AllowedContracts, copy.AllowedContracts);
            Assert.AreEqual(attr.Account, copy.Account);
        }

        [TestMethod]
        public void Serialize_Deserialize_CustomGroups()
        {
            var attr = new Signer()
            {
                Scopes = WitnessScope.CustomGroups,
                AllowedGroups = new[] { ECPoint.Parse("03b209fd4f53a7170ea4444e0cb0a6bb6a53c2bd016926989cf85f9b0fba17a70c", ECCurve.Secp256r1) },
                Account = UInt160.Zero
            };

            var hex = "0000000000000000000000000000000000000000200103b209fd4f53a7170ea4444e0cb0a6bb6a53c2bd016926989cf85f9b0fba17a70c";
            attr.ToArray().ToHexString().Should().Be(hex);

            var copy = hex.HexToBytes().AsSerializable<Signer>();

            Assert.AreEqual(attr.Scopes, copy.Scopes);
            CollectionAssert.AreEqual(attr.AllowedGroups, copy.AllowedGroups);
            Assert.AreEqual(attr.Account, copy.Account);
        }

        [TestMethod]
        public void Json_Global()
        {
            var attr = new Signer()
            {
                Scopes = WitnessScope.Global,
                Account = UInt160.Zero
            };

            var json = "{\"account\":\"0x0000000000000000000000000000000000000000\",\"scopes\":\"Global\"}";
            attr.ToJson().ToString().Should().Be(json);
        }

        [TestMethod]
        public void Json_CalledByEntry()
        {
            var attr = new Signer()
            {
                Scopes = WitnessScope.CalledByEntry,
                Account = UInt160.Zero
            };

            var json = "{\"account\":\"0x0000000000000000000000000000000000000000\",\"scopes\":\"CalledByEntry\"}";
            attr.ToJson().ToString().Should().Be(json);
        }

        [TestMethod]
        public void Json_CustomContracts()
        {
            var attr = new Signer()
            {
                Scopes = WitnessScope.CustomContracts,
                AllowedContracts = new[] { UInt160.Zero },
                Account = UInt160.Zero
            };

            var json = "{\"account\":\"0x0000000000000000000000000000000000000000\",\"scopes\":\"CustomContracts\",\"allowedcontracts\":[\"0x0000000000000000000000000000000000000000\"]}";
            attr.ToJson().ToString().Should().Be(json);
        }

        [TestMethod]
        public void Json_CustomGroups()
        {
            var attr = new Signer()
            {
                Scopes = WitnessScope.CustomGroups,
                AllowedGroups = new[] { ECPoint.Parse("03b209fd4f53a7170ea4444e0cb0a6bb6a53c2bd016926989cf85f9b0fba17a70c", ECCurve.Secp256r1) },
                Account = UInt160.Zero
            };

            var json = "{\"account\":\"0x0000000000000000000000000000000000000000\",\"scopes\":\"CustomGroups\",\"allowedgroups\":[\"03b209fd4f53a7170ea4444e0cb0a6bb6a53c2bd016926989cf85f9b0fba17a70c\"]}";
            attr.ToJson().ToString().Should().Be(json);
        }

        [TestMethod]
        public void Json_From()
        {
            Signer signer = new()
            {
                Account = UInt160.Zero,
                Scopes = WitnessScope.CustomContracts | WitnessScope.CustomGroups | WitnessScope.WitnessRules,
                AllowedContracts = new[] { UInt160.Zero },
                AllowedGroups = new[] { ECPoint.Parse("03b209fd4f53a7170ea4444e0cb0a6bb6a53c2bd016926989cf85f9b0fba17a70c", ECCurve.Secp256r1) },
                Rules = new WitnessRule[] { new() { Action = WitnessRuleAction.Allow, Condition = new BooleanCondition { Expression = true } } }
            };
            var json = signer.ToJson();
            var new_signer = Signer.FromJson(json);
            Assert.IsTrue(new_signer.Account.Equals(signer.Account));
            Assert.IsTrue(new_signer.Scopes == signer.Scopes);
            Assert.AreEqual(1, new_signer.AllowedContracts.Length);
            Assert.IsTrue(new_signer.AllowedContracts[0].Equals(signer.AllowedContracts[0]));
            Assert.AreEqual(1, new_signer.AllowedGroups.Length);
            Assert.IsTrue(new_signer.AllowedGroups[0].Equals(signer.AllowedGroups[0]));
        }
    }
}
