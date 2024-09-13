// Copyright (C) 2021-2024 EpicChain Labs.

//
// KeyBuilder.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Buffers.Binary;
using System.IO;

namespace EpicChain.SmartContract
{
    /// <summary>
    /// Used to build storage keys for native contracts.
    /// </summary>
    public class KeyBuilder
    {
        private readonly MemoryStream stream = new();

        /// <summary>
        /// Initializes a new instance of the <see cref="KeyBuilder"/> class.
        /// </summary>
        /// <param name="id">The id of the contract.</param>
        /// <param name="prefix">The prefix of the key.</param>
        public KeyBuilder(int id, byte prefix)
        {
            var data = new byte[sizeof(int)];
            BinaryPrimitives.WriteInt32LittleEndian(data, id);

            stream.Write(data);
            stream.WriteByte(prefix);
        }

        /// <summary>
        /// Adds part of the key to the builder.
        /// </summary>
        /// <param name="key">Part of the key.</param>
        /// <returns>A reference to this instance after the add operation has completed.</returns>
        public KeyBuilder Add(byte key)
        {
            stream.WriteByte(key);
            return this;
        }

        /// <summary>
        /// Adds part of the key to the builder.
        /// </summary>
        /// <param name="key">Part of the key.</param>
        /// <returns>A reference to this instance after the add operation has completed.</returns>
        public KeyBuilder Add(ReadOnlySpan<byte> key)
        {
            stream.Write(key);
            return this;
        }

        /// <summary>
        /// Adds part of the key to the builder.
        /// </summary>
        /// <param name="key">Part of the key.</param>
        /// <returns>A reference to this instance after the add operation has completed.</returns>
        public KeyBuilder Add(ISerializable key)
        {
            using (BinaryWriter writer = new(stream, Utility.StrictUTF8, true))
            {
                key.Serialize(writer);
                writer.Flush();
            }
            return this;
        }

        /// <summary>
        /// Adds part of the key to the builder in BigEndian.
        /// </summary>
        /// <param name="key">Part of the key.</param>
        /// <returns>A reference to this instance after the add operation has completed.</returns>
        public KeyBuilder AddBigEndian(int key)
        {
            var data = new byte[sizeof(int)];
            BinaryPrimitives.WriteInt32BigEndian(data, key);

            return Add(data);
        }

        /// <summary>
        /// Adds part of the key to the builder in BigEndian.
        /// </summary>
        /// <param name="key">Part of the key.</param>
        /// <returns>A reference to this instance after the add operation has completed.</returns>
        public KeyBuilder AddBigEndian(uint key)
        {
            var data = new byte[sizeof(uint)];
            BinaryPrimitives.WriteUInt32BigEndian(data, key);

            return Add(data);
        }

        /// <summary>
        /// Adds part of the key to the builder in BigEndian.
        /// </summary>
        /// <param name="key">Part of the key.</param>
        /// <returns>A reference to this instance after the add operation has completed.</returns>
        public KeyBuilder AddBigEndian(long key)
        {
            var data = new byte[sizeof(long)];
            BinaryPrimitives.WriteInt64BigEndian(data, key);

            return Add(data);
        }

        /// <summary>
        /// Adds part of the key to the builder in BigEndian.
        /// </summary>
        /// <param name="key">Part of the key.</param>
        /// <returns>A reference to this instance after the add operation has completed.</returns>
        public KeyBuilder AddBigEndian(ulong key)
        {
            var data = new byte[sizeof(ulong)];
            BinaryPrimitives.WriteUInt64BigEndian(data, key);

            return Add(data);
        }

        /// <summary>
        /// Gets the storage key generated by the builder.
        /// </summary>
        /// <returns>The storage key.</returns>
        public byte[] ToArray()
        {
            using (stream)
            {
                return stream.ToArray();
            }
        }

        public static implicit operator StorageKey(KeyBuilder builder)
        {
            using (builder.stream)
            {
                return new StorageKey(builder.stream.ToArray());
            }
        }
    }
}
