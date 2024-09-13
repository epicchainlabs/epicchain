// Copyright (C) 2021-2024 EpicChain Labs.

//
// TrimmedBlock.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Network.P2P.Payloads;
using EpicChain.VM;
using EpicChain.VM.Types;
using System;
using System.IO;
using System.Linq;

namespace EpicChain.SmartContract.Native
{
    /// <summary>
    /// Represents a block which the transactions are trimmed.
    /// </summary>
    public class TrimmedBlock : IInteroperable, ISerializable
    {
        /// <summary>
        /// The header of the block.
        /// </summary>
        public Header Header;

        /// <summary>
        /// The hashes of the transactions of the block.
        /// </summary>
        public UInt256[] Hashes;

        /// <summary>
        /// The hash of the block.
        /// </summary>
        public UInt256 Hash => Header.Hash;

        /// <summary>
        /// The index of the block.
        /// </summary>
        public uint Index => Header.Index;

        public int Size => Header.Size + Hashes.GetVarSize();

        public void Deserialize(ref MemoryReader reader)
        {
            Header = reader.ReadSerializable<Header>();
            Hashes = reader.ReadSerializableArray<UInt256>(ushort.MaxValue);
        }

        public void Serialize(BinaryWriter writer)
        {
            writer.Write(Header);
            writer.Write(Hashes);
        }

        IInteroperable IInteroperable.Clone()
        {
            return new TrimmedBlock
            {
                Header = Header,
                Hashes = Hashes
            };
        }

        void IInteroperable.FromReplica(IInteroperable replica)
        {
            TrimmedBlock from = (TrimmedBlock)replica;
            Header = from.Header;
            Hashes = from.Hashes;
        }

        void IInteroperable.FromStackItem(StackItem stackItem)
        {
            throw new NotSupportedException();
        }

        StackItem IInteroperable.ToStackItem(ReferenceCounter referenceCounter)
        {
            return new VM.Types.Array(referenceCounter, new StackItem[]
            {
                // Computed properties
                Header.Hash.ToArray(),

                // BlockBase properties
                Header.Version,
                Header.PrevHash.ToArray(),
                Header.MerkleRoot.ToArray(),
                Header.Timestamp,
                Header.Nonce,
                Header.Index,
                Header.PrimaryIndex,
                Header.NextConsensus.ToArray(),

                // Block properties
                Hashes.Length
            });
        }
    }
}
