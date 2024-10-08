// Copyright (C) 2021-2024 EpicChain Labs.

//
// Gt.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System.Diagnostics.CodeAnalysis;
using System.Runtime.CompilerServices;
using System.Security.Cryptography;
using static EpicChain.Cryptography.BLS12_381.ConstantTimeUtility;
using static EpicChain.Cryptography.BLS12_381.GtConstants;

namespace EpicChain.Cryptography.BLS12_381;

public readonly struct Gt : IEquatable<Gt>
{
    public readonly Fp12 Value;

    public static readonly Gt Identity = new(in Fp12.One);
    public static readonly Gt Generator = new(in GeneratorValue);

    public bool IsIdentity => this == Identity;

    public Gt(in Fp12 f)
    {
        Value = f;
    }

    public static Gt FromBytes(ReadOnlySpan<byte> data)
    {
        return new(Fp12.FromBytes(data));
    }

    public static bool operator ==(in Gt a, in Gt b)
    {
        return a.Value == b.Value;
    }

    public static bool operator !=(in Gt a, in Gt b)
    {
        return !(a == b);
    }

    public override bool Equals([NotNullWhen(true)] object? obj)
    {
        if (obj is not Gt other) return false;
        return this == other;
    }

    public bool Equals(Gt other)
    {
        return this == other;
    }

    public override int GetHashCode()
    {
        return Value.GetHashCode();
    }

    public byte[] ToArray()
    {
        return Value.ToArray();
    }

    public bool TryWrite(Span<byte> buffer)
    {
        return Value.TryWrite(buffer);
    }

    public static Gt Random(RandomNumberGenerator rng)
    {
        while (true)
        {
            var inner = Fp12.Random(rng);

            // Not all elements of Fp12 are elements of the prime-order multiplicative
            // subgroup. We run the random element through final_exponentiation to obtain
            // a valid element, which requires that it is non-zero.
            if (!inner.IsZero)
            {
                ref MillerLoopResult result = ref Unsafe.As<Fp12, MillerLoopResult>(ref inner);
                return result.FinalExponentiation();
            }
        }
    }

    public Gt Double()
    {
        return new(Value.Square());
    }

    public static Gt operator -(in Gt a)
    {
        // The element is unitary, so we just conjugate.
        return new(a.Value.Conjugate());
    }

    public static Gt operator +(in Gt a, in Gt b)
    {
        return new(a.Value * b.Value);
    }

    public static Gt operator -(in Gt a, in Gt b)
    {
        return a + -b;
    }

    public static Gt operator *(in Gt a, in Scalar b)
    {
        var acc = Identity;

        // This is a simple double-and-add implementation of group element
        // multiplication, moving from most significant to least
        // significant bit of the scalar.
        //
        // We skip the leading bit because it's always unset for Fq
        // elements.
        foreach (bool bit in b
            .ToArray()
            .SelectMany(p => Enumerable.Range(0, 8).Select(q => ((p >> q) & 1) == 1))
            .Reverse()
            .Skip(1))
        {
            acc = acc.Double();
            acc = ConditionalSelect(in acc, acc + a, bit);
        }

        return acc;
    }
}
