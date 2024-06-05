// Copyright (C) 2021-2024 The EpicChain Labs.
//
// NefFile.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.Cryptography;
using Neo.IO;
using Neo.Json;
using Neo.VM;
using System;
using System.Buffers.Binary;
using System.IO;
using System.Linq;

namespace Neo.SmartContract
{
    /*
    ┌───────────────────────────────────────────────────────────────────────┐
    │                    NEO Executable Format 3 (NEF3)                     │
    ├──────────┬───────────────┬────────────────────────────────────────────┤
    │  Field   │     Type      │                  Comment                   │
    ├──────────┼───────────────┼────────────────────────────────────────────┤
    │ Magic    │ uint32        │ Magic header                               │
    │ Compiler │ byte[64]      │ Compiler name and version                  │
    ├──────────┼───────────────┼────────────────────────────────────────────┤
    │ Source   │ byte[]        │ The url of the source files                │
    │ Reserve  │ byte          │ Reserved for future extensions. Must be 0. │
    │ Tokens   │ MethodToken[] │ Method tokens.                             │
    │ Reserve  │ byte[2]       │ Reserved for future extensions. Must be 0. │
    │ Script   │ byte[]        │ Var bytes for the payload                  │
    ├──────────┼───────────────┼────────────────────────────────────────────┤
    │ Checksum │ uint32        │ First four bytes of double SHA256 hash     │
    └──────────┴───────────────┴────────────────────────────────────────────┘
    */
    /// <summary>
    /// Represents the structure of NEO Executable Format.
    /// </summary>
    public class NefFile : ISerializable
    {
        /// <summary>
        /// NEO Executable Format 3 (NEF3)
        /// </summary>
        private const uint Magic = 0x3346454E;

        /// <summary>
        /// The name and version of the compiler that generated this nef file.
        /// </summary>
        public string Compiler { get; set; }

        /// <summary>
        /// The url of the source files.
        /// </summary>
        public string Source { get; set; }

        /// <summary>
        /// The methods that to be called statically.
        /// </summary>
        public MethodToken[] Tokens { get; set; }

        /// <summary>
        /// The script of the contract.
        /// </summary>
        public ReadOnlyMemory<byte> Script { get; set; }

        /// <summary>
        /// The checksum of the nef file.
        /// </summary>
        public uint CheckSum { get; set; }

        private const int HeaderSize =
            sizeof(uint) +  // Magic
            64;             // Compiler

        public int Size =>
            HeaderSize +            // Header
            Source.GetVarSize() +   // Source
            1 +                     // Reserve
            Tokens.GetVarSize() +   // Tokens
            2 +                     // Reserve
            Script.GetVarSize() +   // Script
            sizeof(uint);           // Checksum

        /// <summary>
        /// Parse NefFile from memory
        /// </summary>
        /// <param name="memory">Memory</param>
        /// <param name="verify">Do checksum and MaxItemSize checks</param>
        /// <returns>NefFile</returns>
        public static NefFile Parse(ReadOnlyMemory<byte> memory, bool verify = true)
        {
            var reader = new MemoryReader(memory);
            var nef = new NefFile();
            nef.Deserialize(ref reader, verify);
            return nef;
        }

        public void Serialize(BinaryWriter writer)
        {
            SerializeHeader(writer);
            writer.WriteVarString(Source);
            writer.Write((byte)0);
            writer.Write(Tokens);
            writer.Write((short)0);
            writer.WriteVarBytes(Script.Span);
            writer.Write(CheckSum);
        }

        private void SerializeHeader(BinaryWriter writer)
        {
            writer.Write(Magic);
            writer.WriteFixedString(Compiler, 64);
        }

        public void Deserialize(ref MemoryReader reader) => Deserialize(ref reader, true);

        public void Deserialize(ref MemoryReader reader, bool verify = true)
        {
            long startPosition = reader.Position;
            if (reader.ReadUInt32() != Magic) throw new FormatException("Wrong magic");
            Compiler = reader.ReadFixedString(64);
            Source = reader.ReadVarString(256);
            if (reader.ReadByte() != 0) throw new FormatException("Reserved bytes must be 0");
            Tokens = reader.ReadSerializableArray<MethodToken>(128);
            if (reader.ReadUInt16() != 0) throw new FormatException("Reserved bytes must be 0");
            Script = reader.ReadVarMemory((int)ExecutionEngineLimits.Default.MaxItemSize);
            if (Script.Length == 0) throw new ArgumentException($"Script can't be empty");
            CheckSum = reader.ReadUInt32();
            if (verify)
            {
                if (CheckSum != ComputeChecksum(this)) throw new FormatException("CRC verification fail");
                if (reader.Position - startPosition > ExecutionEngineLimits.Default.MaxItemSize) throw new FormatException("Max vm item size exceed");
            }
        }

        /// <summary>
        /// Computes the checksum for the specified nef file.
        /// </summary>
        /// <param name="file">The specified nef file.</param>
        /// <returns>The checksum of the nef file.</returns>
        public static uint ComputeChecksum(NefFile file)
        {
            return BinaryPrimitives.ReadUInt32LittleEndian(Crypto.Hash256(file.ToArray().AsSpan(..^sizeof(uint))));
        }

        /// <summary>
        /// Converts the nef file to a JSON object.
        /// </summary>
        /// <returns>The nef file represented by a JSON object.</returns>
        public JObject ToJson()
        {
            return new JObject
            {
                ["magic"] = Magic,
                ["compiler"] = Compiler,
                ["source"] = Source,
                ["tokens"] = new JArray(Tokens.Select(p => p.ToJson())),
                ["script"] = Convert.ToBase64String(Script.Span),
                ["checksum"] = CheckSum
            };
        }
    }
}
