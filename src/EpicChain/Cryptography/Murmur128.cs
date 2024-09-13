// Copyright (C) 2021-2024 EpicChain Labs.

//
// Murmur128.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System;
using System.Buffers.Binary;
using System.Runtime.CompilerServices;
using System.Security.Cryptography;

namespace EpicChain.Cryptography
{
    /// <summary>
    /// Computes the 128 bits murmur hash for the input data.
    /// </summary>
    public sealed class Murmur128 : HashAlgorithm
    {
        private const ulong c1 = 0x87c37b91114253d5;
        private const ulong c2 = 0x4cf5ad432745937f;
        private const int r1 = 31;
        private const int r2 = 33;
        private const uint m = 5;
        private const uint n1 = 0x52dce729;
        private const uint n2 = 0x38495ab5;

        private readonly uint seed;
        private int length;

        public override int HashSize => 128;

        private ulong H1 { get; set; }
        private ulong H2 { get; set; }

        /// <summary>
        /// Initializes a new instance of the <see cref="Murmur128"/> class with the specified seed.
        /// </summary>
        /// <param name="seed">The seed to be used.</param>
        public Murmur128(uint seed)
        {
            this.seed = seed;
            Initialize();
        }

        protected override void HashCore(byte[] array, int ibStart, int cbSize)
        {
            HashCore(array.AsSpan(ibStart, cbSize));
        }

        protected override void HashCore(ReadOnlySpan<byte> source)
        {
            int cbSize = source.Length;
            length += cbSize;
            int remainder = cbSize & 15;
            int alignedLength = cbSize - remainder;
            for (int i = 0; i < alignedLength; i += 16)
            {
                ulong k1 = BinaryPrimitives.ReadUInt64LittleEndian(source[i..]);
                k1 *= c1;
                k1 = Helper.RotateLeft(k1, r1);
                k1 *= c2;
                H1 ^= k1;
                H1 = Helper.RotateLeft(H1, 27);
                H1 += H2;
                H1 = H1 * m + n1;

                ulong k2 = BinaryPrimitives.ReadUInt64LittleEndian(source[(i + 8)..]);
                k2 *= c2;
                k2 = Helper.RotateLeft(k2, r2);
                k2 *= c1;
                H2 ^= k2;
                H2 = Helper.RotateLeft(H2, 31);
                H2 += H1;
                H2 = H2 * m + n2;
            }

            if (remainder > 0)
            {
                ulong remainingBytesL = 0, remainingBytesH = 0;
                switch (remainder)
                {
                    case 15: remainingBytesH ^= (ulong)source[alignedLength + 14] << 48; goto case 14;
                    case 14: remainingBytesH ^= (ulong)source[alignedLength + 13] << 40; goto case 13;
                    case 13: remainingBytesH ^= (ulong)source[alignedLength + 12] << 32; goto case 12;
                    case 12: remainingBytesH ^= (ulong)source[alignedLength + 11] << 24; goto case 11;
                    case 11: remainingBytesH ^= (ulong)source[alignedLength + 10] << 16; goto case 10;
                    case 10: remainingBytesH ^= (ulong)source[alignedLength + 9] << 8; goto case 9;
                    case 9: remainingBytesH ^= (ulong)source[alignedLength + 8] << 0; goto case 8;
                    case 8: remainingBytesL ^= (ulong)source[alignedLength + 7] << 56; goto case 7;
                    case 7: remainingBytesL ^= (ulong)source[alignedLength + 6] << 48; goto case 6;
                    case 6: remainingBytesL ^= (ulong)source[alignedLength + 5] << 40; goto case 5;
                    case 5: remainingBytesL ^= (ulong)source[alignedLength + 4] << 32; goto case 4;
                    case 4: remainingBytesL ^= (ulong)source[alignedLength + 3] << 24; goto case 3;
                    case 3: remainingBytesL ^= (ulong)source[alignedLength + 2] << 16; goto case 2;
                    case 2: remainingBytesL ^= (ulong)source[alignedLength + 1] << 8; goto case 1;
                    case 1: remainingBytesL ^= (ulong)source[alignedLength] << 0; break;
                }

                H2 ^= Helper.RotateLeft(remainingBytesH * c2, r2) * c1;
                H1 ^= Helper.RotateLeft(remainingBytesL * c1, r1) * c2;
            }
        }

        protected override byte[] HashFinal()
        {
            byte[] buffer = new byte[sizeof(ulong) * 2];
            TryHashFinal(buffer, out _);
            return buffer;
        }

        protected override bool TryHashFinal(Span<byte> destination, out int bytesWritten)
        {
            ulong len = (ulong)length;
            H1 ^= len; H2 ^= len;

            H1 += H2;
            H2 += H1;

            H1 = FMix(H1);
            H2 = FMix(H2);

            H1 += H2;
            H2 += H1;

            if (BinaryPrimitives.TryWriteUInt64LittleEndian(destination, H1))
                BinaryPrimitives.TryWriteUInt64LittleEndian(destination[sizeof(ulong)..], H2);

            bytesWritten = Math.Min(destination.Length, sizeof(ulong) * 2);
            return bytesWritten == sizeof(ulong) * 2;
        }

        public override void Initialize()
        {
            H1 = H2 = seed;
            length = 0;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        private static ulong FMix(ulong h)
        {
            h = (h ^ (h >> 33)) * 0xff51afd7ed558ccd;
            h = (h ^ (h >> 33)) * 0xc4ceb9fe1a85ec53;

            return (h ^ (h >> 33));
        }
    }
}
