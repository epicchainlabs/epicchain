// Copyright (C) 2021-2024 The EpicChain Labs.
//
// Tree.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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

namespace Neo.Plugins.RpcServer
{
    class Tree<T>
    {
        public TreeNode<T> Root { get; private set; }

        public TreeNode<T> AddRoot(T item)
        {
            if (Root is not null)
                throw new InvalidOperationException();
            Root = new TreeNode<T>(item, null);
            return Root;
        }

        public IEnumerable<T> GetItems()
        {
            if (Root is null) yield break;
            foreach (T item in Root.GetItems())
                yield return item;
        }
    }
}
