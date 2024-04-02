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
// G2Constants.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

namespace Neo.Cryptography.BLS12_381;

static class G2Constants
{
    public static readonly Fp2 GeneratorX = new(Fp.FromRawUnchecked(new ulong[]
    {
        0xf5f2_8fa2_0294_0a10,
        0xb3f5_fb26_87b4_961a,
        0xa1a8_93b5_3e2a_e580,
        0x9894_999d_1a3c_aee9,
        0x6f67_b763_1863_366b,
        0x0581_9192_4350_bcd7
    }), Fp.FromRawUnchecked(new ulong[]
    {
        0xa5a9_c075_9e23_f606,
        0xaaa0_c59d_bccd_60c3,
        0x3bb1_7e18_e286_7806,
        0x1b1a_b6cc_8541_b367,
        0xc2b6_ed0e_f215_8547,
        0x1192_2a09_7360_edf3
    }));

    public static readonly Fp2 GeneratorY = new(Fp.FromRawUnchecked(new ulong[]
    {
        0x4c73_0af8_6049_4c4a,
        0x597c_fa1f_5e36_9c5a,
        0xe7e6_856c_aa0a_635a,
        0xbbef_b5e9_6e0d_495f,
        0x07d3_a975_f0ef_25a2,
        0x0083_fd8e_7e80_dae5
    }), Fp.FromRawUnchecked(new ulong[]
    {
        0xadc0_fc92_df64_b05d,
        0x18aa_270a_2b14_61dc,
        0x86ad_ac6a_3be4_eba0,
        0x7949_5c4e_c93d_a33a,
        0xe717_5850_a43c_caed,
        0x0b2b_c2a1_63de_1bf2
    }));

    public static readonly Fp2 B = new(Fp.FromRawUnchecked(new ulong[]
    {
        0xaa27_0000_000c_fff3,
        0x53cc_0032_fc34_000a,
        0x478f_e97a_6b0a_807f,
        0xb1d3_7ebe_e6ba_24d7,
        0x8ec9_733b_bf78_ab2f,
        0x09d6_4551_3d83_de7e
    }), Fp.FromRawUnchecked(new ulong[]
    {
        0xaa27_0000_000c_fff3,
        0x53cc_0032_fc34_000a,
        0x478f_e97a_6b0a_807f,
        0xb1d3_7ebe_e6ba_24d7,
        0x8ec9_733b_bf78_ab2f,
        0x09d6_4551_3d83_de7e
    }));

    public static readonly Fp2 B3 = B + B + B;

    // 1 / ((u+1) ^ ((q-1)/3))
    public static readonly Fp2 PsiCoeffX = new(in Fp.Zero, Fp.FromRawUnchecked(new ulong[]
    {
        0x890dc9e4867545c3,
        0x2af322533285a5d5,
        0x50880866309b7e2c,
        0xa20d1b8c7e881024,
        0x14e4f04fe2db9068,
        0x14e56d3f1564853a
    }));

    // 1 / ((u+1) ^ (p-1)/2)
    public static readonly Fp2 PsiCoeffY = new(Fp.FromRawUnchecked(new ulong[]
    {
        0x3e2f585da55c9ad1,
        0x4294213d86c18183,
        0x382844c88b623732,
        0x92ad2afd19103e18,
        0x1d794e4fac7cf0b9,
        0x0bd592fc7d825ec8
    }), Fp.FromRawUnchecked(new ulong[]
    {
        0x7bcfa7a25aa30fda,
        0xdc17dec12a927e7c,
        0x2f088dd86b4ebef1,
        0xd1ca2087da74d4a7,
        0x2da2596696cebc1d,
        0x0e2b7eedbbfd87d2
    }));

    // 1 / 2 ^ ((q-1)/3)
    public static readonly Fp2 Psi2CoeffX = new(Fp.FromRawUnchecked(new ulong[]
    {
        0xcd03c9e48671f071,
        0x5dab22461fcda5d2,
        0x587042afd3851b95,
        0x8eb60ebe01bacb9e,
        0x03f97d6e83d050d2,
        0x18f0206554638741
    }), in Fp.Zero);
}
