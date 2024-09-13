// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_MemorySnapshot.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Microsoft.VisualStudio.TestTools.UnitTesting;
using EpicChain.Persistence;
using System.Linq;

namespace EpicChain.UnitTests.Persistence;

[TestClass]
public class UT_MemorySnapshot
{
    private MemoryStore _memoryStore;
    private MemorySnapshot _snapshot;

    [TestInitialize]
    public void Setup()
    {
        _memoryStore = new MemoryStore();
        _snapshot = _memoryStore.GetSnapshot() as MemorySnapshot;
    }

    [TestCleanup]
    public void CleanUp()
    {
        _memoryStore.Reset();
    }

    [TestMethod]
    public void SingleSnapshotTest()
    {
        var key1 = new byte[] { 0x01, 0x02 };
        var value1 = new byte[] { 0x03, 0x04 };

        _snapshot.Delete(key1);
        Assert.IsNull(_snapshot.TryGet(key1));

        // Both Store and Snapshot can not get the value that are cached in the snapshot
        _snapshot.Put(key1, value1);
        Assert.IsNull(_snapshot.TryGet(key1));
        Assert.IsNull(_memoryStore.TryGet(key1));

        _snapshot.Commit();

        // After commit the snapshot, the value can be get from the store but still can not get from the snapshot
        CollectionAssert.AreEqual(value1, _memoryStore.TryGet(key1));
        Assert.IsNull(_snapshot.TryGet(key1));

        _snapshot.Delete(key1);

        // Deleted value can not be found from the snapshot but can still get from the store
        // This is because snapshot has no key1 at all.
        Assert.IsFalse(_snapshot.Contains(key1));
        Assert.IsTrue(_memoryStore.Contains(key1));

        _snapshot.Commit();

        // After commit the snapshot, the value can not be found from the store
        Assert.IsFalse(_memoryStore.Contains(key1));

        // Test seek in order
        _snapshot.Put([0x00, 0x00, 0x04], [0x04]);
        _snapshot.Put([0x00, 0x00, 0x00], [0x00]);
        _snapshot.Put([0x00, 0x00, 0x01], [0x01]);
        _snapshot.Put([0x00, 0x00, 0x02], [0x02]);
        _snapshot.Put([0x00, 0x00, 0x03], [0x03]);

        // Can not get anything from the snapshot
        var entries = _snapshot.Seek([0x00, 0x00, 0x02]).ToArray();
        Assert.AreEqual(0, entries.Length);
    }

    [TestMethod]
    public void MultiSnapshotTest()
    {
        var key1 = new byte[] { 0x01, 0x02 };
        var value1 = new byte[] { 0x03, 0x04 };

        _snapshot.Delete(key1);
        Assert.IsNull(_snapshot.TryGet(key1));

        // Both Store and Snapshot can not get the value that are cached in the snapshot
        _snapshot.Put(key1, value1);
        // After commit the snapshot, the value can be get from the store but still can not get from the snapshot
        // But can get the value from a new snapshot
        _snapshot.Commit();
        var snapshot2 = _memoryStore.GetSnapshot();
        CollectionAssert.AreEqual(value1, _memoryStore.TryGet(key1));
        Assert.IsNull(_snapshot.TryGet(key1));
        CollectionAssert.AreEqual(value1, snapshot2.TryGet(key1));

        _snapshot.Delete(key1);

        // Deleted value can not being found from the snapshot but can still get from the store and snapshot2
        Assert.IsFalse(_snapshot.Contains(key1));
        Assert.IsTrue(_memoryStore.Contains(key1));
        Assert.IsTrue(snapshot2.Contains(key1));

        _snapshot.Commit();

        // After commit the snapshot, the value can not be found from the store, but can be found in snapshots
        // Cause snapshot1 or store can not change the status of snapshot2.
        Assert.IsFalse(_memoryStore.Contains(key1));
        Assert.IsTrue(snapshot2.Contains(key1));
        Assert.IsFalse(_snapshot.Contains(key1));

        // Add value via snapshot2 will not affect snapshot1 at all
        snapshot2.Put(key1, value1);
        snapshot2.Commit();
        Assert.IsNull(_snapshot.TryGet(key1));
        CollectionAssert.AreEqual(value1, snapshot2.TryGet(key1));
    }
}
