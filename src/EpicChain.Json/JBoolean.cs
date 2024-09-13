// Copyright (C) 2021-2024 EpicChain Labs.

//
// JBoolean.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System.Text.Json;

namespace EpicChain.Json
{
    /// <summary>
    /// Represents a JSON boolean value.
    /// </summary>
    public class JBoolean : JToken
    {
        /// <summary>
        /// Gets the value of the JSON token.
        /// </summary>
        public bool Value { get; }

        /// <summary>
        /// Initializes a new instance of the <see cref="JBoolean"/> class with the specified value.
        /// </summary>
        /// <param name="value">The value of the JSON token.</param>
        public JBoolean(bool value = false)
        {
            Value = value;
        }

        public override bool AsBoolean()
        {
            return Value;
        }

        /// <summary>
        /// Converts the current JSON token to a floating point number.
        /// </summary>
        /// <returns>The number 1 if value is <see langword="true"/>; otherwise, 0.</returns>
        public override double AsNumber()
        {
            return Value ? 1 : 0;
        }

        public override string AsString()
        {
            return Value.ToString().ToLowerInvariant();
        }

        public override bool GetBoolean() => Value;

        public override string ToString()
        {
            return AsString();
        }

        internal override void Write(Utf8JsonWriter writer)
        {
            writer.WriteBooleanValue(Value);
        }

        public override JToken Clone()
        {
            return this;
        }

        public static implicit operator JBoolean(bool value)
        {
            return new JBoolean(value);
        }

        public static bool operator ==(JBoolean left, JBoolean right)
        {
            return left.Value.Equals(right.Value);
        }

        public static bool operator !=(JBoolean left, JBoolean right)
        {
            return !left.Value.Equals(right.Value);
        }

        public override bool Equals(object? obj)
        {
            if (ReferenceEquals(this, obj))
            {
                return true;
            }
            if (obj is JBoolean other)
            {
                return Value.Equals(other.Value);
            }
            return false;
        }

        public override int GetHashCode()
        {
            return Value.GetHashCode();
        }
    }
}
