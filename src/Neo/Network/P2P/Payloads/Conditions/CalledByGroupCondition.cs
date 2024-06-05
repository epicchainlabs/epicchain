// Copyright (C) 2021-2024 The EpicChain Labs.
//
// CalledByGroupCondition.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.Cryptography.ECC;
using Neo.IO;
using Neo.Json;
using Neo.SmartContract;
using Neo.SmartContract.Native;
using Neo.VM;
using Neo.VM.Types;
using System.IO;
using System.Linq;

namespace Neo.Network.P2P.Payloads.Conditions
{
    public class CalledByGroupCondition : WitnessCondition
    {
        /// <summary>
        /// The group to be checked.
        /// </summary>
        public ECPoint Group;

        public override int Size => base.Size + Group.Size;
        public override WitnessConditionType Type => WitnessConditionType.CalledByGroup;

        protected override void DeserializeWithoutType(ref MemoryReader reader, int maxNestDepth)
        {
            Group = reader.ReadSerializable<ECPoint>();
        }

        public override bool Match(ApplicationEngine engine)
        {
            engine.ValidateCallFlags(CallFlags.ReadStates);
            ContractState contract = NativeContract.ContractManagement.GetContract(engine.Snapshot, engine.CallingScriptHash);
            return contract is not null && contract.Manifest.Groups.Any(p => p.PubKey.Equals(Group));
        }

        protected override void SerializeWithoutType(BinaryWriter writer)
        {
            writer.Write(Group);
        }

        private protected override void ParseJson(JObject json, int maxNestDepth)
        {
            Group = ECPoint.Parse(json["group"].GetString(), ECCurve.Secp256r1);
        }

        public override JObject ToJson()
        {
            JObject json = base.ToJson();
            json["group"] = Group.ToString();
            return json;
        }

        public override StackItem ToStackItem(ReferenceCounter referenceCounter)
        {
            var result = (VM.Types.Array)base.ToStackItem(referenceCounter);
            result.Add(Group.ToArray());
            return result;
        }
    }
}
