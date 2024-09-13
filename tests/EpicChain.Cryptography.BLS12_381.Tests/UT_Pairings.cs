// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_Pairings.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


namespace EpicChain.Cryptography.BLS12_381.Tests;

[TestClass]
public class UT_Pairings
{
    [TestMethod]
    public void TestGtGenerator()
    {
        Assert.AreEqual(
            Gt.Generator,
            Bls12.Pairing(in G1Affine.Generator, in G2Affine.Generator)
        );
    }

    [TestMethod]
    public void TestBilinearity()
    {
        var a = Scalar.FromRaw(new ulong[] { 1, 2, 3, 4 }).Invert().Square();
        var b = Scalar.FromRaw(new ulong[] { 5, 6, 7, 8 }).Invert().Square();
        var c = a * b;

        var g = new G1Affine(G1Affine.Generator * a);
        var h = new G2Affine(G2Affine.Generator * b);
        var p = Bls12.Pairing(in g, in h);

        Assert.AreNotEqual(Gt.Identity, p);

        var expected = new G1Affine(G1Affine.Generator * c);

        Assert.AreEqual(p, Bls12.Pairing(in expected, in G2Affine.Generator));
        Assert.AreEqual(
            p,
            Bls12.Pairing(in G1Affine.Generator, in G2Affine.Generator) * c
        );
    }

    [TestMethod]
    public void TestUnitary()
    {
        var g = G1Affine.Generator;
        var h = G2Affine.Generator;
        var p = -Bls12.Pairing(in g, in h);
        var q = Bls12.Pairing(in g, -h);
        var r = Bls12.Pairing(-g, in h);

        Assert.AreEqual(p, q);
        Assert.AreEqual(q, r);
    }

    [TestMethod]
    public void TestMillerLoopResultDefault()
    {
        Assert.AreEqual(
            Gt.Identity,
            new MillerLoopResult(Fp12.One).FinalExponentiation()
        );
    }
}
