// Copyright (C) 2021-2024 EpicChain Labs.

//
// KeyedCollectionSlim.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Collections;
using System.Collections.Generic;

namespace EpicChain.IO.Caching;

internal abstract class KeyedCollectionSlim<TKey, TItem>
    where TKey : notnull
    where TItem : class, IStructuralEquatable, IStructuralComparable, IComparable
{
    private readonly LinkedList<TItem> _items = new();
    private readonly Dictionary<TKey, LinkedListNode<TItem>> _dict = [];

    public int Count => _dict.Count;
    public TItem? First => _items.First?.Value;

    protected abstract TKey GetKeyForItem(TItem? item);

    public void Add(TItem item)
    {
        var key = GetKeyForItem(item);
        var node = _items.AddLast(item);
        if (!_dict.TryAdd(key, node))
        {
            _items.RemoveLast();
            throw new ArgumentException("An element with the same key already exists in the collection.");
        }
    }

    public bool Contains(TKey key) => _dict.ContainsKey(key);

    public void Remove(TKey key)
    {
        if (_dict.Remove(key, out var node))
            _items.Remove(node);
    }

    public void RemoveFirst()
    {
        var key = GetKeyForItem(_items.First?.Value);
        _dict.Remove(key);
        _items.RemoveFirst();
    }
}
