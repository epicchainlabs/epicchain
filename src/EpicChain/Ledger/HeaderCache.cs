// Copyright (C) 2021-2024 EpicChain Labs.

//
// HeaderCache.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.IO.Caching;
using EpicChain.Network.P2P.Payloads;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Threading;

namespace EpicChain.Ledger
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
