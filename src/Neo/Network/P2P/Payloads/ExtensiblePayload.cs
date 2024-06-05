// Copyright (C) 2021-2024 The EpicChain Labs.
//
// ExtensiblePayload.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
using Neo.Persistence;
using Neo.SmartContract;
using Neo.SmartContract.Native;
using System;
using System.Collections.Generic;
using System.IO;

namespace Neo.Network.P2P.Payloads
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
