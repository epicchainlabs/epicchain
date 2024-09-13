// Copyright (C) 2021-2024 EpicChain Labs.

//
// NefFile.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.VM;
using System;
using System.Buffers.Binary;
using System.IO;
using System.Linq;

namespace EpicChain.SmartContract
{
    /*
    ┌───────────────────────────────────────────────────────────────────────┐
    │                    EpicChain Executable Format 3 (NEF3)                     │
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
    /// Represents the structure of EpicChain Executable Format.
    /// </summary>
    public class NefFile : ISerializable
    {
        /// <summary>
        /// EpicChain Executable Format 3 (NEF3)
        /// </summary>
        private const uint Magic = 0x3346454E;

        /// <summary>
        /// The name and version of the compiler that generated this xef file.
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
        /// The checksum of the xef file.
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
        /// Computes the checksum for the specified xef file.
        /// </summary>
        /// <param name="file">The specified xef file.</param>
        /// <returns>The checksum of the xef file.</returns>
        public static uint ComputeChecksum(NefFile file)
        {
            return BinaryPrimitives.ReadUInt32LittleEndian(Crypto.Hash256(file.ToArray().AsSpan(..^sizeof(uint))));
        }

        /// <summary>
        /// Converts the xef file to a JSON object.
        /// </summary>
        /// <returns>The xef file represented by a JSON object.</returns>
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
