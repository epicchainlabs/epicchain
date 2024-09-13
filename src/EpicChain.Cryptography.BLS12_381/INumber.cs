// Copyright (C) 2021-2024 EpicChain Labs.

//
// INumber.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

namespace EpicChain.Cryptography.BLS12_381;

interface INumber<T> where T : unmanaged, INumber<T>
{
    //static abstract int Size { get; }
    //static abstract ref readonly T Zero { get; }
    //static abstract ref readonly T One { get; }

    //static abstract T operator -(in T x);
    //static abstract T operator +(in T x, in T y);
    //static abstract T operator -(in T x, in T y);
    //static abstract T operator *(in T x, in T y);

    T Negate();
    T Sum(in T value);
    T Subtract(in T value);
    T Multiply(in T value);

    abstract T Square();
}

static class NumberExtensions
{
    private static T PowVartime<T>(T one, T self, ulong[] by) where T : unmanaged, INumber<T>
    {
        // Although this is labeled "vartime", it is only
        // variable time with respect to the exponent.
        var res = one;
        for (int j = by.Length - 1; j >= 0; j--)
        {
            for (int i = 63; i >= 0; i--)
            {
                res = res.Square();
                if (((by[j] >> i) & 1) == 1)
                {
                    res = res.Multiply(self);
                }
            }
        }
        return res;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Fp PowVartime(this Fp self, ulong[] by) => PowVartime(Fp.One, self, by);

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Fp2 PowVartime(this Fp2 self, ulong[] by) => PowVartime(Fp2.One, self, by);

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Fp6 PowVartime(this Fp6 self, ulong[] by) => PowVartime(Fp6.One, self, by);

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Fp12 PowVartime(this Fp12 self, ulong[] by) => PowVartime(Fp12.One, self, by);

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Scalar PowVartime(this Scalar self, ulong[] by) => PowVartime(Scalar.One, self, by);
}
