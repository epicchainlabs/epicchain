// Copyright (C) 2021-2024 EpicChain Labs.

//
// Signer.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Cryptography.ECC;
using EpicChain.IO;
using EpicChain.Json;
using EpicChain.Network.P2P.Payloads.Conditions;
using EpicChain.SmartContract;
using EpicChain.VM;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace EpicChain.Network.P2P.Payloads
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
            Signer signer = new()
            {
                Account = UInt160.Parse(json["account"].GetString()),
                Scopes = Enum.Parse<WitnessScope>(json["scopes"].GetString())
            };
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
            return new VM.Types.Array(referenceCounter,
            [
                Account.ToArray(),
                (byte)Scopes,
                Scopes.HasFlag(WitnessScope.CustomContracts) ? new VM.Types.Array(referenceCounter, AllowedContracts.Select(u => new VM.Types.ByteString(u.ToArray()))) : new VM.Types.Array(referenceCounter),
                Scopes.HasFlag(WitnessScope.CustomGroups) ? new VM.Types.Array(referenceCounter, AllowedGroups.Select(u => new VM.Types.ByteString(u.ToArray()))) : new VM.Types.Array(referenceCounter),
                Scopes.HasFlag(WitnessScope.WitnessRules) ? new VM.Types.Array(referenceCounter, Rules.Select(u => u.ToStackItem(referenceCounter))) : new VM.Types.Array(referenceCounter)
            ]);
        }
    }
}
