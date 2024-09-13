// Copyright (C) 2021-2024 EpicChain Labs.

//
// CalledByGroupCondition.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using EpicChain.VM.Types;
using System.IO;
using System.Linq;

namespace EpicChain.Network.P2P.Payloads.Conditions
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
            ContractState contract = NativeContract.ContractManagement.GetContract(engine.SnapshotCache, engine.CallingScriptHash);
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
