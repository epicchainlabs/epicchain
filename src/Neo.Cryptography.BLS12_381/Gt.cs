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
// Gt.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using System.Diagnostics.CodeAnalysis;
using System.Runtime.CompilerServices;
using System.Security.Cryptography;
using static Neo.Cryptography.BLS12_381.ConstantTimeUtility;
using static Neo.Cryptography.BLS12_381.GtConstants;

namespace Neo.Cryptography.BLS12_381;

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
