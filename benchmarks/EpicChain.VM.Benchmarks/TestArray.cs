// Copyright (C) 2021-2024 EpicChain Labs.

//
// TestArray.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.VM.Types;
using System.Collections;

namespace EpicChain.VM.Benchmark;

public class TestArray : CompoundType, IReadOnlyList<StackItem>
{
    protected readonly List<StackItem> _array;

    /// <summary>
    /// Get or set item in the array.
    /// </summary>
    /// <param name="index">The index of the item in the array.</param>
    /// <returns>The item at the specified index.</returns>
    public StackItem this[int index]
    {
        get => _array[index];
        set
        {
            if (IsReadOnly) throw new InvalidOperationException("The object is readonly.");
            ReferenceCounter?.RemoveReference(_array[index], this);
            _array[index] = value;
            ReferenceCounter?.AddReference(value, this);
        }
    }

    /// <summary>
    /// The number of items in the array.
    /// </summary>
    public override int Count => _array.Count;
    public override IEnumerable<StackItem> SubItems => _array;
    public override int SubItemsCount => _array.Count;
    public override StackItemType Type => StackItemType.Array;

    /// <summary>
    /// Create an array containing the specified items.
    /// </summary>
    /// <param name="items">The items to be included in the array.</param>
    public TestArray(IEnumerable<StackItem>? items = null)
        : this(null, items)
    {
    }

    /// <summary>
    /// Create an array containing the specified items. And make the array use the specified <see cref="ReferenceCounter"/>.
    /// </summary>
    /// <param name="referenceCounter">The <see cref="ReferenceCounter"/> to be used by this array.</param>
    /// <param name="items">The items to be included in the array.</param>
    public TestArray(ReferenceCounter? referenceCounter, IEnumerable<StackItem>? items = null)
        : base(referenceCounter)
    {
        _array = items switch
        {
            null => new List<StackItem>(),
            List<StackItem> list => list,
            _ => new List<StackItem>(items)
        };
        if (referenceCounter != null)
            foreach (StackItem item in _array)
                referenceCounter.AddReference(item, this);
    }

    /// <summary>
    /// Add a new item at the end of the array.
    /// </summary>
    /// <param name="item">The item to be added.</param>
    public void Add(StackItem item)
    {
        if (IsReadOnly) throw new InvalidOperationException("The object is readonly.");
        _array.Add(item);
        ReferenceCounter?.AddReference(item, this);
    }

    public override void Clear()
    {
        if (IsReadOnly) throw new InvalidOperationException("The object is readonly.");
        if (ReferenceCounter != null)
            foreach (StackItem item in _array)
                ReferenceCounter.RemoveReference(item, this);
        _array.Clear();
    }

    public override StackItem ConvertTo(StackItemType type)
    {
        if (Type == StackItemType.Array && type == StackItemType.Struct)
            return new Struct(ReferenceCounter, new List<StackItem>(_array));
        return base.ConvertTo(type);
    }

    internal sealed override StackItem DeepCopy(Dictionary<StackItem, StackItem> refMap, bool asImmutable)
    {
        if (refMap.TryGetValue(this, out StackItem? mappedItem)) return mappedItem;
        var result = this is TestStruct ? new TestStruct(ReferenceCounter) : new TestArray(ReferenceCounter);
        refMap.Add(this, result);
        foreach (StackItem item in _array)
            result.Add(item.DeepCopy(refMap, asImmutable));
        result.IsReadOnly = true;
        return result;
    }

    IEnumerator IEnumerable.GetEnumerator()
    {
        return GetEnumerator();
    }

    public IEnumerator<StackItem> GetEnumerator()
    {
        return _array.GetEnumerator();
    }

    /// <summary>
    /// Remove the item at the specified index.
    /// </summary>
    /// <param name="index">The index of the item to be removed.</param>
    public void RemoveAt(int index)
    {
        if (IsReadOnly) throw new InvalidOperationException("The object is readonly.");
        ReferenceCounter?.RemoveReference(_array[index], this);
        _array.RemoveAt(index);
    }

    /// <summary>
    /// Reverse all items in the array.
    /// </summary>
    public void Reverse()
    {
        if (IsReadOnly) throw new InvalidOperationException("The object is readonly.");
        _array.Reverse();
    }
}