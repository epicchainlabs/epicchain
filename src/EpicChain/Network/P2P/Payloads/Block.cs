// Copyright (C) 2021-2024 EpicChain Labs.

//
// Block.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Cryptography;
using EpicChain.IO;
using EpicChain.Json;
using EpicChain.Ledger;
using EpicChain.Persistence;
using System;
using System.IO;
using System.Linq;

namespace EpicChain.Network.P2P.Payloads
{
    /// <summary>
    /// Represents a block.
    /// </summary>
    public sealed class Block : IEquatable<Block>, IInventory
    {
        /// <summary>
        /// The header of the block.
        /// </summary>
        public Header Header;

        /// <summary>
        /// The transaction list of the block.
        /// </summary>
        public Transaction[] Transactions;

        public UInt256 Hash => Header.Hash;

        /// <summary>
        /// The version of the block.
        /// </summary>
        public uint Version => Header.Version;

        /// <summary>
        /// The hash of the previous block.
        /// </summary>
        public UInt256 PrevHash => Header.PrevHash;

        /// <summary>
        /// The merkle root of the transactions.
        /// </summary>
        public UInt256 MerkleRoot => Header.MerkleRoot;

        /// <summary>
        /// The timestamp of the block.
        /// </summary>
        public ulong Timestamp => Header.Timestamp;

        /// <summary>
        /// The random number of the block.
        /// </summary>
        public ulong Nonce => Header.Nonce;

        /// <summary>
        /// The index of the block.
        /// </summary>
        public uint Index => Header.Index;

        /// <summary>
        /// The primary index of the consensus node that generated this block.
        /// </summary>
        public byte PrimaryIndex => Header.PrimaryIndex;

        /// <summary>
        /// The multi-signature address of the consensus nodes that generates the next block.
        /// </summary>
        public UInt160 NextConsensus => Header.NextConsensus;

        /// <summary>
        /// The witness of the block.
        /// </summary>
        public Witness Witness => Header.Witness;

        InventoryType IInventory.InventoryType => InventoryType.Block;
        public int Size => Header.Size + Transactions.GetVarSize();
        Witness[] IVerifiable.Witnesses { get => ((IVerifiable)Header).Witnesses; set => throw new NotSupportedException(); }

        public void Deserialize(ref MemoryReader reader)
        {
            Header = reader.ReadSerializable<Header>();
            Transactions = reader.ReadSerializableArray<Transaction>(ushort.MaxValue);
            if (Transactions.Distinct().Count() != Transactions.Length)
                throw new FormatException();
            if (MerkleTree.ComputeRoot(Transactions.Select(p => p.Hash).ToArray()) != Header.MerkleRoot)
                throw new FormatException();
        }

        void IVerifiable.DeserializeUnsigned(ref MemoryReader reader) => throw new NotSupportedException();

        public bool Equals(Block other)
        {
            if (ReferenceEquals(this, other)) return true;
            if (other is null) return false;
            return Hash.Equals(other.Hash);
        }

        public override bool Equals(object obj)
        {
            return Equals(obj as Block);
        }

        public override int GetHashCode()
        {
            return Hash.GetHashCode();
        }

        UInt160[] IVerifiable.GetScriptHashesForVerifying(DataCache snapshot) => ((IVerifiable)Header).GetScriptHashesForVerifying(snapshot);

        public void Serialize(BinaryWriter writer)
        {
            writer.Write(Header);
            writer.Write(Transactions);
        }

        void IVerifiable.SerializeUnsigned(BinaryWriter writer) => ((IVerifiable)Header).SerializeUnsigned(writer);

        /// <summary>
        /// Converts the block to a JSON object.
        /// </summary>
        /// <param name="settings">The <see cref="ProtocolSettings"/> used during the conversion.</param>
        /// <returns>The block represented by a JSON object.</returns>
        public JObject ToJson(ProtocolSettings settings)
        {
            JObject json = Header.ToJson(settings);
            json["size"] = Size;
            json["tx"] = Transactions.Select(p => p.ToJson(settings)).ToArray();
            return json;
        }

        internal bool Verify(ProtocolSettings settings, DataCache snapshot)
        {
            return Header.Verify(settings, snapshot);
        }

        internal bool Verify(ProtocolSettings settings, DataCache snapshot, HeaderCache headerCache)
        {
            return Header.Verify(settings, snapshot, headerCache);
        }
    }
}
