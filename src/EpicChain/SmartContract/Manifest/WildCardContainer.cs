// Copyright (C) 2021-2024 EpicChain Labs.

//
// WildCardContainer.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Json;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace EpicChain.SmartContract.Manifest
{
    /// <summary>
    /// A list that supports wildcard.
    /// </summary>
    /// <typeparam name="T">The type of the elements.</typeparam>
    public class WildcardContainer<T> : IReadOnlyList<T>
    {
        private readonly T[] _data;

        public T this[int index] => _data[index];

        public int Count => _data?.Length ?? 0;

        /// <summary>
        /// Indicates whether the list is a wildcard.
        /// </summary>
        public bool IsWildcard => _data is null;

        private WildcardContainer(T[] data)
        {
            _data = data;
        }

        /// <summary>
        /// Creates a new instance of the <see cref="WildcardContainer{T}"/> class with the initial elements.
        /// </summary>
        /// <param name="data">The initial elements.</param>
        /// <returns>The created list.</returns>
        public static WildcardContainer<T> Create(params T[] data) => new(data);

        /// <summary>
        /// Creates a new instance of the <see cref="WildcardContainer{T}"/> class with wildcard.
        /// </summary>
        /// <returns>The created list.</returns>
        public static WildcardContainer<T> CreateWildcard() => new(null);

        /// <summary>
        /// Converts the list from a JSON object.
        /// </summary>
        /// <param name="json">The list represented by a JSON object.</param>
        /// <param name="elementSelector">A converter for elements.</param>
        /// <returns>The converted list.</returns>
        public static WildcardContainer<T> FromJson(JToken json, Func<JToken, T> elementSelector)
        {
            switch (json)
            {
                case JString str:
                    if (str.Value != "*") throw new FormatException();
                    return CreateWildcard();
                case JArray array:
                    return Create(array.Select(p => elementSelector(p)).ToArray());
                default:
                    throw new FormatException();
            }
        }

        public IEnumerator<T> GetEnumerator()
        {
            if (_data == null) return ((IReadOnlyList<T>)Array.Empty<T>()).GetEnumerator();

            return ((IReadOnlyList<T>)_data).GetEnumerator();
        }

        IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();

        /// <summary>
        /// Converts the list to a JSON object.
        /// </summary>
        /// <returns>The list represented by a JSON object.</returns>
        public JToken ToJson(Func<T, JToken> elementSelector)
        {
            if (IsWildcard) return "*";
            return _data.Select(p => elementSelector(p)).ToArray();
        }
    }
}
