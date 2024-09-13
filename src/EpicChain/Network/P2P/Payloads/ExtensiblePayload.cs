// Copyright (C) 2021-2024 EpicChain Labs.

//
// ExtensiblePayload.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.IO;
using EpicChain.Persistence;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using System;
using System.Collections.Generic;
using System.IO;

namespace EpicChain.Network.P2P.Payloads
{
    /// <summary>
    /// Represents an extensible message that can be relayed.
    /// </summary>
    public class ExtensiblePayload : IInventory
    {
        /// <summary>
        /// The category of the extension.
        /// </summary>
        public string Category;

        /// <summary>
        /// Indicates that the payload is only valid when the block height is greater than or equal to this value.
        /// </summary>
        public uint ValidBlockStart;

        /// <summary>
        /// Indicates that the payload is only valid when the block height is less than this value.
        /// </summary>
        public uint ValidBlockEnd;

        /// <summary>
        /// The sender of the payload.
        /// </summary>
        public UInt160 Sender;

        /// <summary>
        /// The data of the payload.
        /// </summary>
        public ReadOnlyMemory<byte> Data;

        /// <summary>
        /// The witness of the payload. It must match the <see cref="Sender"/>.
        /// </summary>
        public Witness Witness;

        private UInt256 _hash = null;
        public UInt256 Hash
        {
            get
            {
                if (_hash == null)
                {
                    _hash = this.CalculateHash();
                }
                return _hash;
            }
        }

        InventoryType IInventory.InventoryType => InventoryType.Extensible;

        public int Size =>
            Category.GetVarSize() + //Category
            sizeof(uint) +          //ValidBlockStart
            sizeof(uint) +          //ValidBlockEnd
            UInt160.Length +        //Sender
            Data.GetVarSize() +     //Data
            1 + Witness.Size;       //Witness

        Witness[] IVerifiable.Witnesses
        {
            get
            {
                return new[] { Witness };
            }
            set
            {
                if (value.Length != 1) throw new ArgumentException();
                Witness = value[0];
            }
        }

        void ISerializable.Deserialize(ref MemoryReader reader)
        {
            ((IVerifiable)this).DeserializeUnsigned(ref reader);
            if (reader.ReadByte() != 1) throw new FormatException();
            Witness = reader.ReadSerializable<Witness>();
        }

        void IVerifiable.DeserializeUnsigned(ref MemoryReader reader)
        {
            Category = reader.ReadVarString(32);
            ValidBlockStart = reader.ReadUInt32();
            ValidBlockEnd = reader.ReadUInt32();
            if (ValidBlockStart >= ValidBlockEnd) throw new FormatException();
            Sender = reader.ReadSerializable<UInt160>();
            Data = reader.ReadVarMemory();
        }

        UInt160[] IVerifiable.GetScriptHashesForVerifying(DataCache snapshot)
        {
            return new[] { Sender }; // This address should be checked by consumer
        }

        void ISerializable.Serialize(BinaryWriter writer)
        {
            ((IVerifiable)this).SerializeUnsigned(writer);
            writer.Write((byte)1); writer.Write(Witness);
        }

        void IVerifiable.SerializeUnsigned(BinaryWriter writer)
        {
            writer.WriteVarString(Category);
            writer.Write(ValidBlockStart);
            writer.Write(ValidBlockEnd);
            writer.Write(Sender);
            writer.WriteVarBytes(Data.Span);
        }

        internal bool Verify(ProtocolSettings settings, DataCache snapshot, ISet<UInt160> extensibleWitnessWhiteList)
        {
            uint height = NativeContract.Ledger.CurrentIndex(snapshot);
            if (height < ValidBlockStart || height >= ValidBlockEnd) return false;
            if (!extensibleWitnessWhiteList.Contains(Sender)) return false;
            return this.VerifyWitnesses(settings, snapshot, 0_06000000L);
        }
    }
}
