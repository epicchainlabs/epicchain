// Copyright (C) 2021-2024 The EpicChain Labs.
//
// FIFOCache.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using System.Collections.Generic;

namespace Neo.IO.Caching
{
    internal abstract class FIFOCache<TKey, TValue> : Cache<TKey, TValue>
    {
        public FIFOCache(int max_capacity, IEqualityComparer<TKey> comparer = null)
            : base(max_capacity, comparer)
        {
        }

        protected override void OnAccess(CacheItem item)
        {
        }
    }
}
