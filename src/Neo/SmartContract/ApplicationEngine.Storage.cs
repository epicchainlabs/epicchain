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
// ApplicationEngine.Storage.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.Persistence;
using Neo.SmartContract.Iterators;
using Neo.SmartContract.Native;
using System;

namespace Neo.SmartContract
{
    partial class ApplicationEngine
    {
        /// <summary>
        /// The maximum size of storage keys.
        /// </summary>
        public const int MaxStorageKeySize = 64;

        /// <summary>
        /// The maximum size of storage values.
        /// </summary>
        public const int MaxStorageValueSize = ushort.MaxValue;

        /// <summary>
        /// The <see cref="InteropDescriptor"/> of System.Storage.GetContext.
        /// Gets the storage context for the current contract.
        /// </summary>
        public static readonly InteropDescriptor System_Storage_GetContext = Register("System.Storage.GetContext", nameof(GetStorageContext), 1 << 4, CallFlags.ReadStates);

        /// <summary>
        /// The <see cref="InteropDescriptor"/> of System.Storage.GetReadOnlyContext.
        /// Gets the readonly storage context for the current contract.
        /// </summary>
        public static readonly InteropDescriptor System_Storage_GetReadOnlyContext = Register("System.Storage.GetReadOnlyContext", nameof(GetReadOnlyContext), 1 << 4, CallFlags.ReadStates);

        /// <summary>
        /// The <see cref="InteropDescriptor"/> of System.Storage.AsReadOnly.
        /// Converts the specified storage context to a new readonly storage context.
        /// </summary>
        public static readonly InteropDescriptor System_Storage_AsReadOnly = Register("System.Storage.AsReadOnly", nameof(AsReadOnly), 1 << 4, CallFlags.ReadStates);

        /// <summary>
        /// The <see cref="InteropDescriptor"/> of System.Storage.Get.
        /// Gets the entry with the specified key from the storage.
        /// </summary>
        public static readonly InteropDescriptor System_Storage_Get = Register("System.Storage.Get", nameof(Get), 1 << 15, CallFlags.ReadStates);

        /// <summary>
        /// The <see cref="InteropDescriptor"/> of System.Storage.Find.
        /// Finds the entries from the storage.
        /// </summary>
        public static readonly InteropDescriptor System_Storage_Find = Register("System.Storage.Find", nameof(Find), 1 << 15, CallFlags.ReadStates);

        /// <summary>
        /// The <see cref="InteropDescriptor"/> of System.Storage.Put.
        /// Puts a new entry into the storage.
        /// </summary>
        public static readonly InteropDescriptor System_Storage_Put = Register("System.Storage.Put", nameof(Put), 1 << 15, CallFlags.WriteStates);

        /// <summary>
        /// The <see cref="InteropDescriptor"/> of System.Storage.Delete.
        /// Deletes an entry from the storage.
        /// </summary>
        public static readonly InteropDescriptor System_Storage_Delete = Register("System.Storage.Delete", nameof(Delete), 1 << 15, CallFlags.WriteStates);

        /// <summary>
        /// The implementation of System.Storage.GetContext.
        /// Gets the storage context for the current contract.
        /// </summary>
        /// <returns>The storage context for the current contract.</returns>
        protected internal StorageContext GetStorageContext()
        {
            ContractState contract = NativeContract.ContractManagement.GetContract(Snapshot, CurrentScriptHash);
            return new StorageContext
            {
                Id = contract.Id,
                IsReadOnly = false
            };
        }

        /// <summary>
        /// The implementation of System.Storage.GetReadOnlyContext.
        /// Gets the readonly storage context for the current contract.
        /// </summary>
        /// <returns>The storage context for the current contract.</returns>
        protected internal StorageContext GetReadOnlyContext()
        {
            ContractState contract = NativeContract.ContractManagement.GetContract(Snapshot, CurrentScriptHash);
            return new StorageContext
            {
                Id = contract.Id,
                IsReadOnly = true
            };
        }

        /// <summary>
        /// The implementation of System.Storage.AsReadOnly.
        /// Converts the specified storage context to a new readonly storage context.
        /// </summary>
        /// <param name="context">The storage context to convert.</param>
        /// <returns>The readonly storage context.</returns>
        protected internal static StorageContext AsReadOnly(StorageContext context)
        {
            if (!context.IsReadOnly)
                context = new StorageContext
                {
                    Id = context.Id,
                    IsReadOnly = true
                };
            return context;
        }

