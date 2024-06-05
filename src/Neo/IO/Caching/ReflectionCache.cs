// Copyright (C) 2021-2024 The EpicChain Labs.
//
// ReflectionCache.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using System;
using System.Collections.Generic;
using System.Reflection;

namespace Neo.IO.Caching
{
    internal static class ReflectionCache<T> where T : Enum
    {
        private static readonly Dictionary<T, Type> dictionary = new();

        public static int Count => dictionary.Count;

        static ReflectionCache()
        {
            foreach (FieldInfo field in typeof(T).GetFields(BindingFlags.Public | BindingFlags.Static))
            {
                // Get attribute
                ReflectionCacheAttribute attribute = field.GetCustomAttribute<ReflectionCacheAttribute>();
                if (attribute == null) continue;

                // Append to cache
                dictionary.Add((T)field.GetValue(null), attribute.Type);
            }
        }

        public static object CreateInstance(T key, object def = null)
        {
            // Get Type from cache
            if (dictionary.TryGetValue(key, out Type t))
                return Activator.CreateInstance(t);

            // return null
            return def;
        }

        public static ISerializable CreateSerializable(T key, ReadOnlyMemory<byte> data)
        {
            if (dictionary.TryGetValue(key, out Type t))
                return data.AsSerializable(t);
            return null;
        }
    }
}
