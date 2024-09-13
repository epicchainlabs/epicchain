// Copyright (C) 2021-2024 EpicChain Labs.

//
// FpConstants.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


namespace EpicChain.Cryptography.BLS12_381;

static class FpConstants
{
    // p = 4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787
    public static readonly ulong[] MODULUS =
    {
        0xb9fe_ffff_ffff_aaab,
        0x1eab_fffe_b153_ffff,
        0x6730_d2a0_f6b0_f624,
        0x6477_4b84_f385_12bf,
        0x4b1b_a7b6_434b_acd7,
        0x1a01_11ea_397f_e69a
    };

    // p - 2
    public static readonly ulong[] P_2 =
    {
        0xb9fe_ffff_ffff_aaa9,
        0x1eab_fffe_b153_ffff,
        0x6730_d2a0_f6b0_f624,
        0x6477_4b84_f385_12bf,
        0x4b1b_a7b6_434b_acd7,
        0x1a01_11ea_397f_e69a
    };

    // (p + 1) / 4
    public static readonly ulong[] P_1_4 =
    {
        0xee7f_bfff_ffff_eaab,
        0x07aa_ffff_ac54_ffff,
        0xd9cc_34a8_3dac_3d89,
        0xd91d_d2e1_3ce1_44af,
        0x92c6_e9ed_90d2_eb35,
        0x0680_447a_8e5f_f9a6
    };

    // INV = -(p^{-1} mod 2^64) mod 2^64
    public const ulong INV = 0x89f3_fffc_fffc_fffd;

    // R = 2^384 mod p
    public static readonly Fp R = Fp.FromRawUnchecked(new ulong[]
    {
        0x7609_0000_0002_fffd,
        0xebf4_000b_c40c_0002,
        0x5f48_9857_53c7_58ba,
        0x77ce_5853_7052_5745,
        0x5c07_1a97_a256_ec6d,
        0x15f6_5ec3_fa80_e493
    });

    // R2 = 2^(384*2) mod p
    public static readonly Fp R2 = Fp.FromRawUnchecked(new ulong[]
    {
        0xf4df_1f34_1c34_1746,
        0x0a76_e6a6_09d1_04f1,
        0x8de5_476c_4c95_b6d5,
        0x67eb_88a9_939d_83c0,
        0x9a79_3e85_b519_952d,
        0x1198_8fe5_92ca_e3aa
    });

    // R3 = 2^(384*3) mod p
    public static readonly Fp R3 = Fp.FromRawUnchecked(new ulong[]
    {
        0xed48_ac6b_d94c_a1e0,
        0x315f_831e_03a7_adf8,
        0x9a53_352a_615e_29dd,
        0x34c0_4e5e_921e_1761,
        0x2512_d435_6572_4728,
        0x0aa6_3460_9175_5d4d
    });
}
