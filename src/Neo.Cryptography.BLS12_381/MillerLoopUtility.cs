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
// MillerLoopUtility.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using static Neo.Cryptography.BLS12_381.Constants;

namespace Neo.Cryptography.BLS12_381;

static class MillerLoopUtility
{
    public static T MillerLoop<T, D>(D driver) where D : IMillerLoopDriver<T>
    {
        var f = driver.One;

        var found_one = false;
        foreach (var i in Enumerable.Range(0, 64).Reverse().Select(b => ((BLS_X >> 1 >> b) & 1) == 1))
        {
            if (!found_one)
            {
                found_one = i;
                continue;
            }

            f = driver.DoublingStep(f);

            if (i)
                f = driver.AdditionStep(f);

            f = driver.Square(f);
        }

        f = driver.DoublingStep(f);

        if (BLS_X_IS_NEGATIVE)
            f = driver.Conjugate(f);

        return f;
    }

    public static Fp12 Ell(in Fp12 f, in (Fp2 X, Fp2 Y, Fp2 Z) coeffs, in G1Affine p)
    {
        var c0 = new Fp2(coeffs.X.C0 * p.Y, coeffs.X.C1 * p.Y);
        var c1 = new Fp2(coeffs.Y.C0 * p.X, coeffs.Y.C1 * p.X);
        return f.MulBy_014(in coeffs.Z, in c1, in c0);
    }

    public static (Fp2, Fp2, Fp2) DoublingStep(ref G2Projective r)
    {
        // Adaptation of Algorithm 26, https://eprint.iacr.org/2010/354.pdf
        var tmp0 = r.X.Square();
        var tmp1 = r.Y.Square();
        var tmp2 = tmp1.Square();
        var tmp3 = (tmp1 + r.X).Square() - tmp0 - tmp2;
        tmp3 += tmp3;
        var tmp4 = tmp0 + tmp0 + tmp0;
        var tmp6 = r.X + tmp4;
        var tmp5 = tmp4.Square();
        var zsquared = r.Z.Square();
        var x = tmp5 - tmp3 - tmp3;
        var z = (r.Z + r.Y).Square() - tmp1 - zsquared;
        var y = (tmp3 - x) * tmp4;
        tmp2 += tmp2;
        tmp2 += tmp2;
        tmp2 += tmp2;
        y -= tmp2;
        r = new(in x, in y, in z);
        tmp3 = tmp4 * zsquared;
        tmp3 += tmp3;
        tmp3 = -tmp3;
        tmp6 = tmp6.Square() - tmp0 - tmp5;
        tmp1 += tmp1;
        tmp1 += tmp1;
        tmp6 -= tmp1;
        tmp0 = r.Z * zsquared;
        tmp0 += tmp0;

        return (tmp0, tmp3, tmp6);
    }

    public static (Fp2, Fp2, Fp2) AdditionStep(ref G2Projective r, in G2Affine q)
    {
        // Adaptation of Algorithm 27, https://eprint.iacr.org/2010/354.pdf
        var zsquared = r.Z.Square();
        var ysquared = q.Y.Square();
        var t0 = zsquared * q.X;
        var t1 = ((q.Y + r.Z).Square() - ysquared - zsquared) * zsquared;
        var t2 = t0 - r.X;
        var t3 = t2.Square();
        var t4 = t3 + t3;
        t4 += t4;
        var t5 = t4 * t2;
        var t6 = t1 - r.Y - r.Y;
        var t9 = t6 * q.X;
        var t7 = t4 * r.X;
        var x = t6.Square() - t5 - t7 - t7;
        var z = (r.Z + t2).Square() - zsquared - t3;
        var t10 = q.Y + z;
        var t8 = (t7 - x) * t6;
        t0 = r.Y * t5;
        t0 += t0;
        var y = t8 - t0;
        r = new(in x, in y, in z);
        t10 = t10.Square() - ysquared;
        var ztsquared = r.Z.Square();
        t10 -= ztsquared;
        t9 = t9 + t9 - t10;
        t10 = r.Z + r.Z;
        t6 = -t6;
        t1 = t6 + t6;

        return (t10, t1, t9);
    }
}
