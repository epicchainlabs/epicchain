// Copyright (C) 2021-2024 The EpicChain Labs.
//
// StorageItem.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.IO;
using Neo.VM;
using System;
using System.IO;
using System.Numerics;

namespace Neo.SmartContract
{
    /// <summary>
    /// Represents the values in contract storage.
    /// </summary>
    public class StorageItem : ISerializable
    {
        private ReadOnlyMemory<byte> value;
        private object cache;

        public int Size => Value.GetVarSize();

        /// <summary>
        /// The byte array value of the <see cref="StorageItem"/>.
        /// </summary>
        public ReadOnlyMemory<byte> Value
        {
            get
            {
                return !value.IsEmpty ? value : value = cache switch
                {
                    BigInteger bi => bi.ToByteArrayStandard(),
                    IInteroperable interoperable => BinarySerializer.Serialize(interoperable.ToStackItem(null), ExecutionEngineLimits.Default),
                    null => ReadOnlyMemory<byte>.Empty,
                    _ => throw new InvalidCastException()
                };
            }
            set
            {
                this.value = value;
                cache = null;
            }
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="StorageItem"/> class.
        /// </summary>
        public StorageItem() { }

        /// <summary>
        /// Initializes a new instance of the <see cref="StorageItem"/> class.
        /// </summary>
        /// <param name="value">The byte array value of the <see cref="StorageItem"/>.</param>
        public StorageItem(byte[] value)
        {
            this.value = value;
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="StorageItem"/> class.
        /// </summary>
        /// <param name="value">The integer value of the <see cref="StorageItem"/>.</param>
        public StorageItem(BigInteger value)
        {
            cache = value;
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="StorageItem"/> class.
        /// </summary>
        /// <param name="interoperable">The <see cref="IInteroperable"/> value of the <see cref="StorageItem"/>.</param>
        public StorageItem(IInteroperable interoperable)
        {
            cache = interoperable;
        }

        /// <summary>
        /// Increases the integer value in the store by the specified value.
        /// </summary>
        /// <param name="integer">The integer to add.</param>
        public void Add(BigInteger integer)
        {
            Set(this + integer);
        }

        /// <summary>
        /// Creates a new instance of <see cref="StorageItem"/> with the same value as this instance.
        /// </summary>
        /// <returns>The created <see cref="StorageItem"/>.</returns>
        public StorageItem Clone()
        {
            return new()
            {
                value = value,
                cache = cache is IInteroperable interoperable ? interoperable.Clone() : cache
            };
        }

        public void Deserialize(ref MemoryReader reader)
        {
            Value = reader.ReadToEnd();
        }

        /// <summary>
        /// Copies the value of another <see cref="StorageItem"/> instance to this instance.
        /// </summary>
        /// <param name="replica">The instance to be copied.</param>
        public void FromReplica(StorageItem replica)
        {
            value = replica.value;
            if (replica.cache is IInteroperable interoperable)
            {
                if (cache?.GetType() == interoperable.GetType())
                    ((IInteroperable)cache).FromReplica(interoperable);
                else
                    cache = interoperable.Clone();
            }
            else
            {
                cache = replica.cache;
            }
        }

        /// <summary>
        /// Gets an <see cref="IInteroperable"/> from the storage.
        /// </summary>
        /// <typeparam name="T">The type of the <see cref="IInteroperable"/>.</typeparam>
        /// <returns>The <see cref="IInteroperable"/> in the storage.</returns>
        public T GetInteroperable<T>() where T : IInteroperable, new()
        {
            if (cache is null)
            {
                var interoperable = new T();
                interoperable.FromStackItem(BinarySerializer.Deserialize(value, ExecutionEngineLimits.Default));
                cache = interoperable;
            }
            value = null;
            return (T)cache;
        }

        /// <summary>
        /// Gets an <see cref="IInteroperable"/> from the storage.
        /// </summary>
        /// <param name="verify">Verify deserialization</param>
        /// <typeparam name="T">The type of the <see cref="IInteroperable"/>.</typeparam>
        /// <returns>The <see cref="IInteroperable"/> in the storage.</returns>
        public T GetInteroperable<T>(bool verify = true) where T : IInteroperableVerifiable, new()
        {
            if (cache is null)
            {
                var interoperable = new T();
                interoperable.FromStackItem(BinarySerializer.Deserialize(value, ExecutionEngineLimits.Default), verify);
                cache = interoperable;
            }
            value = null;
            return (T)cache;
        }

        public void Serialize(BinaryWriter writer)
        {
            writer.Write(Value.Span);
        }

        /// <summary>
        /// Sets the integer value of the storage.
        /// </summary>
        /// <param name="integer">The integer value to set.</param>
        public void Set(BigInteger integer)
        {
            cache = integer;
            value = null;
        }

        /// <summary>
        /// Sets the interoperable value of the storage.
        /// </summary>
        /// <param name="interoperable">The <see cref="IInteroperable"/> value of the <see cref="StorageItem"/>.</param>
        public void Set(IInteroperable interoperable)
        {
            cache = interoperable;
            value = null;
        }

        public static implicit operator BigInteger(StorageItem item)
        {
            item.cache ??= new BigInteger(item.value.Span);
            return (BigInteger)item.cache;
        }
    }
}
