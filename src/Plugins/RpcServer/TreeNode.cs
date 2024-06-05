// Copyright (C) 2021-2024 The EpicChain Labs.
//
// TreeNode.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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

namespace Neo.Plugins.RpcServer
{
    class TreeNode<T>
    {
        private readonly List<TreeNode<T>> children = new();

        public T Item { get; }
        public TreeNode<T> Parent { get; }
        public IReadOnlyList<TreeNode<T>> Children => children;

        internal TreeNode(T item, TreeNode<T> parent)
        {
            Item = item;
            Parent = parent;
        }

        public TreeNode<T> AddChild(T item)
        {
            TreeNode<T> child = new(item, this);
            children.Add(child);
            return child;
        }

        internal IEnumerable<T> GetItems()
        {
            yield return Item;
            foreach (var child in children)
                foreach (T item in child.GetItems())
                    yield return item;
        }
    }
}
