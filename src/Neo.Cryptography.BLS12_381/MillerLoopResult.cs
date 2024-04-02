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
// MillerLoopResult.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using System.Runtime.InteropServices;
using static Neo.Cryptography.BLS12_381.Constants;

namespace Neo.Cryptography.BLS12_381;

[StructLayout(LayoutKind.Explicit, Size = Fp12.Size)]
readonly struct MillerLoopResult
{
    [FieldOffset(0)]
    private readonly Fp12 v;

    public MillerLoopResult(in Fp12 v)
    {
        this.v = v;
    }

    public Gt FinalExponentiation()
    {
        static (Fp2, Fp2) Fp4Square(in Fp2 a, in Fp2 b)
        {
            var t0 = a.Square();
            var t1 = b.Square();
            var t2 = t1.MulByNonresidue();
            var c0 = t2 + t0;
            t2 = a + b;
            t2 = t2.Square();
            t2 -= t0;
            var c1 = t2 - t1;

            return (c0, c1);
        }

        static Fp12 CyclotomicSquare(in Fp12 f)
        {
            var z0 = f.C0.C0;
            var z4 = f.C0.C1;
            var z3 = f.C0.C2;
            var z2 = f.C1.C0;
            var z1 = f.C1.C1;
            var z5 = f.C1.C2;

            var (t0, t1) = Fp4Square(in z0, in z1);

            // For A
            z0 = t0 - z0;
            z0 = z0 + z0 + t0;

            z1 = t1 + z1;
            z1 = z1 + z1 + t1;

            (t0, t1) = Fp4Square(in z2, in z3);
            var (t2, t3) = Fp4Square(in z4, in z5);

            // For C
            z4 = t0 - z4;
            z4 = z4 + z4 + t0;

            z5 = t1 + z5;
            z5 = z5 + z5 + t1;

            // For B
            t0 = t3.MulByNonresidue();
            z2 = t0 + z2;
            z2 = z2 + z2 + t0;

            z3 = t2 - z3;
            z3 = z3 + z3 + t2;

            return new Fp12(new Fp6(in z0, in z4, in z3), new Fp6(in z2, in z1, in z5));
        }

        static Fp12 CycolotomicExp(in Fp12 f)
        {
            var x = BLS_X;
            var tmp = Fp12.One;
            var found_one = false;
            foreach (bool i in Enumerable.Range(0, 64).Select(b => ((x >> b) & 1) == 1).Reverse())
            {
                if (found_one)
                    tmp = CyclotomicSquare(tmp);
                else
                    found_one = i;

                if (i)
                    tmp *= f;
            }

            return tmp.Conjugate();
        }

        var f = v;
        var t0 = f
            .FrobeniusMap()
            .FrobeniusMap()
            .FrobeniusMap()
            .FrobeniusMap()
            .FrobeniusMap()
            .FrobeniusMap();
        var t1 = f.Invert();
        var t2 = t0 * t1;
        t1 = t2;
        t2 = t2.FrobeniusMap().FrobeniusMap();
        t2 *= t1;
        t1 = CyclotomicSquare(t2).Conjugate();
        var t3 = CycolotomicExp(t2);
        var t4 = CyclotomicSquare(t3);
        var t5 = t1 * t3;
        t1 = CycolotomicExp(t5);
        t0 = CycolotomicExp(t1);
        var t6 = CycolotomicExp(t0);
        t6 *= t4;
        t4 = CycolotomicExp(t6);
        t5 = t5.Conjugate();
        t4 *= t5 * t2;
        t5 = t2.Conjugate();
        t1 *= t2;
        t1 = t1.FrobeniusMap().FrobeniusMap().FrobeniusMap();
        t6 *= t5;
        t6 = t6.FrobeniusMap();
        t3 *= t0;
        t3 = t3.FrobeniusMap().FrobeniusMap();
        t3 *= t1;
        t3 *= t6;
        f = t3 * t4;
        return new Gt(f);
    }

    public static MillerLoopResult operator +(in MillerLoopResult a, in MillerLoopResult b)
    {
        return new(a.v * b.v);
    }
}
