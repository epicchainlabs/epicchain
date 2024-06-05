// Copyright (C) 2021-2024 The EpicChain Labs.
//
// KeyedCollectionSlim.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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

namespace Neo.IO.Caching;

abstract class KeyedCollectionSlim<TKey, TItem> where TKey : notnull
{
    private readonly LinkedList<TItem> _items = new();
    private readonly Dictionary<TKey, LinkedListNode<TItem>> dict = new();

    public int Count => dict.Count;
    public TItem First => _items.First.Value;

    protected abstract TKey GetKeyForItem(TItem item);

    public void Add(TItem item)
    {
        var key = GetKeyForItem(item);
        var node = _items.AddLast(item);
        if (!dict.TryAdd(key, node))
        {
            _items.RemoveLast();
            throw new ArgumentException("An element with the same key already exists in the collection.");
        }
    }

    public bool Contains(TKey key) => dict.ContainsKey(key);

    public void Remove(TKey key)
    {
        if (dict.Remove(key, out var node))
            _items.Remove(node);
    }

    public void RemoveFirst()
    {
        var key = GetKeyForItem(_items.First.Value);
        dict.Remove(key);
        _items.RemoveFirst();
    }
}
