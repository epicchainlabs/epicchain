// Copyright (C) 2021-2024 EpicChain Labs.

//
// JArray.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System.Collections;
using System.Text.Json;

namespace EpicChain.Json
{
    /// <summary>
    /// Represents a JSON array.
    /// </summary>
    public class JArray : JContainer, IList<JToken?>
    {
        private readonly List<JToken?> items = new();

        /// <summary>
        /// Initializes a new instance of the <see cref="JArray"/> class.
        /// </summary>
        /// <param name="items">The initial items in the array.</param>
        public JArray(params JToken?[] items) : this((IEnumerable<JToken?>)items)
        {
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="JArray"/> class.
        /// </summary>
        /// <param name="items">The initial items in the array.</param>
        public JArray(IEnumerable<JToken?> items)
        {
            this.items.AddRange(items);
        }

        public override JToken? this[int index]
        {
            get
            {
                return items[index];
            }
            set
            {
                items[index] = value;
            }
        }

        public override IReadOnlyList<JToken?> Children => items;

        public bool IsReadOnly
        {
            get
            {
                return false;
            }
        }

        public void Add(JToken? item)
        {
            items.Add(item);
        }

        public override string AsString()
        {
            return ToString();
        }

        public override void Clear()
        {
            items.Clear();
        }

        public bool Contains(JToken? item)
        {
            return items.Contains(item);
        }

        public IEnumerator<JToken?> GetEnumerator()
        {
            return items.GetEnumerator();
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            return GetEnumerator();
        }

        public int IndexOf(JToken? item)
        {
            return items.IndexOf(item);
        }

        public void Insert(int index, JToken? item)
        {
            items.Insert(index, item);
        }

        public bool Remove(JToken? item)
        {
            return items.Remove(item);
        }

        public void RemoveAt(int index)
        {
            items.RemoveAt(index);
        }

        internal override void Write(Utf8JsonWriter writer)
        {
            writer.WriteStartArray();
            foreach (JToken? item in items)
            {
                if (item is null)
                    writer.WriteNullValue();
                else
                    item.Write(writer);
            }
            writer.WriteEndArray();
        }

        public override JToken Clone()
        {
            var cloned = new JArray();

            foreach (JToken? item in items)
            {
                cloned.Add(item?.Clone());
            }

            return cloned;
        }

        public static implicit operator JArray(JToken?[] value)
        {
            return new JArray(value);
        }
    }
}
