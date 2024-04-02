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
// Signer.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.Cryptography.ECC;
using Neo.IO;
using Neo.Json;
using Neo.Network.P2P.Payloads.Conditions;
using Neo.SmartContract;
using Neo.VM;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace Neo.Network.P2P.Payloads
{
    /// <summary>
    /// Represents a signer of a <see cref="Transaction"/>.
    /// </summary>
    public class Signer : IInteroperable, ISerializable
    {
        // This limits maximum number of AllowedContracts or AllowedGroups here
        private const int MaxSubitems = 16;

        /// <summary>
        /// The account of the signer.
        /// </summary>
        public UInt160 Account;

        /// <summary>
        /// The scopes of the witness.
        /// </summary>
        public WitnessScope Scopes;

        /// <summary>
        /// The contracts that allowed by the witness.
        /// Only available when the <see cref="WitnessScope.CustomContracts"/> flag is set.
        /// </summary>
        public UInt160[] AllowedContracts;

        /// <summary>
        /// The groups that allowed by the witness.
        /// Only available when the <see cref="WitnessScope.CustomGroups"/> flag is set.
        /// </summary>
        public ECPoint[] AllowedGroups;

        /// <summary>
        /// The rules that the witness must meet.
        /// Only available when the <see cref="WitnessScope.WitnessRules"/> flag is set.
        /// </summary>
        public WitnessRule[] Rules;

        public int Size =>
            /*Account*/             UInt160.Length +
            /*Scopes*/              sizeof(WitnessScope) +
            /*AllowedContracts*/    (Scopes.HasFlag(WitnessScope.CustomContracts) ? AllowedContracts.GetVarSize() : 0) +
            /*AllowedGroups*/       (Scopes.HasFlag(WitnessScope.CustomGroups) ? AllowedGroups.GetVarSize() : 0) +
            /*Rules*/               (Scopes.HasFlag(WitnessScope.WitnessRules) ? Rules.GetVarSize() : 0);

        public void Deserialize(ref MemoryReader reader)
        {
            Account = reader.ReadSerializable<UInt160>();
            Scopes = (WitnessScope)reader.ReadByte();
            if ((Scopes & ~(WitnessScope.CalledByEntry | WitnessScope.CustomContracts | WitnessScope.CustomGroups | WitnessScope.WitnessRules | WitnessScope.Global)) != 0)
                throw new FormatException();
            if (Scopes.HasFlag(WitnessScope.Global) && Scopes != WitnessScope.Global)
                throw new FormatException();
            AllowedContracts = Scopes.HasFlag(WitnessScope.CustomContracts)
                ? reader.ReadSerializableArray<UInt160>(MaxSubitems)
                : Array.Empty<UInt160>();
            AllowedGroups = Scopes.HasFlag(WitnessScope.CustomGroups)
                ? reader.ReadSerializableArray<ECPoint>(MaxSubitems)
                : Array.Empty<ECPoint>();
            Rules = Scopes.HasFlag(WitnessScope.WitnessRules)
                ? reader.ReadSerializableArray<WitnessRule>(MaxSubitems)
                : Array.Empty<WitnessRule>();
        }

        /// <summary>
        /// Converts all rules contained in the <see cref="Signer"/> object to <see cref="WitnessRule"/>.
        /// </summary>
        /// <returns>The <see cref="WitnessRule"/> array used to represent the current signer.</returns>
        public IEnumerable<WitnessRule> GetAllRules()
        {
            if (Scopes == WitnessScope.Global)
            {
                yield return new WitnessRule
                {
                    Action = WitnessRuleAction.Allow,
                    Condition = new BooleanCondition { Expression = true }
                };
            }
            else
            {
                if (Scopes.HasFlag(WitnessScope.CalledByEntry))
                {
                    yield return new WitnessRule
                    {
                        Action = WitnessRuleAction.Allow,
                        Condition = new CalledByEntryCondition()
                    };
                }
                if (Scopes.HasFlag(WitnessScope.CustomContracts))
                {
                    foreach (UInt160 hash in AllowedContracts)
                        yield return new WitnessRule
                        {
                            Action = WitnessRuleAction.Allow,
                            Condition = new ScriptHashCondition { Hash = hash }
                        };
                }
                if (Scopes.HasFlag(WitnessScope.CustomGroups))
                {
                    foreach (ECPoint group in AllowedGroups)
                        yield return new WitnessRule
                        {
                            Action = WitnessRuleAction.Allow,
                            Condition = new GroupCondition { Group = group }
                        };
                }
                if (Scopes.HasFlag(WitnessScope.WitnessRules))
                {
                    foreach (WitnessRule rule in Rules)
                        yield return rule;
                }
            }
        }

        public void Serialize(BinaryWriter writer)
        {
            writer.Write(Account);
            writer.Write((byte)Scopes);
            if (Scopes.HasFlag(WitnessScope.CustomContracts))
                writer.Write(AllowedContracts);
            if (Scopes.HasFlag(WitnessScope.CustomGroups))
                writer.Write(AllowedGroups);
            if (Scopes.HasFlag(WitnessScope.WitnessRules))
                writer.Write(Rules);
        }

        /// <summary>
        /// Converts the signer from a JSON object.
        /// </summary>
        /// <param name="json">The signer represented by a JSON object.</param>
        /// <returns>The converted signer.</returns>
        public static Signer FromJson(JObject json)
        {
            Signer signer = new();
            signer.Account = UInt160.Parse(json["account"].GetString());
            signer.Scopes = Enum.Parse<WitnessScope>(json["scopes"].GetString());
            if (signer.Scopes.HasFlag(WitnessScope.CustomContracts))
                signer.AllowedContracts = ((JArray)json["allowedcontracts"]).Select(p => UInt160.Parse(p.GetString())).ToArray();
            if (signer.Scopes.HasFlag(WitnessScope.CustomGroups))
                signer.AllowedGroups = ((JArray)json["allowedgroups"]).Select(p => ECPoint.Parse(p.GetString(), ECCurve.Secp256r1)).ToArray();
            if (signer.Scopes.HasFlag(WitnessScope.WitnessRules))
                signer.Rules = ((JArray)json["rules"]).Select(p => WitnessRule.FromJson((JObject)p)).ToArray();
            return signer;
        }

        /// <summary>
        /// Converts the signer to a JSON object.
        /// </summary>
        /// <returns>The signer represented by a JSON object.</returns>
        public JObject ToJson()
        {
            var json = new JObject();
            json["account"] = Account.ToString();
            json["scopes"] = Scopes;
            if (Scopes.HasFlag(WitnessScope.CustomContracts))
                json["allowedcontracts"] = AllowedContracts.Select(p => (JToken)p.ToString()).ToArray();
            if (Scopes.HasFlag(WitnessScope.CustomGroups))
                json["allowedgroups"] = AllowedGroups.Select(p => (JToken)p.ToString()).ToArray();
            if (Scopes.HasFlag(WitnessScope.WitnessRules))
                json["rules"] = Rules.Select(p => p.ToJson()).ToArray();
            return json;
        }

        void IInteroperable.FromStackItem(VM.Types.StackItem stackItem)
        {
            throw new NotSupportedException();
        }

        VM.Types.StackItem IInteroperable.ToStackItem(ReferenceCounter referenceCounter)
        {
            return new VM.Types.Array(referenceCounter, new VM.Types.StackItem[]
            {
                Account.ToArray(),
                (byte)Scopes,
                new VM.Types.Array(referenceCounter, AllowedContracts.Select(u => new VM.Types.ByteString(u.ToArray()))),
                new VM.Types.Array(referenceCounter, AllowedGroups.Select(u => new VM.Types.ByteString(u.ToArray()))),
                new VM.Types.Array(referenceCounter, Rules.Select(u => u.ToStackItem(referenceCounter)))
            });
        }
    }
}
