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
// G1Affine.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using System.Diagnostics.CodeAnalysis;
using static Neo.Cryptography.BLS12_381.ConstantTimeUtility;
using static Neo.Cryptography.BLS12_381.G1Constants;

namespace Neo.Cryptography.BLS12_381;

public readonly struct G1Affine : IEquatable<G1Affine>
{
    public readonly Fp X;
    public readonly Fp Y;
    public readonly bool Infinity;

    public static readonly G1Affine Identity = new(in Fp.Zero, in Fp.One, true);
    public static readonly G1Affine Generator = new(in GeneratorX, in GeneratorY, false);

    public bool IsIdentity => Infinity;
    public bool IsTorsionFree => -new G1Projective(this).MulByX().MulByX() == new G1Projective(Endomorphism());
    public bool IsOnCurve => ((Y.Square() - (X.Square() * X)) == B) | Infinity;

    public G1Affine(in Fp x, in Fp y)
        : this(in x, in y, false)
    {
    }

    private G1Affine(in Fp x, in Fp y, bool infinity)
    {
        X = x;
        Y = y;
        Infinity = infinity;
    }

    public G1Affine(in G1Projective p)
    {
        bool s = p.Z.TryInvert(out Fp zinv);

        zinv = ConditionalSelect(in Fp.Zero, in zinv, s);
        Fp x = p.X * zinv;
        Fp y = p.Y * zinv;

        G1Affine tmp = new(in x, in y, false);
        this = ConditionalSelect(in tmp, in Identity, !s);
    }

    public static G1Affine FromUncompressed(ReadOnlySpan<byte> data)
    {
        return FromBytes(data, false, true);
    }

    public static G1Affine FromCompressed(ReadOnlySpan<byte> data)
    {
        return FromBytes(data, true, true);
    }

    private static G1Affine FromBytes(ReadOnlySpan<byte> data, bool compressed, bool check)
    {
        bool compression_flag_set = (data[0] & 0x80) != 0;
        bool infinity_flag_set = (data[0] & 0x40) != 0;
        bool sort_flag_set = (data[0] & 0x20) != 0;
        byte[] tmp = data[0..48].ToArray();
        tmp[0] &= 0b0001_1111;
        Fp x = Fp.FromBytes(tmp);
        if (compressed)
        {
            Fp y = ((x.Square() * x) + B).Sqrt();
            y = ConditionalSelect(in y, -y, y.LexicographicallyLargest() ^ sort_flag_set);
            G1Affine result = new(in x, in y, infinity_flag_set);
            result = ConditionalSelect(in result, in Identity, infinity_flag_set);
            if (check)
            {
                bool _checked = (!infinity_flag_set | (infinity_flag_set & !sort_flag_set & x.IsZero))
                    & compression_flag_set;
                _checked &= result.IsTorsionFree;
                if (!_checked) throw new FormatException();
            }
            return result;
        }
        else
        {
            Fp y = Fp.FromBytes(data[48..96]);
            G1Affine result = ConditionalSelect(new(in x, in y, infinity_flag_set), in Identity, infinity_flag_set);
            if (check)
            {
                bool _checked = (!infinity_flag_set | (infinity_flag_set & x.IsZero & y.IsZero))
                    & !compression_flag_set
                    & !sort_flag_set;
                _checked &= result.IsOnCurve & result.IsTorsionFree;
                if (!_checked) throw new FormatException();
            }
            return result;
        }
    }

    public static bool operator ==(in G1Affine a, in G1Affine b)
    {
        return (a.Infinity & b.Infinity) | (!a.Infinity & !b.Infinity & a.X == b.X & a.Y == b.Y);
    }

    public static bool operator !=(in G1Affine a, in G1Affine b)
    {
        return !(a == b);
    }

    public override bool Equals([NotNullWhen(true)] object? obj)
    {
        if (obj is not G1Affine other) return false;
        return this == other;
    }

    public bool Equals(G1Affine other)
    {
        return this == other;
    }

    public override int GetHashCode()
    {
        if (Infinity) return Infinity.GetHashCode();
        return X.GetHashCode() ^ Y.GetHashCode();
    }

    public static G1Affine operator -(in G1Affine p)
    {
        return new G1Affine(in p.X, ConditionalSelect(-p.Y, in Fp.One, p.Infinity), p.Infinity);
    }

    public byte[] ToCompressed()
    {
        byte[] res = ConditionalSelect(in X, in Fp.Zero, Infinity).ToArray();

        // This point is in compressed form, so we set the most significant bit.
        res[0] |= 0x80;

        // Is this point at infinity? If so, set the second-most significant bit.
        res[0] |= ConditionalSelect((byte)0, (byte)0x40, Infinity);

        // Is the y-coordinate the lexicographically largest of the two associated with the
        // x-coordinate? If so, set the third-most significant bit so long as this is not
        // the point at infinity.
        res[0] |= ConditionalSelect((byte)0, (byte)0x20, !Infinity & Y.LexicographicallyLargest());

        return res;
    }

    public byte[] ToUncompressed()
    {
        byte[] res = new byte[96];

        ConditionalSelect(in X, in Fp.Zero, Infinity).TryWrite(res.AsSpan(0..48));
        ConditionalSelect(in Y, in Fp.Zero, Infinity).TryWrite(res.AsSpan(48..96));

        // Is this point at infinity? If so, set the second-most significant bit.
        res[0] |= ConditionalSelect((byte)0, (byte)0x40, Infinity);

        return res;
    }

    public G1Projective ToCurve()
    {
        return new(this);
    }

    private G1Affine Endomorphism()
    {
        return new(X * BETA, in Y, Infinity);
    }

    public static G1Projective operator *(in G1Affine a, in Scalar b)
    {
        return new G1Projective(in a) * b.ToArray();
    }
}
