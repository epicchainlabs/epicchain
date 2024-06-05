// Copyright (C) 2021-2024 The EpicChain Labs.
//
// HeaderCache.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


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
