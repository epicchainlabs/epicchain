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
// UT_BigDecimal.cs file belongs to the neo project and is free
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
using System.Numerics;

namespace Neo.UnitTests
{
    [TestClass]
    public class UT_BigDecimal
    {
        [TestMethod]
        public void TestChangeDecimals()
        {
            BigDecimal originalValue = new(new BigInteger(12300), 5);
            BigDecimal result1 = originalValue.ChangeDecimals(7);
            result1.Value.Should().Be(new BigInteger(1230000));
            result1.Decimals.Should().Be(7);
            BigDecimal result2 = originalValue.ChangeDecimals(3);
            result2.Value.Should().Be(new BigInteger(123));
            result2.Decimals.Should().Be(3);
            BigDecimal result3 = originalValue.ChangeDecimals(5);
            result3.Value.Should().Be(originalValue.Value);
            Action action = () => originalValue.ChangeDecimals(2);
            action.Should().Throw<ArgumentOutOfRangeException>();
        }

        [TestMethod]
        public void TestBigDecimalConstructor()
        {
            BigDecimal value = new(new BigInteger(45600), 7);
            value.Value.Should().Be(new BigInteger(45600));
            value.Decimals.Should().Be(7);

            value = new BigDecimal(new BigInteger(0), 5);
            value.Value.Should().Be(new BigInteger(0));
            value.Decimals.Should().Be(5);

            value = new BigDecimal(new BigInteger(-10), 0);
            value.Value.Should().Be(new BigInteger(-10));
            value.Decimals.Should().Be(0);

            value = new BigDecimal(123.456789M, 6);
            value.Value.Should().Be(new BigInteger(123456789));
            value.Decimals.Should().Be(6);

            value = new BigDecimal(-123.45M, 3);
            value.Value.Should().Be(new BigInteger(-123450));
            value.Decimals.Should().Be(3);

            value = new BigDecimal(123.45M, 2);
            value.Value.Should().Be(new BigInteger(12345));
            value.Decimals.Should().Be(2);

            value = new BigDecimal(123M, 0);
            value.Value.Should().Be(new BigInteger(123));
            value.Decimals.Should().Be(0);

            value = new BigDecimal(0M, 0);
            value.Value.Should().Be(new BigInteger(0));
            value.Decimals.Should().Be(0);

            value = new BigDecimal(5.5M, 1);
            var b = new BigDecimal(55M);
            value.Value.Should().Be(b.Value);
        }

        [TestMethod]
        public void TestGetDecimals()
        {
            BigDecimal value = new(new BigInteger(45600), 7);
            value.Sign.Should().Be(1);
            value = new BigDecimal(new BigInteger(0), 5);
            value.Sign.Should().Be(0);
            value = new BigDecimal(new BigInteger(-10), 0);
            value.Sign.Should().Be(-1);
        }

        [TestMethod]
        public void TestCompareDecimals()
        {
            BigDecimal a = new(5.5M, 1);
            BigDecimal b = new(55M);
            BigDecimal c = new(55M, 1);
            a.Equals(b).Should().Be(false);
            a.Equals(c).Should().Be(false);
            b.Equals(c).Should().Be(true);
            a.CompareTo(b).Should().Be(-1);
            a.CompareTo(c).Should().Be(-1);
            b.CompareTo(c).Should().Be(0);
        }

        [TestMethod]
        public void TestGetSign()
        {
            BigDecimal value = new(new BigInteger(45600), 7);
            value.Sign.Should().Be(1);
            value = new BigDecimal(new BigInteger(0), 5);
            value.Sign.Should().Be(0);
            value = new BigDecimal(new BigInteger(-10), 0);
            value.Sign.Should().Be(-1);
        }

        [TestMethod]
        public void TestParse()
        {
            string s = "12345";
            byte decimals = 0;
            BigDecimal.Parse(s, decimals).Should().Be(new BigDecimal(new BigInteger(12345), 0));

            s = "abcdEfg";
            Action action = () => BigDecimal.Parse(s, decimals);
            action.Should().Throw<FormatException>();
        }

