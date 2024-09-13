// Copyright (C) 2021-2024 EpicChain Labs.

//
// ConstantTimeUtility.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Runtime.InteropServices;

namespace EpicChain.Cryptography.BLS12_381;

public static class ConstantTimeUtility
{
    public static bool ConstantTimeEq<T>(in T a, in T b) where T : unmanaged
    {
        ReadOnlySpan<byte> a_bytes = MemoryMarshal.AsBytes(MemoryMarshal.CreateReadOnlySpan(ref Unsafe.AsRef(in a), 1));
        ReadOnlySpan<byte> b_bytes = MemoryMarshal.AsBytes(MemoryMarshal.CreateReadOnlySpan(ref Unsafe.AsRef(in b), 1));
        ReadOnlySpan<ulong> a_u64 = MemoryMarshal.Cast<byte, ulong>(a_bytes);
        ReadOnlySpan<ulong> b_u64 = MemoryMarshal.Cast<byte, ulong>(b_bytes);
        ulong f = 0;
        for (int i = 0; i < a_u64.Length; i++)
            f |= a_u64[i] ^ b_u64[i];
        for (int i = a_u64.Length * sizeof(ulong); i < a_bytes.Length; i++)
            f |= (ulong)a_bytes[i] ^ a_bytes[i];
        return f == 0;
    }

    public static T ConditionalSelect<T>(in T a, in T b, bool choice) where T : unmanaged
    {
        return choice ? b : a;
    }

    public static void ConditionalAssign<T>(this ref T self, in T other, bool choice) where T : unmanaged
    {
        self = ConditionalSelect(in self, in other, choice);
    }
}
