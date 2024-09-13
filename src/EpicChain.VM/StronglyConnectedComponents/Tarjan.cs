// Copyright (C) 2021-2024 EpicChain Labs.

//
// Tarjan.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using T = EpicChain.VM.Types.StackItem;

namespace EpicChain.VM.StronglyConnectedComponents
{
    class Tarjan
    {
        private readonly IEnumerable<T> vertexs;
        private readonly LinkedList<HashSet<T>> components = new();
        private readonly Stack<T> stack = new();
        private int index = 0;

        public Tarjan(IEnumerable<T> vertexs)
        {
            this.vertexs = vertexs;
        }

        public LinkedList<HashSet<T>> Invoke()
        {
            foreach (var v in vertexs)
            {
                if (v.DFN < 0)
                {
                    StrongConnectNonRecursive(v);
                }
            }
            return components;
        }

        private void StrongConnect(T v)
        {
            v.DFN = v.LowLink = ++index;
            stack.Push(v);
            v.OnStack = true;

            foreach (T w in v.Successors)
            {
                if (w.DFN < 0)
                {
                    StrongConnect(w);
                    v.LowLink = Math.Min(v.LowLink, w.LowLink);
                }
                else if (w.OnStack)
                {
                    v.LowLink = Math.Min(v.LowLink, w.DFN);
                }
            }

            if (v.LowLink == v.DFN)
            {
                HashSet<T> scc = new(ReferenceEqualityComparer.Instance);
                T w;
                do
                {
                    w = stack.Pop();
                    w.OnStack = false;
                    scc.Add(w);
                } while (v != w);
                components.AddLast(scc);
            }
        }

        private void StrongConnectNonRecursive(T v)
        {
            Stack<(T node, T?, IEnumerator<T>?, int)> sstack = new();
            sstack.Push((v, null, null, 0));
            while (sstack.TryPop(out var state))
            {
                v = state.node;
                var (_, w, s, n) = state;
                switch (n)
                {
                    case 0:
                        v.DFN = v.LowLink = ++index;
                        stack.Push(v);
                        v.OnStack = true;
                        s = v.Successors.GetEnumerator();
                        goto case 2;
                    case 1:
                        v.LowLink = Math.Min(v.LowLink, w!.LowLink);
                        goto case 2;
                    case 2:
                        while (s!.MoveNext())
                        {
                            w = s.Current;
                            if (w.DFN < 0)
                            {
                                sstack.Push((v, w, s, 1));
                                v = w;
                                goto case 0;
                            }
                            else if (w.OnStack)
                            {
                                v.LowLink = Math.Min(v.LowLink, w.DFN);
                            }
                        }
                        if (v.LowLink == v.DFN)
                        {
                            HashSet<T> scc = new(ReferenceEqualityComparer.Instance);
                            do
                            {
                                w = stack.Pop();
                                w.OnStack = false;
                                scc.Add(w);
                            } while (v != w);
                            components.AddLast(scc);
                        }
                        break;
                }
            }
        }
    }
}
