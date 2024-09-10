// Copyright (C) 2021-2024 EpicChain Labs.

//
// Bls12.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using static Neo.Cryptography.BLS12_381.ConstantTimeUtility;
using static Neo.Cryptography.BLS12_381.MillerLoopUtility;

namespace Neo.Cryptography.BLS12_381;

public static partial class Bls12
{
    public static Gt Pairing(in G1Affine p, in G2Affine q)
    {
        var either_identity = p.IsIdentity | q.IsIdentity;
        var p2 = ConditionalSelect(in p, in G1Affine.Generator, either_identity);
        var q2 = ConditionalSelect(in q, in G2Affine.Generator, either_identity);

        var adder = new Adder(p2, q2);

        var tmp = MillerLoop<Fp12, Adder>(adder);
        var tmp2 = new MillerLoopResult(ConditionalSelect(in tmp, in Fp12.One, either_identity));
        return tmp2.FinalExponentiation();
    }
}
