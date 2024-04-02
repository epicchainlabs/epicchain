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
// Block.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.Cryptography;
using Neo.IO;
using Neo.Json;
using Neo.Ledger;
using Neo.Persistence;
using System;
using System.IO;
using System.Linq;

namespace Neo.Network.P2P.Payloads
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
