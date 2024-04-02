// EpicChain Copyright Project (2021-2024)
// 
// Copyright (c) 2021-2024 EpicChain
// 
// EpicChain is an innovative blockchain network developed and maintained by xmoohad. This copyright project outlines the rights and responsibilities associated with the EpicChain software and its related components.
// 
// 1. Copyright Holder:
//    - xmoohad
// 
// 2. Project Name:
//    - EpicChain
// 
// 3. Project Description:
//    - EpicChain is a decentralized blockchain network that aims to revolutionize the way digital assets are managed, traded, and secured. With its innovative features and robust architecture, EpicChain provides a secure and efficient platform for various decentralized applications (dApps) and digital asset management.
// 
// 4. Copyright Period:
//    - The copyright for the EpicChain software and its related components is valid from 2021 to 2024.
// 
// 5. Copyright Statement:
//    - All rights reserved. No part of the EpicChain software or its related components may be reproduced, distributed, or transmitted in any form or by any means, without the prior written permission of the copyright holder, except in the case of brief quotations embodied in critical reviews and certain other noncommercial uses permitted by copyright law.
// 
// 6. License:
//    - The EpicChain software is licensed under the EpicChain Software License, a custom license that governs the use, distribution, and modification of the software. The EpicChain Software License is designed to promote the free and open development of the EpicChain network while protecting the interests of the copyright holder.
// 
// 7. Open Source:
//    - EpicChain is an open-source project, and its source code is available to the public under the terms of the EpicChain Software License. Developers are encouraged to contribute to the development of EpicChain and create innovative applications on top of the EpicChain network.
// 
// 8. Disclaimer:
//    - The EpicChain software and its related components are provided "as is," without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall the copyright holder or contributors be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the EpicChain software or its related components.
// 
// 9. Contact Information:
//    - For inquiries regarding the EpicChain copyright project, please contact xmoohad at [email address].
// 
// 10. Updates:
//     - This copyright project may be updated or modified from time to time to reflect changes in the EpicChain project or to address new legal or regulatory requirements. Users and developers are encouraged to check the latest version of the copyright project periodically.

//
// Tarjan.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using System;
using System.Collections.Generic;
using T = Neo.VM.Types.StackItem;

namespace Neo.VM.StronglyConnectedComponents
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
