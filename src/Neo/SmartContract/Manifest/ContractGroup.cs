// Copyright (C) 2021-2024 EpicChain Labs.

//
// ContractGroup.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Neo.Cryptography;
using Neo.Cryptography.ECC;
using Neo.IO;
using Neo.Json;
using Neo.VM;
using Neo.VM.Types;
using System;

namespace Neo.SmartContract.Manifest
{
    /// <summary>
    /// Represents a set of mutually trusted contracts.
    /// A contract will trust and allow any contract in the same group to invoke it, and the user interface will not give any warnings.
    /// A group is identified by a public key and must be accompanied by a signature for the contract hash to prove that the contract is indeed included in the group.
    /// </summary>
    public class ContractGroup : IInteroperable
    {
        /// <summary>
        /// The public key of the group.
        /// </summary>
        public ECPoint PubKey { get; set; }

        /// <summary>
        /// The signature of the contract hash which can be verified by <see cref="PubKey"/>.
        /// </summary>
        public byte[] Signature { get; set; }

        void IInteroperable.FromStackItem(StackItem stackItem)
        {
            Struct @struct = (Struct)stackItem;
            PubKey = ECPoint.DecodePoint(@struct[0].GetSpan(), ECCurve.Secp256r1);
            Signature = @struct[1].GetSpan().ToArray();
        }

        public StackItem ToStackItem(ReferenceCounter referenceCounter)
        {
            return new Struct(referenceCounter) { PubKey.ToArray(), Signature };
        }

        /// <summary>
        /// Converts the group from a JSON object.
        /// </summary>
        /// <param name="json">The group represented by a JSON object.</param>
        /// <returns>The converted group.</returns>
        public static ContractGroup FromJson(JObject json)
        {
            ContractGroup group = new()
            {
                PubKey = ECPoint.Parse(json["pubkey"].GetString(), ECCurve.Secp256r1),
                Signature = Convert.FromBase64String(json["signature"].GetString()),
            };
            if (group.Signature.Length != 64) throw new FormatException();
            return group;
        }

        /// <summary>
        /// Determines whether the signature in the group is valid.
        /// </summary>
        /// <param name="hash">The hash of the contract.</param>
        /// <returns><see langword="true"/> if the signature is valid; otherwise, <see langword="false"/>.</returns>
        public bool IsValid(UInt160 hash)
        {
            return Crypto.VerifySignature(hash.ToArray(), Signature, PubKey);
        }

        /// <summary>
        /// Converts the group to a JSON object.
        /// </summary>
        /// <returns>The group represented by a JSON object.</returns>
        public JObject ToJson()
        {
            var json = new JObject();
            json["pubkey"] = PubKey.ToString();
            json["signature"] = Convert.ToBase64String(Signature);
            return json;
        }
    }
}
