// Copyright (C) 2021-2024 EpicChain Labs.

//
// Store.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.IO;

namespace EpicChain.Plugins.Storage
{
    internal class Store : IStore
    {
        private readonly RocksDb db;

        public Store(string path)
        {
            db = RocksDb.Open(Options.Default, Path.GetFullPath(path));
        }

        public void Dispose()
        {
            db.Dispose();
        }

        public ISnapshot GetSnapshot()
        {
            return new Snapshot(db);
        }

        public IEnumerable<(byte[] Key, byte[] Value)> Seek(byte[] keyOrPrefix, SeekDirection direction = SeekDirection.Forward)
        {
            if (keyOrPrefix == null) keyOrPrefix = Array.Empty<byte>();

            using var it = db.NewIterator();
            if (direction == SeekDirection.Forward)
                for (it.Seek(keyOrPrefix); it.Valid(); it.Next())
                    yield return (it.Key(), it.Value());
            else
                for (it.SeekForPrev(keyOrPrefix); it.Valid(); it.Prev())
                    yield return (it.Key(), it.Value());
        }

        public bool Contains(byte[] key)
        {
            return db.Get(key, Array.Empty<byte>(), 0, 0) >= 0;
        }

        public byte[] TryGet(byte[] key)
        {
            return db.Get(key);
        }

        public void Delete(byte[] key)
        {
            db.Remove(key);
        }

        public void Put(byte[] key, byte[] value)
        {
            db.Put(key, value);
        }

        public void PutSync(byte[] key, byte[] value)
        {
            db.Put(key, value, writeOptions: Options.WriteDefaultSync);
        }
    }
}
