// Copyright (C) 2021-2024 EpicChain Labs.

//
// ApplicationEngine.Storage.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.SmartContract.Iterators;
using EpicChain.SmartContract.Native;
using System;

namespace EpicChain.SmartContract
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
            ContractState contract = NativeContract.ContractManagement.GetContract(SnapshotCache, CurrentScriptHash);
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
            ContractState contract = NativeContract.ContractManagement.GetContract(SnapshotCache, CurrentScriptHash);
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
            return SnapshotCache.TryGet(new StorageKey
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
            return new StorageIterator(SnapshotCache.Find(prefix_key, direction).GetEnumerator(), prefix.Length, options);
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
            StorageItem item = SnapshotCache.GetAndChange(skey);
            if (item is null)
            {
                newDataSize = key.Length + value.Length;
                SnapshotCache.Add(skey, item = new StorageItem());
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
            AddFee(newDataSize * StoragePrice);

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
            SnapshotCache.Delete(new StorageKey
            {
                Id = context.Id,
                Key = key
            });
        }
    }
}
