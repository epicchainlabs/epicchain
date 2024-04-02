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
// HeaderCache.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.IO.Caching;
using Neo.Network.P2P.Payloads;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Threading;

namespace Neo.Ledger
{
    /// <summary>
    /// Used to cache the headers of the blocks that have not been received.
    /// </summary>
    public sealed class HeaderCache : IDisposable, IEnumerable<Header>
    {
        private readonly IndexedQueue<Header> headers = new();
        private readonly ReaderWriterLockSlim readerWriterLock = new();

        /// <summary>
        /// Gets the <see cref="Header"/> at the specified index in the cache.
        /// </summary>
        /// <param name="index">The zero-based index of the <see cref="Header"/> to get.</param>
        /// <returns>The <see cref="Header"/> at the specified index in the cache.</returns>
        public Header this[uint index]
        {
            get
            {
                readerWriterLock.EnterReadLock();
                try
                {
                    if (headers.Count == 0) return null;
                    uint firstIndex = headers[0].Index;
                    if (index < firstIndex) return null;
                    index -= firstIndex;
                    if (index >= headers.Count) return null;
                    return headers[(int)index];
                }
                finally
                {
                    readerWriterLock.ExitReadLock();
                }
            }
        }

        /// <summary>
        /// Gets the number of elements in the cache.
        /// </summary>
        public int Count => headers.Count;

        /// <summary>
        /// Indicates whether the cache is full.
        /// </summary>
        public bool Full => headers.Count >= 10000;

        /// <summary>
        /// Gets the last <see cref="Header"/> in the cache. Or <see langword="null"/> if the cache is empty.
        /// </summary>
        public Header Last
        {
            get
            {
                readerWriterLock.EnterReadLock();
                try
                {
                    if (headers.Count == 0) return null;
                    return headers[^1];
                }
                finally
                {
                    readerWriterLock.ExitReadLock();
                }
            }
        }

        public void Dispose()
        {
            readerWriterLock.Dispose();
        }

        internal void Add(Header header)
        {
            readerWriterLock.EnterWriteLock();
            try
            {
                headers.Enqueue(header);
            }
            finally
            {
                readerWriterLock.ExitWriteLock();
            }
        }

        internal bool TryRemoveFirst(out Header header)
        {
            readerWriterLock.EnterWriteLock();
            try
            {
                return headers.TryDequeue(out header);
            }
            finally
            {
                readerWriterLock.ExitWriteLock();
            }
        }

        public IEnumerator<Header> GetEnumerator()
        {
            readerWriterLock.EnterReadLock();
            try
            {
                foreach (Header header in headers)
                    yield return header;
            }
            finally
            {
                readerWriterLock.ExitReadLock();
            }
        }

        IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();
    }
}
