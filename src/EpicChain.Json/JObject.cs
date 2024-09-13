// Copyright (C) 2021-2024 EpicChain Labs.

//
// JObject.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
    /// Represents a JSON object.
    /// </summary>
    public class JObject : JContainer
    {
        private readonly OrderedDictionary<string, JToken?> properties = new();

        /// <summary>
        /// Gets or sets the properties of the JSON object.
        /// </summary>
        public IDictionary<string, JToken?> Properties => properties;

        /// <summary>
        /// Gets or sets the properties of the JSON object.
        /// </summary>
        /// <param name="name">The name of the property to get or set.</param>
        /// <returns>The property with the specified name.</returns>
        public override JToken? this[string name]
        {
            get
            {
                if (Properties.TryGetValue(name, out JToken? value))
                    return value;
                return null;
            }
            set
            {
                Properties[name] = value;
            }
        }

        public override IReadOnlyList<JToken?> Children => properties.Values;

        /// <summary>
        /// Determines whether the JSON object contains a property with the specified name.
        /// </summary>
        /// <param name="key">The property name to locate in the JSON object.</param>
        /// <returns><see langword="true"/> if the JSON object contains a property with the name; otherwise, <see langword="false"/>.</returns>
        public bool ContainsProperty(string key)
        {
            return Properties.ContainsKey(key);
        }

        public override void Clear() => properties.Clear();

        internal override void Write(Utf8JsonWriter writer)
        {
            writer.WriteStartObject();
            foreach (var (key, value) in Properties)
            {
                writer.WritePropertyName(key);
                if (value is null)
                    writer.WriteNullValue();
                else
                    value.Write(writer);
            }
            writer.WriteEndObject();
        }

        /// <summary>
        /// Creates a copy of the current JSON object.
        /// </summary>
        /// <returns>A copy of the current JSON object.</returns>
        public override JToken Clone()
        {
            var cloned = new JObject();

            foreach (var (key, value) in Properties)
            {
                cloned[key] = value != null ? value.Clone() : Null;
            }

            return cloned;
        }
    }
}
