// Copyright (C) 2021-2024 EpicChain Labs.

//
// BigDecimal.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Numerics;

namespace EpicChain
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
                decimals = buffer[14];
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
