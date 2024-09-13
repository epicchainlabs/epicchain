// Copyright (C) 2021-2024 EpicChain Labs.

//
// Snapshot.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Persistence;
using RocksDbSharp;
using System;
using System.Collections.Generic;

namespace EpicChain.Plugins.Storage
{
    internal class Snapshot : ISnapshot
    {
        private readonly RocksDb db;
        private readonly RocksDbSharp.Snapshot snapshot;
        private readonly WriteBatch batch;
        private readonly ReadOptions options;

        public Snapshot(RocksDb db)
        {
            this.db = db;
            snapshot = db.CreateSnapshot();
            batch = new WriteBatch();

            options = new ReadOptions();
            options.SetFillCache(false);
            options.SetSnapshot(snapshot);
        }

        public void Commit()
        {
            db.Write(batch, Options.WriteDefault);
        }

        public void Delete(byte[] key)
        {
            batch.Delete(key);
        }

        public void Put(byte[] key, byte[] value)
        {
            batch.Put(key, value);
        }

        public IEnumerable<(byte[] Key, byte[] Value)> Seek(byte[] keyOrPrefix, SeekDirection direction)
        {
            if (keyOrPrefix == null) keyOrPrefix = Array.Empty<byte>();

            using var it = db.NewIterator(readOptions: options);

            if (direction == SeekDirection.Forward)
                for (it.Seek(keyOrPrefix); it.Valid(); it.Next())
                    yield return (it.Key(), it.Value());
            else
                for (it.SeekForPrev(keyOrPrefix); it.Valid(); it.Prev())
                    yield return (it.Key(), it.Value());
        }

        public bool Contains(byte[] key)
        {
            return db.Get(key, Array.Empty<byte>(), 0, 0, readOptions: options) >= 0;
        }

        public byte[] TryGet(byte[] key)
        {
            return db.Get(key, readOptions: options);
        }

        public void Dispose()
        {
            snapshot.Dispose();
            batch.Dispose();
        }
    }
}
