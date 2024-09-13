// Copyright (C) 2021-2024 EpicChain Labs.

//
// G2Prepared.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using static EpicChain.Cryptography.BLS12_381.ConstantTimeUtility;
using static EpicChain.Cryptography.BLS12_381.MillerLoopUtility;

namespace EpicChain.Cryptography.BLS12_381;

partial class G2Prepared
{
    public readonly bool Infinity;
    public readonly List<(Fp2, Fp2, Fp2)> Coeffs;

    public G2Prepared(in G2Affine q)
    {
        Infinity = q.IsIdentity;
        var q2 = ConditionalSelect(in q, in G2Affine.Generator, Infinity);
        var adder = new Adder(q2);
        MillerLoop<object?, Adder>(adder);
        Coeffs = adder.Coeffs;
        if (Coeffs.Count != 68) throw new InvalidOperationException();
    }
}
