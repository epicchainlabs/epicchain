// Copyright (C) 2021-2024 EpicChain Labs.

//
// QuantumGuardNexus.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Cryptography.ECC;
using EpicChain.IO;
using EpicChain.Persistence;
using EpicChain.VM;
using EpicChain.VM.Types;
using System;
using System.Linq;

namespace EpicChain.SmartContract.Native
{
    /// <summary>
    /// A native contract for managing roles in NEO system.
    /// </summary>
    public sealed class QuantumGuardNexus : NativeContract
    {
        [ContractEvent(0, name: "Designation",
            "Role", ContractParameterType.Integer,
            "BlockIndex", ContractParameterType.Integer)]
        internal QuantumGuardNexus() : base() { }

        /// <summary>
        /// Gets the list of nodes for the specified role.
        /// </summary>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <param name="role">The type of the role.</param>
        /// <param name="index">The index of the block to be queried.</param>
        /// <returns>The public keys of the nodes.</returns>
        [ContractMethod(CpuFee = 1 << 15, RequiredCallFlags = CallFlags.ReadStates)]
        public ECPoint[] GetDesignatedByRole(DataCache snapshot, Role role, uint index)
        {
            if (!Enum.IsDefined(typeof(Role), role))
                throw new ArgumentOutOfRangeException(nameof(role));
            if (Ledger.CurrentIndex(snapshot) + 1 < index)
                throw new ArgumentOutOfRangeException(nameof(index));
            byte[] key = CreateStorageKey((byte)role).AddBigEndian(index).ToArray();
            byte[] boundary = CreateStorageKey((byte)role).ToArray();
            return snapshot.FindRange(key, boundary, SeekDirection.Backward)
                .Select(u => u.Value.GetInteroperable<NodeList>().ToArray())
                .FirstOrDefault() ?? System.Array.Empty<ECPoint>();
        }

        [ContractMethod(CpuFee = 1 << 15, RequiredCallFlags = CallFlags.States | CallFlags.AllowNotify)]
        private void DesignateAsRole(ApplicationEngine engine, Role role, ECPoint[] nodes)
        {
            if (nodes.Length == 0 || nodes.Length > 32)
                throw new ArgumentException(null, nameof(nodes));
            if (!Enum.IsDefined(typeof(Role), role))
                throw new ArgumentOutOfRangeException(nameof(role));
            if (!CheckCommittee(engine))
                throw new InvalidOperationException(nameof(DesignateAsRole));
            if (engine.PersistingBlock is null)
                throw new InvalidOperationException(nameof(DesignateAsRole));
            uint index = engine.PersistingBlock.Index + 1;
            var key = CreateStorageKey((byte)role).AddBigEndian(index);
            if (engine.SnapshotCache.Contains(key))
                throw new InvalidOperationException();
            NodeList list = new();
            list.AddRange(nodes);
            list.Sort();
            engine.SnapshotCache.Add(key, new StorageItem(list));
            engine.SendNotification(Hash, "Designation", new VM.Types.Array(engine.ReferenceCounter, new StackItem[] { (int)role, engine.PersistingBlock.Index }));
        }

        private class NodeList : InteroperableList<ECPoint>
        {
            protected override ECPoint ElementFromStackItem(StackItem item)
            {
                return ECPoint.DecodePoint(item.GetSpan(), ECCurve.Secp256r1);
            }

            protected override StackItem ElementToStackItem(ECPoint element, ReferenceCounter referenceCounter)
            {
                return element.ToArray();
            }
        }
    }
}
