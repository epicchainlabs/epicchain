// Copyright (C) 2021-2024 EpicChain Labs.

//
// CryptoHive.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Cryptography.ECC;
using System;
using System.Collections.Generic;

namespace EpicChain.SmartContract.Native
{
    /// <summary>
    /// A native contract library that provides cryptographic algorithms.
    /// </summary>
    public sealed partial class CryptoHive : NativeContract
    {
        private static readonly Dictionary<NamedCurveHash, (ECCurve Curve, Hasher Hasher)> s_curves = new()
        {
            [NamedCurveHash.secp256k1SHA256] = (ECCurve.Secp256k1, Hasher.SHA256),
            [NamedCurveHash.secp256r1SHA256] = (ECCurve.Secp256r1, Hasher.SHA256),
            [NamedCurveHash.secp256k1Keccak256] = (ECCurve.Secp256k1, Hasher.Keccak256),
            [NamedCurveHash.secp256r1Keccak256] = (ECCurve.Secp256r1, Hasher.Keccak256),
        };

        internal CryptoHive() : base() { }

        /// <summary>
        /// Computes the hash value for the specified byte array using the ripemd160 algorithm.
        /// </summary>
        /// <param name="data">The input to compute the hash code for.</param>
        /// <returns>The computed hash code.</returns>
        [ContractMethod(CpuFee = 1 << 15, Name = "ripemd160")]
        public static byte[] RIPEMD160(byte[] data)
        {
            return data.RIPEMD160();
        }

        /// <summary>
        /// Computes the hash value for the specified byte array using the sha256 algorithm.
        /// </summary>
        /// <param name="data">The input to compute the hash code for.</param>
        /// <returns>The computed hash code.</returns>
        [ContractMethod(CpuFee = 1 << 15)]
        public static byte[] Sha256(byte[] data)
        {
            return data.Sha256();
        }

        /// <summary>
        /// Computes the hash value for the specified byte array using the murmur32 algorithm.
        /// </summary>
        /// <param name="data">The input to compute the hash code for.</param>
        /// <param name="seed">The seed of the murmur32 hash function</param>
        /// <returns>The computed hash code.</returns>
        [ContractMethod(CpuFee = 1 << 13)]
        public static byte[] Murmur32(byte[] data, uint seed)
        {
            using Murmur32 murmur = new(seed);
            return murmur.ComputeHash(data);
        }

        /// <summary>
        /// Computes the hash value for the specified byte array using the keccak256 algorithm.
        /// </summary>
        /// <param name="data">The input to compute the hash code for.</param>
        /// <returns>Computed hash</returns>
        [ContractMethod(Hardfork.HF_Cockatrice, CpuFee = 1 << 15)]
        public static byte[] Keccak256(byte[] data)
        {
            return data.Keccak256();
        }

        /// <summary>
        /// Verifies that a digital signature is appropriate for the provided key and message using the ECDSA algorithm.
        /// </summary>
        /// <param name="message">The signed message.</param>
        /// <param name="pubkey">The public key to be used.</param>
        /// <param name="signature">The signature to be verified.</param>
        /// <param name="curveHash">A pair of the curve to be used by the ECDSA algorithm and the hasher function to be used to hash message.</param>
        /// <returns><see langword="true"/> if the signature is valid; otherwise, <see langword="false"/>.</returns>
        [ContractMethod(Hardfork.HF_Cockatrice, CpuFee = 1 << 15)]
        public static bool VerifyWithECDsa(byte[] message, byte[] pubkey, byte[] signature, NamedCurveHash curveHash)
        {
            try
            {
                var ch = s_curves[curveHash];
                return Crypto.VerifySignature(message, signature, pubkey, ch.Curve, ch.Hasher);
            }
            catch (ArgumentException)
            {
                return false;
            }
        }

        // This is for solving the hardfork issue in https://github.com/epicchainlabs/epicchain/pull/3209
        [ContractMethod(true, Hardfork.HF_Cockatrice, CpuFee = 1 << 15, Name = "verifyWithECDsa")]
        public static bool VerifyWithECDsaV0(byte[] message, byte[] pubkey, byte[] signature, NamedCurveHash curve)
        {
            if (curve != NamedCurveHash.secp256k1SHA256 && curve != NamedCurveHash.secp256r1SHA256)
                throw new ArgumentOutOfRangeException(nameof(curve));

            try
            {
                return Crypto.VerifySignature(message, signature, pubkey, s_curves[curve].Curve);
            }
            catch (ArgumentException)
            {
                return false;
            }
        }
    }
}
