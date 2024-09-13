// Copyright (C) 2021-2024 EpicChain Labs.

//
// ReflectionCache.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Collections.Generic;
using System.Reflection;

namespace EpicChain.IO.Caching
{
    internal static class ReflectionCache<T>
        where T : Enum
    {
        private static readonly Dictionary<T, Type> s_dictionary = [];

        public static int Count => s_dictionary.Count;

        static ReflectionCache()
        {
            foreach (var field in typeof(T).GetFields(BindingFlags.Public | BindingFlags.Static))
            {
                // Get attribute
                var attribute = field.GetCustomAttribute<ReflectionCacheAttribute>();
                if (attribute == null) continue;

                // Append to cache
                s_dictionary.Add((T)field.GetValue(null), attribute.Type);
            }
        }

        public static object CreateInstance(T key, object def = null)
        {
            // Get Type from cache
            if (s_dictionary.TryGetValue(key, out var t))
                return Activator.CreateInstance(t);

            // return null
            return def;
        }

        public static ISerializable CreateSerializable(T key, ReadOnlyMemory<byte> data)
        {
            if (s_dictionary.TryGetValue(key, out var t))
                return data.AsSerializable(t);
            return null;
        }
    }
}
