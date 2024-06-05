// Copyright (C) 2021-2024 The EpicChain Labs.
//
// BooleanCondition.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
using System.IO;

namespace Neo.Network.P2P.Payloads.Conditions
{
    public class BooleanCondition : WitnessCondition
    {
        /// <summary>
        /// The expression of the <see cref="BooleanCondition"/>.
        /// </summary>
        public bool Expression;

        public override int Size => base.Size + sizeof(bool);
        public override WitnessConditionType Type => WitnessConditionType.Boolean;

        protected override void DeserializeWithoutType(ref MemoryReader reader, int maxNestDepth)
        {
            Expression = reader.ReadBoolean();
        }

        public override bool Match(ApplicationEngine engine)
        {
            return Expression;
        }

        protected override void SerializeWithoutType(BinaryWriter writer)
        {
            writer.Write(Expression);
        }

        private protected override void ParseJson(JObject json, int maxNestDepth)
        {
            Expression = json["expression"].GetBoolean();
        }

        public override JObject ToJson()
        {
            JObject json = base.ToJson();
            json["expression"] = Expression;
            return json;
        }

        public override StackItem ToStackItem(ReferenceCounter referenceCounter)
        {
            var result = (VM.Types.Array)base.ToStackItem(referenceCounter);
            result.Add(Expression);
            return result;
        }
    }
}
