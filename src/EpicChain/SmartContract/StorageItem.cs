// Copyright (C) 2021-2024 EpicChain Labs.

//
// StorageItem.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Extensions;
using EpicChain.IO;
using EpicChain.VM;
using System;
using System.IO;
using System.Numerics;

namespace EpicChain.SmartContract
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

        public static implicit operator StorageItem(BigInteger value)
        {
            return new StorageItem(value);
        }

        public static implicit operator StorageItem(byte[] value)
        {
            return new StorageItem(value);
        }
    }
}
