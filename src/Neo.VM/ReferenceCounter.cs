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
// ReferenceCounter.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.VM.StronglyConnectedComponents;
using Neo.VM.Types;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.CompilerServices;

namespace Neo.VM
{
    /// <summary>
    /// Used for reference counting of objects in the VM.
    /// </summary>
    public sealed class ReferenceCounter
    {
        private const bool TrackAllItems = false;

        private readonly HashSet<StackItem> tracked_items = new(ReferenceEqualityComparer.Instance);
        private readonly HashSet<StackItem> zero_referred = new(ReferenceEqualityComparer.Instance);
        private LinkedList<HashSet<StackItem>>? cached_components;
        private int references_count = 0;

        /// <summary>
        /// Indicates the number of this counter.
        /// </summary>
        public int Count => references_count;

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        private static bool NeedTrack(StackItem item)
        {
#pragma warning disable CS0162
            if (TrackAllItems) return true;
#pragma warning restore CS0162
            if (item is CompoundType or Buffer) return true;
            return false;
        }

        internal void AddReference(StackItem item, CompoundType parent)
        {
            references_count++;
            if (!NeedTrack(item)) return;
            cached_components = null;
            tracked_items.Add(item);
            item.ObjectReferences ??= new(ReferenceEqualityComparer.Instance);
            if (!item.ObjectReferences.TryGetValue(parent, out var pEntry))
            {
                pEntry = new(parent);
                item.ObjectReferences.Add(parent, pEntry);
            }
            pEntry.References++;
        }

        internal void AddStackReference(StackItem item, int count = 1)
        {
            references_count += count;
            if (!NeedTrack(item)) return;
            if (tracked_items.Add(item))
                cached_components?.AddLast(new HashSet<StackItem>(ReferenceEqualityComparer.Instance) { item });
            item.StackReferences += count;
            zero_referred.Remove(item);
        }

        internal void AddZeroReferred(StackItem item)
        {
            zero_referred.Add(item);
            if (!NeedTrack(item)) return;
            cached_components?.AddLast(new HashSet<StackItem>(ReferenceEqualityComparer.Instance) { item });
            tracked_items.Add(item);
        }

        internal int CheckZeroReferred()
        {
            if (zero_referred.Count > 0)
            {
                zero_referred.Clear();
                if (cached_components is null)
                {
                    //Tarjan<StackItem> tarjan = new(tracked_items.Where(p => p.StackReferences == 0));
                    Tarjan tarjan = new(tracked_items);
                    cached_components = tarjan.Invoke();
                }
                foreach (StackItem item in tracked_items)
                    item.Reset();
                for (var node = cached_components.First; node != null;)
                {
                    var component = node.Value;
                    bool on_stack = false;
                    foreach (StackItem item in component)
                    {
                        if (item.StackReferences > 0 || item.ObjectReferences?.Values.Any(p => p.References > 0 && p.Item.OnStack) == true)
                        {
                            on_stack = true;
                            break;
                        }
                    }
                    if (on_stack)
                    {
                        foreach (StackItem item in component)
                            item.OnStack = true;
                        node = node.Next;
                    }
                    else
                    {
                        foreach (StackItem item in component)
                        {
                            tracked_items.Remove(item);
                            if (item is CompoundType compound)
                            {
                                references_count -= compound.SubItemsCount;
                                foreach (StackItem subitem in compound.SubItems)
                                {
                                    if (component.Contains(subitem)) continue;
                                    if (!NeedTrack(subitem)) continue;
                                    subitem.ObjectReferences!.Remove(compound);
                                }
                            }
                            item.Cleanup();
                        }
                        var nodeToRemove = node;
                        node = node.Next;
                        cached_components.Remove(nodeToRemove);
                    }
                }
            }
            return references_count;
        }

        internal void RemoveReference(StackItem item, CompoundType parent)
        {
            references_count--;
            if (!NeedTrack(item)) return;
            cached_components = null;
            item.ObjectReferences![parent].References--;
            if (item.StackReferences == 0)
                zero_referred.Add(item);
        }

        internal void RemoveStackReference(StackItem item)
        {
            references_count--;
            if (!NeedTrack(item)) return;
            if (--item.StackReferences == 0)
                zero_referred.Add(item);
        }
    }
}
