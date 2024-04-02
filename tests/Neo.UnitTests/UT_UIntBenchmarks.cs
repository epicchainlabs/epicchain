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
// UT_UIntBenchmarks.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using System;
using System.Diagnostics;

namespace Neo.UnitTests
{
    [TestClass]
    public class UT_UIntBenchmarks
    {
        private const int MAX_TESTS = 1000;

        byte[][] base_32_1;
        byte[][] base_32_2;
        byte[][] base_20_1;
        byte[][] base_20_2;

        private Random random;

        [TestInitialize]
        public void TestSetup()
        {
            int SEED = 123456789;
            random = new Random(SEED);

            base_32_1 = new byte[MAX_TESTS][];
            base_32_2 = new byte[MAX_TESTS][];
            base_20_1 = new byte[MAX_TESTS][];
            base_20_2 = new byte[MAX_TESTS][];

            for (var i = 0; i < MAX_TESTS; i++)
            {
                base_32_1[i] = RandomBytes(32);
                base_20_1[i] = RandomBytes(20);
                if (i % 2 == 0)
                {
                    base_32_2[i] = RandomBytes(32);
                    base_20_2[i] = RandomBytes(20);
                }
                else
                {
                    base_32_2[i] = new byte[32];
                    Buffer.BlockCopy(base_32_1[i], 0, base_32_2[i], 0, 32);
                    base_20_2[i] = new byte[20];
                    Buffer.BlockCopy(base_20_1[i], 0, base_20_2[i], 0, 20);
                }
            }
        }

        [TestMethod]
        public void Test_UInt160_Parse()
        {
            string uint160strbig = "0x0001020304050607080900010203040506070809";
            UInt160 num1 = UInt160.Parse(uint160strbig);
            num1.ToString().Should().Be("0x0001020304050607080900010203040506070809");

            string uint160strbig2 = "0X0001020304050607080900010203040506070809";
            UInt160 num2 = UInt160.Parse(uint160strbig2);
            num2.ToString().Should().Be("0x0001020304050607080900010203040506070809");
        }

        private byte[] RandomBytes(int count)
        {
            byte[] randomBytes = new byte[count];
            random.NextBytes(randomBytes);
            return randomBytes;
        }

        public delegate object BenchmarkMethod();

        public (TimeSpan, object) Benchmark(BenchmarkMethod method)
        {
            Stopwatch sw0 = new Stopwatch();
            sw0.Start();
            var result = method();
            sw0.Stop();
            TimeSpan elapsed = sw0.Elapsed;
            Console.WriteLine($"Elapsed={elapsed} Sum={result}");
            return (elapsed, result);
        }

        [TestMethod]
        public void Benchmark_CompareTo_UInt256()
        {
            // testing "official UInt256 version"
            UInt256[] uut_32_1 = new UInt256[MAX_TESTS];
            UInt256[] uut_32_2 = new UInt256[MAX_TESTS];

            for (var i = 0; i < MAX_TESTS; i++)
            {
                uut_32_1[i] = new UInt256(base_32_1[i]);
                uut_32_2[i] = new UInt256(base_32_2[i]);
            }

            var checksum0 = Benchmark(() =>
            {
                var checksum = 0;
                for (var i = 0; i < MAX_TESTS; i++)
                {
                    checksum += uut_32_1[i].CompareTo(uut_32_2[i]);
                }

                return checksum;
            }).Item2;

            var checksum1 = Benchmark(() =>
            {
                var checksum = 0;
                for (var i = 0; i < MAX_TESTS; i++)
                {
                    checksum += code1_UInt256CompareTo(base_32_1[i], base_32_2[i]);
                }

                return checksum;
            }).Item2;

            var checksum2 = Benchmark(() =>
            {
                var checksum = 0;
                for (var i = 0; i < MAX_TESTS; i++)
                {
                    checksum += code2_UInt256CompareTo(base_32_1[i], base_32_2[i]);
                }

                return checksum;
            }).Item2;

            var checksum3 = Benchmark(() =>
            {
                var checksum = 0;
                for (var i = 0; i < MAX_TESTS; i++)
                {
                    checksum += code3_UInt256CompareTo(base_32_1[i], base_32_2[i]);
                }

                return checksum;
            }).Item2;

            checksum0.Should().Be(checksum1);
            checksum0.Should().Be(checksum2);
            checksum0.Should().Be(checksum3);
        }

