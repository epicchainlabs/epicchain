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


using EpicChain.IO.Data.LevelDB;
using EpicChain.Persistence;
using System.Collections.Generic;

namespace EpicChain.Plugins.Storage
{
    internal class Store : IStore
    {
        private readonly DB db;

        public Store(string path)
        {
            db = DB.Open(path, new Options { CreateIfMissing = true, FilterPolicy = Native.leveldb_filterpolicy_create_bloom(15) });
        }

        public void Delete(byte[] key)
        {
            db.Delete(WriteOptions.Default, key);
        }

        public void Dispose()
        {
            db.Dispose();
        }

        public IEnumerable<(byte[], byte[])> Seek(byte[] prefix, SeekDirection direction = SeekDirection.Forward)
        {
            return db.Seek(ReadOptions.Default, prefix, direction, (k, v) => (k, v));
        }

        public ISnapshot GetSnapshot()
        {
            return new Snapshot(db);
        }

        public void Put(byte[] key, byte[] value)
        {
            db.Put(WriteOptions.Default, key, value);
        }

        public void PutSync(byte[] key, byte[] value)
        {
            db.Put(WriteOptions.SyncWrite, key, value);
        }

        public bool Contains(byte[] key)
        {
            return db.Contains(ReadOptions.Default, key);
        }

        public byte[] TryGet(byte[] key)
        {
            return db.Get(ReadOptions.Default, key);
        }
    }
}
