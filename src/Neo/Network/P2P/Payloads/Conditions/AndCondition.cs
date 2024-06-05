// Copyright (C) 2021-2024 The EpicChain Labs.
//
// AndCondition.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.IO;
using Neo.Json;
using Neo.SmartContract;
using Neo.VM;
using Neo.VM.Types;
using System;
using System.IO;
using System.Linq;

namespace Neo.Network.P2P.Payloads.Conditions
{
    /// <summary>
    /// Represents the condition that all conditions must be met.
    /// </summary>
    public class AndCondition : WitnessCondition
    {
        /// <summary>
        /// The expressions of the condition.
        /// </summary>
        public WitnessCondition[] Expressions;

        public override int Size => base.Size + Expressions.GetVarSize();
        public override WitnessConditionType Type => WitnessConditionType.And;

        protected override void DeserializeWithoutType(ref MemoryReader reader, int maxNestDepth)
        {
            if (maxNestDepth <= 0) throw new FormatException();
            Expressions = DeserializeConditions(ref reader, maxNestDepth - 1);
            if (Expressions.Length == 0) throw new FormatException();
        }

        public override bool Match(ApplicationEngine engine)
        {
            return Expressions.All(p => p.Match(engine));
        }

        protected override void SerializeWithoutType(BinaryWriter writer)
        {
            writer.Write(Expressions);
        }

        private protected override void ParseJson(JObject json, int maxNestDepth)
        {
            if (maxNestDepth <= 0) throw new FormatException();
            JArray expressions = (JArray)json["expressions"];
            if (expressions.Count > MaxSubitems) throw new FormatException();
            Expressions = expressions.Select(p => FromJson((JObject)p, maxNestDepth - 1)).ToArray();
            if (Expressions.Length == 0) throw new FormatException();
        }

        public override JObject ToJson()
        {
            JObject json = base.ToJson();
            json["expressions"] = Expressions.Select(p => p.ToJson()).ToArray();
            return json;
        }

        public override StackItem ToStackItem(ReferenceCounter referenceCounter)
        {
            var result = (VM.Types.Array)base.ToStackItem(referenceCounter);
            result.Add(new VM.Types.Array(referenceCounter, Expressions.Select(p => p.ToStackItem(referenceCounter))));
            return result;
        }
    }
}
