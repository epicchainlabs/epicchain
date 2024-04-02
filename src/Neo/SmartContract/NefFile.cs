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
// NefFile.cs file belongs to the neo project and is free
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

        public void Deserialize(ref MemoryReader reader)
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
            if (CheckSum != ComputeChecksum(this)) throw new FormatException("CRC verification fail");
            if (reader.Position - startPosition > ExecutionEngineLimits.Default.MaxItemSize) throw new FormatException("Max vm item size exceed");
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
