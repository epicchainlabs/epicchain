// Copyright (C) 2021-2024 EpicChain Labs.

//
// StateRoot.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Network.P2P;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Persistence;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using System;
using System.IO;

namespace EpicChain.Plugins.StateService.Network
{
    class StateRoot : IVerifiable
    {
        public const byte CurrentVersion = 0x00;

        public byte Version;
        public uint Index;
        public UInt256 RootHash;
        public Witness Witness;

        private UInt256 _hash = null;
        public UInt256 Hash
        {
            get
            {
                if (_hash is null)
                {
                    _hash = this.CalculateHash();
                }
                return _hash;
            }
        }

        Witness[] IVerifiable.Witnesses
        {
            get
            {
                return new[] { Witness };
            }
            set
            {
                if (value.Length != 1) throw new ArgumentException(null, nameof(value));
                Witness = value[0];
            }
        }

        int ISerializable.Size =>
            sizeof(byte) +      //Version
            sizeof(uint) +      //Index
            UInt256.Length +    //RootHash
            (Witness is null ? 1 : 1 + Witness.Size); //Witness

        void ISerializable.Deserialize(ref MemoryReader reader)
        {
            DeserializeUnsigned(ref reader);
            Witness[] witnesses = reader.ReadSerializableArray<Witness>(1);
            Witness = witnesses.Length switch
            {
                0 => null,
                1 => witnesses[0],
                _ => throw new FormatException(),
            };
        }

        public void DeserializeUnsigned(ref MemoryReader reader)
        {
            Version = reader.ReadByte();
            Index = reader.ReadUInt32();
            RootHash = reader.ReadSerializable<UInt256>();
        }

        void ISerializable.Serialize(BinaryWriter writer)
        {
            SerializeUnsigned(writer);
            if (Witness is null)
                writer.WriteVarInt(0);
            else
                writer.Write(new[] { Witness });
        }

        public void SerializeUnsigned(BinaryWriter writer)
        {
            writer.Write(Version);
            writer.Write(Index);
            writer.Write(RootHash);
        }

        public bool Verify(ProtocolSettings settings, DataCache snapshot)
        {
            return this.VerifyWitnesses(settings, snapshot, 2_00000000L);
        }

        public UInt160[] GetScriptHashesForVerifying(DataCache snapshot)
        {
            ECPoint[] validators = NativeContract.QuantumGuardNexus.GetDesignatedByRole(snapshot, Role.StateValidator, Index);
            if (validators.Length < 1) throw new InvalidOperationException("No script hash for state root verifying");
            return new UInt160[] { Contract.GetBFTAddress(validators) };
        }

        public JObject ToJson()
        {
            var json = new JObject();
            json["version"] = Version;
            json["index"] = Index;
            json["roothash"] = RootHash.ToString();
            json["witnesses"] = Witness is null ? new JArray() : new JArray(Witness.ToJson());
            return json;
        }
    }
}
