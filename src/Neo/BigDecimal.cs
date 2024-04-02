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
// BigDecimal.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using System;
using System.Numerics;

namespace Neo
{
    /// <summary>
    /// Represents a fixed-point number of arbitrary precision.
    /// </summary>
    public struct BigDecimal : IComparable<BigDecimal>, IEquatable<BigDecimal>
    {
        private readonly BigInteger value;
        private readonly byte decimals;

        /// <summary>
        /// The <see cref="BigInteger"/> value of the number.
        /// </summary>
        public readonly BigInteger Value => value;

        /// <summary>
        /// The number of decimal places for this number.
        /// </summary>
        public readonly byte Decimals => decimals;

        /// <summary>
        /// The sign of the number.
        /// </summary>
        public readonly int Sign => value.Sign;

        /// <summary>
        /// Initializes a new instance of the <see cref="BigDecimal"/> struct.
        /// </summary>
        /// <param name="value">The <see cref="BigInteger"/> value of the number.</param>
        /// <param name="decimals">The number of decimal places for this number.</param>
        public BigDecimal(BigInteger value, byte decimals)
        {
            this.value = value;
            this.decimals = decimals;
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="BigDecimal"/> struct with the value of <see cref="decimal"/>.
        /// </summary>
        /// <param name="value">The value of the number.</param>
        public unsafe BigDecimal(decimal value)
        {
            Span<int> span = stackalloc int[4];
            span = decimal.GetBits(value);
            fixed (int* p = span)
            {
                ReadOnlySpan<byte> buffer = new(p, 16);
                this.value = new BigInteger(buffer[..12], isUnsigned: true);
                if (buffer[15] != 0) this.value = -this.value;
                this.decimals = buffer[14];
            }
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="BigDecimal"/> struct with the value of <see cref="decimal"/>.
        /// </summary>
        /// <param name="value">The value of the number.</param>
        /// <param name="decimals">The number of decimal places for this number.</param>
        public unsafe BigDecimal(decimal value, byte decimals)
        {
            Span<int> span = stackalloc int[4];
            span = decimal.GetBits(value);
            fixed (int* p = span)
            {
                ReadOnlySpan<byte> buffer = new(p, 16);
                this.value = new BigInteger(buffer[..12], isUnsigned: true);
                if (buffer[14] > decimals)
                    throw new ArgumentException(null, nameof(value));
                else if (buffer[14] < decimals)
                    this.value *= BigInteger.Pow(10, decimals - buffer[14]);
                if (buffer[15] != 0)
                    this.value = -this.value;
            }
            this.decimals = decimals;
        }

        /// <summary>
        /// Changes the decimals of the <see cref="BigDecimal"/>.
        /// </summary>
        /// <param name="decimals">The new decimals field.</param>
        /// <returns>The <see cref="BigDecimal"/> that has the new number of decimal places.</returns>
        public readonly BigDecimal ChangeDecimals(byte decimals)
        {
            if (this.decimals == decimals) return this;
            BigInteger value;
            if (this.decimals < decimals)
            {
                value = this.value * BigInteger.Pow(10, decimals - this.decimals);
            }
            else
            {
                BigInteger divisor = BigInteger.Pow(10, this.decimals - decimals);
                value = BigInteger.DivRem(this.value, divisor, out BigInteger remainder);
                if (remainder > BigInteger.Zero)
                    throw new ArgumentOutOfRangeException(nameof(decimals));
            }
            return new BigDecimal(value, decimals);
        }

        /// <summary>
        /// Parses a <see cref="BigDecimal"/> from the specified <see cref="string"/>.
        /// </summary>
        /// <param name="s">A number represented by a <see cref="string"/>.</param>
        /// <param name="decimals">The number of decimal places for this number.</param>
        /// <returns>The parsed <see cref="BigDecimal"/>.</returns>
        /// <exception cref="FormatException"><paramref name="s"/> is not in the correct format.</exception>
        public static BigDecimal Parse(string s, byte decimals)
        {
            if (!TryParse(s, decimals, out BigDecimal result))
                throw new FormatException();
            return result;
        }

        /// <summary>
        /// Gets a <see cref="string"/> representing the number.
        /// </summary>
        /// <returns>The <see cref="string"/> representing the number.</returns>
        public override readonly string ToString()
        {
            BigInteger divisor = BigInteger.Pow(10, decimals);
            BigInteger result = BigInteger.DivRem(value, divisor, out BigInteger remainder);
            if (remainder == 0) return result.ToString();
            return $"{result}.{remainder.ToString("d" + decimals)}".TrimEnd('0');
        }

        /// <summary>
        /// Parses a <see cref="BigDecimal"/> from the specified <see cref="string"/>.
        /// </summary>
        /// <param name="s">A number represented by a <see cref="string"/>.</param>
        /// <param name="decimals">The number of decimal places for this number.</param>
        /// <param name="result">The parsed <see cref="BigDecimal"/>.</param>
        /// <returns><see langword="true"/> if a number is successfully parsed; otherwise, <see langword="false"/>.</returns>
        public static bool TryParse(string s, byte decimals, out BigDecimal result)
        {
            int e = 0;
            int index = s.IndexOfAny(new[] { 'e', 'E' });
            if (index >= 0)
            {
                if (!sbyte.TryParse(s[(index + 1)..], out sbyte e_temp))
                {
                    result = default;
                    return false;
                }
                e = e_temp;
                s = s.Substring(0, index);
            }
            index = s.IndexOf('.');
            if (index >= 0)
            {
                s = s.TrimEnd('0');
                e -= s.Length - index - 1;
                s = s.Remove(index, 1);
            }
            int ds = e + decimals;
            if (ds < 0)
            {
                result = default;
                return false;
            }
            if (ds > 0)
                s += new string('0', ds);
            if (!BigInteger.TryParse(s, out BigInteger value))
            {
                result = default;
                return false;
            }
            result = new BigDecimal(value, decimals);
            return true;
        }

        public readonly int CompareTo(BigDecimal other)
        {
            BigInteger left = value, right = other.value;
            if (decimals < other.decimals)
                left *= BigInteger.Pow(10, other.decimals - decimals);
            else if (decimals > other.decimals)
                right *= BigInteger.Pow(10, decimals - other.decimals);
            return left.CompareTo(right);
        }

        public override readonly bool Equals(object obj)
        {
            if (obj is not BigDecimal @decimal) return false;
            return Equals(@decimal);
        }

        public readonly bool Equals(BigDecimal other)
        {
            return CompareTo(other) == 0;
        }

        public override readonly int GetHashCode()
        {
            BigInteger divisor = BigInteger.Pow(10, decimals);
            BigInteger result = BigInteger.DivRem(value, divisor, out BigInteger remainder);
            return HashCode.Combine(result, remainder);
        }

        public static bool operator ==(BigDecimal left, BigDecimal right)
        {
            return left.CompareTo(right) == 0;
        }

        public static bool operator !=(BigDecimal left, BigDecimal right)
        {
            return left.CompareTo(right) != 0;
        }

        public static bool operator <(BigDecimal left, BigDecimal right)
        {
            return left.CompareTo(right) < 0;
        }

        public static bool operator <=(BigDecimal left, BigDecimal right)
        {
            return left.CompareTo(right) <= 0;
        }

        public static bool operator >(BigDecimal left, BigDecimal right)
        {
            return left.CompareTo(right) > 0;
        }

        public static bool operator >=(BigDecimal left, BigDecimal right)
        {
            return left.CompareTo(right) >= 0;
        }
    }
}
