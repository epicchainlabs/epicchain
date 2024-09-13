// Copyright (C) 2021-2024 EpicChain Labs.

//
// Witness.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Json;
using EpicChain.SmartContract;
using System;
using System.IO;

namespace EpicChain.Network.P2P.Payloads
{
    /// <summary>
    /// Represents a witness of an <see cref="IVerifiable"/> object.
    /// </summary>
    public class Witness : ISerializable
    {
        // This is designed to allow a MultiSig 21/11 (committee)
        // Invocation = 11 * (64 + 2) = 726
        private const int MaxInvocationScript = 1024;

        // Verification = m + (PUSH_PubKey * 21) + length + null + syscall = 1 + ((2 + 33) * 21) + 2 + 1 + 5 = 744
        private const int MaxVerificationScript = 1024;

        /// <summary>
        /// The invocation script of the witness. Used to pass arguments for <see cref="VerificationScript"/>.
        /// </summary>
        public ReadOnlyMemory<byte> InvocationScript;

        /// <summary>
        /// The verification script of the witness. It can be empty if the contract is deployed.
        /// </summary>
        public ReadOnlyMemory<byte> VerificationScript;

        private UInt160 _scriptHash;
        /// <summary>
        /// The hash of the <see cref="VerificationScript"/>.
        /// </summary>
        public UInt160 ScriptHash
        {
            get
            {
                if (_scriptHash == null)
                {
                    _scriptHash = VerificationScript.Span.ToScriptHash();
                }
                return _scriptHash;
            }
        }

        public int Size => InvocationScript.GetVarSize() + VerificationScript.GetVarSize();

        void ISerializable.Deserialize(ref MemoryReader reader)
        {
            InvocationScript = reader.ReadVarMemory(MaxInvocationScript);
            VerificationScript = reader.ReadVarMemory(MaxVerificationScript);
        }

        void ISerializable.Serialize(BinaryWriter writer)
        {
            writer.WriteVarBytes(InvocationScript.Span);
            writer.WriteVarBytes(VerificationScript.Span);
        }

        /// <summary>
        /// Converts the witness to a JSON object.
        /// </summary>
        /// <returns>The witness represented by a JSON object.</returns>
        public JObject ToJson()
        {
            JObject json = new();
            json["invocation"] = Convert.ToBase64String(InvocationScript.Span);
            json["verification"] = Convert.ToBase64String(VerificationScript.Span);
            return json;
        }
    }
}
