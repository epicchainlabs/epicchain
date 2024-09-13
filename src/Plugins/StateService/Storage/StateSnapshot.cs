// Copyright (C) 2021-2024 EpicChain Labs.

//
// StateSnapshot.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Cryptography.MPTTrie;
using EpicChain.IO;
using EpicChain.Persistence;
using EpicChain.Plugins.StateService.Network;
using System;

namespace EpicChain.Plugins.StateService.Storage
{
    class StateSnapshot : IDisposable
    {
        private readonly ISnapshot snapshot;
        public Trie Trie;

        public StateSnapshot(IStore store)
        {
            snapshot = store.GetSnapshot();
            Trie = new Trie(snapshot, CurrentLocalRootHash(), Settings.Default.FullState);
        }

        public StateRoot GetStateRoot(uint index)
        {
            return snapshot.TryGet(Keys.StateRoot(index))?.AsSerializable<StateRoot>();
        }

        public void AddLocalStateRoot(StateRoot state_root)
        {
            snapshot.Put(Keys.StateRoot(state_root.Index), state_root.ToArray());
            snapshot.Put(Keys.CurrentLocalRootIndex, BitConverter.GetBytes(state_root.Index));
        }

        public uint? CurrentLocalRootIndex()
        {
            var bytes = snapshot.TryGet(Keys.CurrentLocalRootIndex);
            if (bytes is null) return null;
            return BitConverter.ToUInt32(bytes);
        }

        public UInt256 CurrentLocalRootHash()
        {
            var index = CurrentLocalRootIndex();
            if (index is null) return null;
            return GetStateRoot((uint)index)?.RootHash;
        }

        public void AddValidatedStateRoot(StateRoot state_root)
        {
            if (state_root?.Witness is null)
                throw new ArgumentException(nameof(state_root) + " missing witness in invalidated state root");
            snapshot.Put(Keys.StateRoot(state_root.Index), state_root.ToArray());
            snapshot.Put(Keys.CurrentValidatedRootIndex, BitConverter.GetBytes(state_root.Index));
        }

        public uint? CurrentValidatedRootIndex()
        {
            var bytes = snapshot.TryGet(Keys.CurrentValidatedRootIndex);
            if (bytes is null) return null;
            return BitConverter.ToUInt32(bytes);
        }

        public UInt256 CurrentValidatedRootHash()
        {
            var index = CurrentLocalRootIndex();
            if (index is null) return null;
            var state_root = GetStateRoot((uint)index);
            if (state_root is null || state_root.Witness is null)
                throw new InvalidOperationException(nameof(CurrentValidatedRootHash) + " could not get validated state root");
            return state_root.RootHash;
        }

        public void Commit()
        {
            Trie.Commit();
            snapshot.Commit();
        }

        public void Dispose()
        {
            snapshot.Dispose();
        }
    }
}
