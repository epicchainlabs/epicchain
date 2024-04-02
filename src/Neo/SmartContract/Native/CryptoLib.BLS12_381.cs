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
// CryptoLib.BLS12_381.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.Cryptography.BLS12_381;
using Neo.VM.Types;
using System;

namespace Neo.SmartContract.Native;

partial class CryptoLib
{
    /// <summary>
    /// Serialize a bls12381 point.
    /// </summary>
    /// <param name="g">The point to be serialized.</param>
    /// <returns></returns>
    [ContractMethod(CpuFee = 1 << 19)]
    public static byte[] Bls12381Serialize(InteropInterface g)
    {
        return g.GetInterface<object>() switch
        {
            G1Affine p => p.ToCompressed(),
            G1Projective p => new G1Affine(p).ToCompressed(),
            G2Affine p => p.ToCompressed(),
            G2Projective p => new G2Affine(p).ToCompressed(),
            Gt p => p.ToArray(),
            _ => throw new ArgumentException($"Bls12381 operation fault, type:format, error:type mismatch")
        };
    }

    /// <summary>
    /// Deserialize a bls12381 point.
    /// </summary>
    /// <param name="data">The point as byte array.</param>
    /// <returns></returns>
    [ContractMethod(CpuFee = 1 << 19)]
    public static InteropInterface Bls12381Deserialize(byte[] data)
    {
        return data.Length switch
        {
            48 => new InteropInterface(G1Affine.FromCompressed(data)),
            96 => new InteropInterface(G2Affine.FromCompressed(data)),
            576 => new InteropInterface(Gt.FromBytes(data)),
            _ => throw new ArgumentException($"Bls12381 operation fault, type:format, error:valid point length"),
        };
    }

    /// <summary>
    /// Determines whether the specified points are equal.
    /// </summary>
    /// <param name="x">The first point.</param>
    /// <param name="y">Teh second point.</param>
    /// <returns><c>true</c> if the specified points are equal; otherwise, <c>false</c>.</returns>
    [ContractMethod(CpuFee = 1 << 5)]
    public static bool Bls12381Equal(InteropInterface x, InteropInterface y)
    {
        return (x.GetInterface<object>(), y.GetInterface<object>()) switch
        {
            (G1Affine p1, G1Affine p2) => p1.Equals(p2),
            (G1Projective p1, G1Projective p2) => p1.Equals(p2),
            (G2Affine p1, G2Affine p2) => p1.Equals(p2),
            (G2Projective p1, G2Projective p2) => p1.Equals(p2),
            (Gt p1, Gt p2) => p1.Equals(p2),
            _ => throw new ArgumentException($"Bls12381 operation fault, type:format, error:type mismatch")
        };
    }

    /// <summary>
    /// Add operation of two points.
    /// </summary>
    /// <param name="x">The first point.</param>
    /// <param name="y">The second point.</param>
    /// <returns></returns>
    [ContractMethod(CpuFee = 1 << 19)]
    public static InteropInterface Bls12381Add(InteropInterface x, InteropInterface y)
    {
        return (x.GetInterface<object>(), y.GetInterface<object>()) switch
        {
            (G1Affine p1, G1Affine p2) => new(new G1Projective(p1) + p2),
            (G1Affine p1, G1Projective p2) => new(p1 + p2),
            (G1Projective p1, G1Affine p2) => new(p1 + p2),
            (G1Projective p1, G1Projective p2) => new(p1 + p2),
            (G2Affine p1, G2Affine p2) => new(new G2Projective(p1) + p2),
            (G2Affine p1, G2Projective p2) => new(p1 + p2),
            (G2Projective p1, G2Affine p2) => new(p1 + p2),
            (G2Projective p1, G2Projective p2) => new(p1 + p2),
            (Gt p1, Gt p2) => new(p1 + p2),
            _ => throw new ArgumentException($"Bls12381 operation fault, type:format, error:type mismatch")
        };
    }

    /// <summary>
    /// Mul operation of gt point and multiplier
    /// </summary>
    /// <param name="x">The point</param>
    /// <param name="mul">Multiplier,32 bytes,little-endian</param>
    /// <param name="neg">negative number</param>
    /// <returns></returns>
    [ContractMethod(CpuFee = 1 << 21)]
    public static InteropInterface Bls12381Mul(InteropInterface x, byte[] mul, bool neg)
    {
        Scalar X = neg ? -Scalar.FromBytes(mul) : Scalar.FromBytes(mul);
        return x.GetInterface<object>() switch
        {
            G1Affine p => new(p * X),
            G1Projective p => new(p * X),
            G2Affine p => new(p * X),
            G2Projective p => new(p * X),
            Gt p => new(p * X),
            _ => throw new ArgumentException($"Bls12381 operation fault, type:format, error:type mismatch")
        };
    }

    /// <summary>
    /// Pairing operation of g1 and g2
    /// </summary>
    /// <param name="g1">The g1 point.</param>
    /// <param name="g2">The g2 point.</param>
    /// <returns></returns>
    [ContractMethod(CpuFee = 1 << 23)]
    public static InteropInterface Bls12381Pairing(InteropInterface g1, InteropInterface g2)
    {
        G1Affine g1a = g1.GetInterface<object>() switch
        {
            G1Affine g => g,
            G1Projective g => new(g),
            _ => throw new ArgumentException($"Bls12381 operation fault, type:format, error:type mismatch")
        };
        G2Affine g2a = g2.GetInterface<object>() switch
        {
            G2Affine g => g,
            G2Projective g => new(g),
            _ => throw new ArgumentException($"Bls12381 operation fault, type:format, error:type mismatch")
        };
        return new(Bls12.Pairing(in g1a, in g2a));
    }
}
