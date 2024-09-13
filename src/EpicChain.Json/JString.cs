// Copyright (C) 2021-2024 EpicChain Labs.

//
// JString.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System.Globalization;
using System.Text.Json;

namespace EpicChain.Json
{
    /// <summary>
    /// Represents a JSON string.
    /// </summary>
    public class JString : JToken
    {
        /// <summary>
        /// Gets the value of the JSON token.
        /// </summary>
        public string Value { get; }

        /// <summary>
        /// Initializes a new instance of the <see cref="JString"/> class with the specified value.
        /// </summary>
        /// <param name="value">The value of the JSON token.</param>
        public JString(string value)
        {
            Value = value ?? throw new ArgumentNullException(nameof(value));
        }

        /// <summary>
        /// Converts the current JSON token to a boolean value.
        /// </summary>
        /// <returns><see langword="true"/> if value is not empty; otherwise, <see langword="false"/>.</returns>
        public override bool AsBoolean()
        {
            return !string.IsNullOrEmpty(Value);
        }

        public override double AsNumber()
        {
            if (string.IsNullOrEmpty(Value)) return 0;
            return double.TryParse(Value, NumberStyles.Float, CultureInfo.InvariantCulture, out double result) ? result : double.NaN;
        }

        public override string AsString()
        {
            return Value;
        }

        public override string GetString() => Value;

        public override T AsEnum<T>(T defaultValue = default, bool ignoreCase = false)
        {
            try
            {
                return Enum.Parse<T>(Value, ignoreCase);
            }
            catch
            {
                return defaultValue;
            }
        }

        public override T GetEnum<T>(bool ignoreCase = false)
        {
            T result = Enum.Parse<T>(Value, ignoreCase);
            if (!Enum.IsDefined(typeof(T), result)) throw new InvalidCastException();
            return result;
        }

        internal override void Write(Utf8JsonWriter writer)
        {
            writer.WriteStringValue(Value);
        }

        public override JToken Clone()
        {
            return this;
        }

        public static implicit operator JString(Enum value)
        {
            return new JString(value.ToString());
        }

        public static implicit operator JString?(string? value)
        {
            return value == null ? null : new JString(value);
        }

        public static bool operator ==(JString left, JString? right)
        {
            if (right is null) return false;
            return ReferenceEquals(left, right) || left.Value.Equals(right.Value);
        }

        public static bool operator !=(JString left, JString right)
        {
            return !(left == right);
        }

        public override bool Equals(object? obj)
        {
            if (obj is null) return false;
            if (ReferenceEquals(this, obj)) return true;
            if (obj is JString other)
            {
                return this == other;
            }
            if (obj is string str)
            {
                return Value == str;
            }
            return false;
        }

        public override int GetHashCode()
        {
            return Value.GetHashCode();
        }
    }
}