        [TestMethod]
        public void TestToString()
        {
            BigDecimal value = new(new BigInteger(100000), 5);
            value.ToString().Should().Be("1");
            value = new BigDecimal(new BigInteger(123456), 5);
            value.ToString().Should().Be("1.23456");
        }

        [TestMethod]
        public void TestTryParse()
        {
            string s = "12345";
            byte decimals = 0;
            BigDecimal.TryParse(s, decimals, out BigDecimal result).Should().BeTrue();
            result.Should().Be(new BigDecimal(new BigInteger(12345), 0));

            s = "12345E-5";
            decimals = 5;
            BigDecimal.TryParse(s, decimals, out result).Should().BeTrue();
            result.Should().Be(new BigDecimal(new BigInteger(12345), 5));

            s = "abcdEfg";
            BigDecimal.TryParse(s, decimals, out result).Should().BeFalse();
            result.Should().Be(default(BigDecimal));

            s = "123.45";
            decimals = 2;
            BigDecimal.TryParse(s, decimals, out result).Should().BeTrue();
            result.Should().Be(new BigDecimal(new BigInteger(12345), 2));

            s = "123.45E-5";
            decimals = 7;
            BigDecimal.TryParse(s, decimals, out result).Should().BeTrue();
            result.Should().Be(new BigDecimal(new BigInteger(12345), 7));

            s = "12345E-5";
            decimals = 3;
            BigDecimal.TryParse(s, decimals, out result).Should().BeFalse();
            result.Should().Be(default(BigDecimal));

            s = "1.2345";
            decimals = 3;
            BigDecimal.TryParse(s, decimals, out result).Should().BeFalse();
            result.Should().Be(default(BigDecimal));

            s = "1.2345E-5";
            decimals = 3;
            BigDecimal.TryParse(s, decimals, out result).Should().BeFalse();
            result.Should().Be(default(BigDecimal));

            s = "12345";
            decimals = 3;
            BigDecimal.TryParse(s, decimals, out result).Should().BeTrue();
            result.Should().Be(new BigDecimal(new BigInteger(12345000), 3));

            s = "12345E-2";
            decimals = 3;
            BigDecimal.TryParse(s, decimals, out result).Should().BeTrue();
            result.Should().Be(new BigDecimal(new BigInteger(123450), 3));

            s = "123.45";
            decimals = 3;
            BigDecimal.TryParse(s, decimals, out result).Should().BeTrue();
            result.Should().Be(new BigDecimal(new BigInteger(123450), 3));

            s = "123.45E3";
            decimals = 3;
            BigDecimal.TryParse(s, decimals, out result).Should().BeTrue();
            result.Should().Be(new BigDecimal(new BigInteger(123450000), 3));

            s = "a456bcdfg";
            decimals = 0;
            BigDecimal.TryParse(s, decimals, out result).Should().BeFalse();
            result.Should().Be(default(BigDecimal));

            s = "a456bce-5";
            decimals = 5;
            BigDecimal.TryParse(s, decimals, out result).Should().BeFalse();
            result.Should().Be(default(BigDecimal));

            s = "a4.56bcd";
            decimals = 5;
            BigDecimal.TryParse(s, decimals, out result).Should().BeFalse();
            result.Should().Be(default(BigDecimal));

            s = "a4.56bce3";
            decimals = 2;
            BigDecimal.TryParse(s, decimals, out result).Should().BeFalse();
            result.Should().Be(default(BigDecimal));

            s = "a456bcd";
            decimals = 2;
            BigDecimal.TryParse(s, decimals, out result).Should().BeFalse();
            result.Should().Be(default(BigDecimal));

            s = "a456bcdE3";
            decimals = 2;
            BigDecimal.TryParse(s, decimals, out result).Should().BeFalse();
            result.Should().Be(default(BigDecimal));

            s = "a456b.cd";
            decimals = 5;
            BigDecimal.TryParse(s, decimals, out result).Should().BeFalse();
            result.Should().Be(default(BigDecimal));

            s = "a456b.cdE3";
            decimals = 5;
            BigDecimal.TryParse(s, decimals, out result).Should().BeFalse();
            result.Should().Be(default(BigDecimal));
        }
    }
}
