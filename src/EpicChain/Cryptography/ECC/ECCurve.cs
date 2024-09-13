// Copyright (C) 2021-2024 EpicChain Labs.

//
// ECCurve.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Extensions;
using System.Globalization;
using System.Numerics;

namespace EpicChain.Cryptography.ECC
{
    /// <summary>
    /// Represents an elliptic curve.
    /// </summary>
    public class ECCurve
    {
        internal readonly BigInteger Q;
        internal readonly ECFieldElement A;
        internal readonly ECFieldElement B;
        public readonly BigInteger N;
        /// <summary>
        /// The point at infinity.
        /// </summary>
        public readonly ECPoint Infinity;
        /// <summary>
        /// The generator, or base point, for operations on the curve.
        /// </summary>
        public readonly ECPoint G;

        internal readonly int ExpectedECPointLength;

        private ECCurve(BigInteger Q, BigInteger A, BigInteger B, BigInteger N, byte[] G)
        {
            this.Q = Q;
            ExpectedECPointLength = ((int)VM.Utility.GetBitLength(Q) + 7) / 8;
            this.A = new ECFieldElement(A, this);
            this.B = new ECFieldElement(B, this);
            this.N = N;
            Infinity = new ECPoint(null, null, this);
            this.G = ECPoint.DecodePoint(G, this);
        }

        /// <summary>
        /// Represents a secp256k1 named curve.
        /// </summary>
        public static readonly ECCurve Secp256k1 = new
        (
            BigInteger.Parse("00FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", NumberStyles.AllowHexSpecifier),
            BigInteger.Zero,
            7,
            BigInteger.Parse("00FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", NumberStyles.AllowHexSpecifier),
            ("04" + "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798" + "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8").HexToBytes()
        );

        /// <summary>
        /// Represents a secp256r1 named curve.
        /// </summary>
        public static readonly ECCurve Secp256r1 = new
        (
            BigInteger.Parse("00FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF", NumberStyles.AllowHexSpecifier),
            BigInteger.Parse("00FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFC", NumberStyles.AllowHexSpecifier),
            BigInteger.Parse("005AC635D8AA3A93E7B3EBBD55769886BC651D06B0CC53B0F63BCE3C3E27D2604B", NumberStyles.AllowHexSpecifier),
            BigInteger.Parse("00FFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551", NumberStyles.AllowHexSpecifier),
            ("04" + "6B17D1F2E12C4247F8BCE6E563A440F277037D812DEB33A0F4A13945D898C296" + "4FE342E2FE1A7F9B8EE7EB4A7C0F9E162BCE33576B315ECECBB6406837BF51F5").HexToBytes()
        );
    }
}
