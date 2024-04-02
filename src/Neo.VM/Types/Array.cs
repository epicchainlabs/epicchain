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
// Array.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using System;
using System.Collections;
using System.Collections.Generic;

namespace Neo.VM.Types
{
    /// <summary>
    /// Represents an array or a complex object in the VM.
    /// </summary>
    public class Array : CompoundType, IReadOnlyList<StackItem>
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
        public Array(IEnumerable<StackItem>? items = null)
            : this(null, items)
        {
        }

        /// <summary>
        /// Create an array containing the specified items. And make the array use the specified <see cref="ReferenceCounter"/>.
        /// </summary>
        /// <param name="referenceCounter">The <see cref="ReferenceCounter"/> to be used by this array.</param>
        /// <param name="items">The items to be included in the array.</param>
        public Array(ReferenceCounter? referenceCounter, IEnumerable<StackItem>? items = null)
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
            Array result = this is Struct ? new Struct(ReferenceCounter) : new Array(ReferenceCounter);
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
}