        [TestMethod]
        public void Benchmark_CompareTo_UInt160()
        {
            // testing "official UInt160 version"
            UInt160[] uut_20_1 = new UInt160[MAX_TESTS];
            UInt160[] uut_20_2 = new UInt160[MAX_TESTS];

            for (var i = 0; i < MAX_TESTS; i++)
            {
                uut_20_1[i] = new UInt160(base_20_1[i]);
                uut_20_2[i] = new UInt160(base_20_2[i]);
            }

            var checksum0 = Benchmark(() =>
            {
                var checksum = 0;
                for (var i = 0; i < MAX_TESTS; i++)
                {
                    checksum += uut_20_1[i].CompareTo(uut_20_2[i]);
                }

                return checksum;
            }).Item2;

            var checksum1 = Benchmark(() =>
            {
                var checksum = 0;
                for (var i = 0; i < MAX_TESTS; i++)
                {
                    checksum += code1_UInt160CompareTo(base_20_1[i], base_20_2[i]);
                }

                return checksum;
            }).Item2;

            var checksum2 = Benchmark(() =>
            {
                var checksum = 0;
                for (var i = 0; i < MAX_TESTS; i++)
                {
                    checksum += code2_UInt160CompareTo(base_20_1[i], base_20_2[i]);
                }

                return checksum;
            }).Item2;

            var checksum3 = Benchmark(() =>
            {
                var checksum = 0;
                for (var i = 0; i < MAX_TESTS; i++)
                {
                    checksum += code3_UInt160CompareTo(base_20_1[i], base_20_2[i]);
                }

                return checksum;
            }).Item2;

            checksum0.Should().Be(checksum1);
            checksum0.Should().Be(checksum2);
            checksum0.Should().Be(checksum3);
        }

        [TestMethod]
        public void Benchmark_UInt_IsCorrect_Self_CompareTo()
        {
            for (var i = 0; i < MAX_TESTS; i++)
            {
                code1_UInt160CompareTo(base_20_1[i], base_20_1[i]).Should().Be(0);
                code2_UInt160CompareTo(base_20_1[i], base_20_1[i]).Should().Be(0);
                code3_UInt160CompareTo(base_20_1[i], base_20_1[i]).Should().Be(0);
                code1_UInt256CompareTo(base_32_1[i], base_32_1[i]).Should().Be(0);
                code2_UInt256CompareTo(base_32_1[i], base_32_1[i]).Should().Be(0);
                code3_UInt256CompareTo(base_32_1[i], base_32_1[i]).Should().Be(0);
            }
        }

        private int code1_UInt256CompareTo(byte[] b1, byte[] b2)
        {
            byte[] x = b1;
            byte[] y = b2;
            for (int i = x.Length - 1; i >= 0; i--)
            {
                if (x[i] > y[i])
                    return 1;
                if (x[i] < y[i])
                    return -1;
            }
            return 0;
        }

        private unsafe int code2_UInt256CompareTo(byte[] b1, byte[] b2)
        {
            fixed (byte* px = b1, py = b2)
            {
                uint* lpx = (uint*)px;
                uint* lpy = (uint*)py;
                for (int i = 256 / 32 - 1; i >= 0; i--)
                {
                    if (lpx[i] > lpy[i])
                        return 1;
                    if (lpx[i] < lpy[i])
                        return -1;
                }
            }
            return 0;
        }

        private unsafe int code3_UInt256CompareTo(byte[] b1, byte[] b2)
        {
            fixed (byte* px = b1, py = b2)
            {
                ulong* lpx = (ulong*)px;
                ulong* lpy = (ulong*)py;
                for (int i = 256 / 64 - 1; i >= 0; i--)
                {
                    if (lpx[i] > lpy[i])
                        return 1;
                    if (lpx[i] < lpy[i])
                        return -1;
                }
            }
            return 0;
        }
        private int code1_UInt160CompareTo(byte[] b1, byte[] b2)
        {
            byte[] x = b1;
            byte[] y = b2;
            for (int i = x.Length - 1; i >= 0; i--)
            {
                if (x[i] > y[i])
                    return 1;
                if (x[i] < y[i])
                    return -1;
            }
            return 0;
        }

        private unsafe int code2_UInt160CompareTo(byte[] b1, byte[] b2)
        {
            fixed (byte* px = b1, py = b2)
            {
                uint* lpx = (uint*)px;
                uint* lpy = (uint*)py;
                for (int i = 160 / 32 - 1; i >= 0; i--)
                {
                    if (lpx[i] > lpy[i])
                        return 1;
                    if (lpx[i] < lpy[i])
                        return -1;
                }
            }
            return 0;
        }

        private unsafe int code3_UInt160CompareTo(byte[] b1, byte[] b2)
        {
            // LSB -----------------> MSB
            // --------------------------
            // | 8B      | 8B      | 4B |
            // --------------------------
            //   0l        1l        4i
            // --------------------------
            fixed (byte* px = b1, py = b2)
            {
                uint* ipx = (uint*)px;
                uint* ipy = (uint*)py;
                if (ipx[4] > ipy[4])
                    return 1;
                if (ipx[4] < ipy[4])
                    return -1;

                ulong* lpx = (ulong*)px;
                ulong* lpy = (ulong*)py;
                if (lpx[1] > lpy[1])
                    return 1;
                if (lpx[1] < lpy[1])
                    return -1;
                if (lpx[0] > lpy[0])
                    return 1;
                if (lpx[0] < lpy[0])
                    return -1;
            }
            return 0;
        }

    }
}
