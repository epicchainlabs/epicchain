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


using EpicChain.IO.Data.LevelDB;
using EpicChain.Persistence;
using System.Collections.Generic;
using LSnapshot = EpicChain.IO.Data.LevelDB.Snapshot;

namespace EpicChain.Plugins.Storage
{
    internal class Snapshot : ISnapshot
    {
        private readonly DB db;
        private readonly LSnapshot snapshot;
        private readonly ReadOptions options;
        private readonly WriteBatch batch;

        public Snapshot(DB db)
        {
            this.db = db;
            snapshot = db.GetSnapshot();
            options = new ReadOptions { FillCache = false, Snapshot = snapshot };
            batch = new WriteBatch();
        }

        public void Commit()
        {
            db.Write(WriteOptions.Default, batch);
        }

        public void Delete(byte[] key)
        {
            batch.Delete(key);
        }

        public void Dispose()
        {
            snapshot.Dispose();
        }

        public IEnumerable<(byte[] Key, byte[] Value)> Seek(byte[] prefix, SeekDirection direction = SeekDirection.Forward)
        {
            return db.Seek(options, prefix, direction, (k, v) => (k, v));
        }

        public void Put(byte[] key, byte[] value)
        {
            batch.Put(key, value);
        }

        public bool Contains(byte[] key)
        {
            return db.Contains(options, key);
        }

        public byte[] TryGet(byte[] key)
        {
            return db.Get(options, key);
        }
    }
}