        /// <summary>
        /// The implementation of System.Storage.Get.
        /// Gets the entry with the specified key from the storage.
        /// </summary>
        /// <param name="context">The context of the storage.</param>
        /// <param name="key">The key of the entry.</param>
        /// <returns>The value of the entry. Or <see langword="null"/> if the entry doesn't exist.</returns>
        protected internal ReadOnlyMemory<byte>? Get(StorageContext context, byte[] key)
        {
            return Snapshot.TryGet(new StorageKey
            {
                Id = context.Id,
                Key = key
            })?.Value;
        }

        /// <summary>
        /// The implementation of System.Storage.Find.
        /// Finds the entries from the storage.
        /// </summary>
        /// <param name="context">The context of the storage.</param>
        /// <param name="prefix">The prefix of keys to find.</param>
        /// <param name="options">The options of the search.</param>
        /// <returns>An iterator for the results.</returns>
        protected internal IIterator Find(StorageContext context, byte[] prefix, FindOptions options)
        {
            if ((options & ~FindOptions.All) != 0)
                throw new ArgumentOutOfRangeException(nameof(options));
            if (options.HasFlag(FindOptions.KeysOnly) && (options.HasFlag(FindOptions.ValuesOnly) || options.HasFlag(FindOptions.DeserializeValues) || options.HasFlag(FindOptions.PickField0) || options.HasFlag(FindOptions.PickField1)))
                throw new ArgumentException(null, nameof(options));
            if (options.HasFlag(FindOptions.ValuesOnly) && (options.HasFlag(FindOptions.KeysOnly) || options.HasFlag(FindOptions.RemovePrefix)))
                throw new ArgumentException(null, nameof(options));
            if (options.HasFlag(FindOptions.PickField0) && options.HasFlag(FindOptions.PickField1))
                throw new ArgumentException(null, nameof(options));
            if ((options.HasFlag(FindOptions.PickField0) || options.HasFlag(FindOptions.PickField1)) && !options.HasFlag(FindOptions.DeserializeValues))
                throw new ArgumentException(null, nameof(options));
            byte[] prefix_key = StorageKey.CreateSearchPrefix(context.Id, prefix);
            SeekDirection direction = options.HasFlag(FindOptions.Backwards) ? SeekDirection.Backward : SeekDirection.Forward;
            return new StorageIterator(Snapshot.Find(prefix_key, direction).GetEnumerator(), prefix.Length, options);
        }

        /// <summary>
        /// The implementation of System.Storage.Put.
        /// Puts a new entry into the storage.
        /// </summary>
        /// <param name="context">The context of the storage.</param>
        /// <param name="key">The key of the entry.</param>
        /// <param name="value">The value of the entry.</param>
        protected internal void Put(StorageContext context, byte[] key, byte[] value)
        {
            if (key.Length > MaxStorageKeySize) throw new ArgumentException("Key length too big", nameof(key));
            if (value.Length > MaxStorageValueSize) throw new ArgumentException("Value length too big", nameof(value));
            if (context.IsReadOnly) throw new ArgumentException("StorageContext is readonly", nameof(context));

            int newDataSize;
            StorageKey skey = new()
            {
                Id = context.Id,
                Key = key
            };
            StorageItem item = Snapshot.GetAndChange(skey);
            if (item is null)
            {
                newDataSize = key.Length + value.Length;
                Snapshot.Add(skey, item = new StorageItem());
            }
            else
            {
                if (value.Length == 0)
                    newDataSize = 0;
                else if (value.Length <= item.Value.Length)
                    newDataSize = (value.Length - 1) / 4 + 1;
                else if (item.Value.Length == 0)
                    newDataSize = value.Length;
                else
                    newDataSize = (item.Value.Length - 1) / 4 + 1 + value.Length - item.Value.Length;
            }
            AddGas(newDataSize * StoragePrice);

            item.Value = value;
        }

        /// <summary>
        /// The implementation of System.Storage.Delete.
        /// Deletes an entry from the storage.
        /// </summary>
        /// <param name="context">The context of the storage.</param>
        /// <param name="key">The key of the entry.</param>
        protected internal void Delete(StorageContext context, byte[] key)
        {
            if (context.IsReadOnly) throw new ArgumentException(null, nameof(context));
            Snapshot.Delete(new StorageKey
            {
                Id = context.Id,
                Key = key
            });
        }
    }
}
