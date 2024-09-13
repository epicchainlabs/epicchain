// Copyright (C) 2021-2024 EpicChain Labs.

//
// Bls12.Adder.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System.Runtime.CompilerServices;
using static EpicChain.Cryptography.BLS12_381.MillerLoopUtility;

namespace EpicChain.Cryptography.BLS12_381;

partial class Bls12
{
    class Adder : IMillerLoopDriver<Fp12>
    {
        public G2Projective Curve;
        public readonly G2Affine Base;
        public readonly G1Affine P;

        public Adder(in G1Affine p, in G2Affine q)
        {
            Curve = new(q);
            Base = q;
            P = p;
        }

        Fp12 IMillerLoopDriver<Fp12>.DoublingStep(in Fp12 f)
        {
            var coeffs = DoublingStep(ref Curve);
            return Ell(in f, in coeffs, in P);
        }

        Fp12 IMillerLoopDriver<Fp12>.AdditionStep(in Fp12 f)
        {
            var coeffs = AdditionStep(ref Curve, in Base);
            return Ell(in f, in coeffs, in P);
        }

        #region IMillerLoopDriver<T>

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static Fp12 Square(in Fp12 f) => f.Square();

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static Fp12 Conjugate(in Fp12 f) => f.Conjugate();

        public static Fp12 One
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get => Fp12.One;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        Fp12 IMillerLoopDriver<Fp12>.Square(in Fp12 f) => Adder.Square(f);

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        Fp12 IMillerLoopDriver<Fp12>.Conjugate(in Fp12 f) => Adder.Conjugate(f);
        Fp12 IMillerLoopDriver<Fp12>.One
        {
            [MethodImpl(MethodImplOptions.AggressiveInlining)]
            get => Adder.One;
        }

        #endregion
    }
}
