// Copyright (C) 2021-2024 EpicChain Labs.

//
// Node.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System;
using System.IO;

namespace EpicChain.Cryptography.MPTTrie
{
    public partial class Node : ISerializable
    {
        private NodeType type;
        private UInt256 hash;
        public int Reference;
        public UInt256 Hash => hash ??= new UInt256(Crypto.Hash256(ToArrayWithoutReference()));
        public NodeType Type => type;
        public bool IsEmpty => type == NodeType.Empty;
        public int Size
        {
            get
            {
                int size = sizeof(NodeType);
                switch (type)
                {
                    case NodeType.BranchNode:
                        return size + BranchSize + IO.Helper.GetVarSize(Reference);
                    case NodeType.ExtensionNode:
                        return size + ExtensionSize + IO.Helper.GetVarSize(Reference);
                    case NodeType.LeafNode:
                        return size + LeafSize + IO.Helper.GetVarSize(Reference);
                    case NodeType.HashNode:
                        return size + HashSize;
                    case NodeType.Empty:
                        return size;
                    default:
                        throw new InvalidOperationException($"{nameof(Node)} Cannt get size, unsupport type");
                };
            }
        }

        public Node()
        {
            type = NodeType.Empty;
        }

        public void SetDirty()
        {
            hash = null;
        }

        public int SizeAsChild
        {
            get
            {
                switch (type)
                {
                    case NodeType.BranchNode:
                    case NodeType.ExtensionNode:
                    case NodeType.LeafNode:
                        return NewHash(Hash).Size;
                    case NodeType.HashNode:
                    case NodeType.Empty:
                        return Size;
                    default:
                        throw new InvalidOperationException(nameof(Node));
                }
            }
        }

        public void SerializeAsChild(BinaryWriter writer)
        {
            switch (type)
            {
                case NodeType.BranchNode:
                case NodeType.ExtensionNode:
                case NodeType.LeafNode:
                    var n = NewHash(Hash);
                    n.Serialize(writer);
                    break;
                case NodeType.HashNode:
                case NodeType.Empty:
                    Serialize(writer);
                    break;
                default:
                    throw new FormatException(nameof(SerializeAsChild));
            }
        }

        private void SerializeWithoutReference(BinaryWriter writer)
        {
            writer.Write((byte)type);
            switch (type)
            {
                case NodeType.BranchNode:
                    SerializeBranch(writer);
                    break;
                case NodeType.ExtensionNode:
                    SerializeExtension(writer);
                    break;
                case NodeType.LeafNode:
                    SerializeLeaf(writer);
                    break;
                case NodeType.HashNode:
                    SerializeHash(writer);
                    break;
                case NodeType.Empty:
                    break;
                default:
                    throw new FormatException(nameof(SerializeWithoutReference));
            }
        }

        public void Serialize(BinaryWriter writer)
        {
            SerializeWithoutReference(writer);
            if (type == NodeType.BranchNode || type == NodeType.ExtensionNode || type == NodeType.LeafNode)
                writer.WriteVarInt(Reference);
        }

        public byte[] ToArrayWithoutReference()
        {
            using MemoryStream ms = new MemoryStream();
            using BinaryWriter writer = new BinaryWriter(ms, Utility.StrictUTF8, true);

            SerializeWithoutReference(writer);
            writer.Flush();
            return ms.ToArray();
        }

        public void Deserialize(ref MemoryReader reader)
        {
            type = (NodeType)reader.ReadByte();
            switch (type)
            {
                case NodeType.BranchNode:
                    DeserializeBranch(ref reader);
                    Reference = (int)reader.ReadVarInt();
                    break;
                case NodeType.ExtensionNode:
                    DeserializeExtension(ref reader);
                    Reference = (int)reader.ReadVarInt();
                    break;
                case NodeType.LeafNode:
                    DeserializeLeaf(ref reader);
                    Reference = (int)reader.ReadVarInt();
                    break;
                case NodeType.Empty:
                    break;
                case NodeType.HashNode:
                    DeserializeHash(ref reader);
                    break;
                default:
                    throw new FormatException(nameof(Deserialize));
            }
        }

        private Node CloneAsChild()
        {
            switch (type)
            {
                case NodeType.BranchNode:
                case NodeType.ExtensionNode:
                case NodeType.LeafNode:
                    return new Node
                    {
                        type = NodeType.HashNode,
                        hash = Hash,
                    };
                case NodeType.HashNode:
                case NodeType.Empty:
                    return Clone();
                default:
                    throw new InvalidOperationException(nameof(Clone));
            }
        }

        public Node Clone()
        {
            switch (type)
            {
                case NodeType.BranchNode:
                    var n = new Node
                    {
                        type = type,
                        Reference = Reference,
                        Children = new Node[BranchChildCount],
                    };
                    for (int i = 0; i < BranchChildCount; i++)
                    {
                        n.Children[i] = Children[i].CloneAsChild();
                    }
                    return n;
                case NodeType.ExtensionNode:
                    return new Node
                    {
                        type = type,
                        Key = Key,
                        Next = Next.CloneAsChild(),
                        Reference = Reference,
                    };
                case NodeType.LeafNode:
                    return new Node
                    {
                        type = type,
                        Value = Value,
                        Reference = Reference,
                    };
                case NodeType.HashNode:
                case NodeType.Empty:
                    return this;
                default:
                    throw new InvalidOperationException(nameof(Clone));
            }
        }

        public void FromReplica(Node n)
        {
            MemoryReader reader = new(n.ToArray());
            Deserialize(ref reader);
        }
    }
}
