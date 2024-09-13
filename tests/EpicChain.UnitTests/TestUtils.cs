// Copyright (C) 2021-2024 EpicChain Labs.

//
// TestUtils.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using EpicChain.Cryptography;
using EpicChain.Cryptography.ECC;
using EpicChain.IO;
using EpicChain.Json;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Persistence;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Manifest;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using EpicChain.Wallets;
using EpicChain.Wallets.XEP6;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;

namespace EpicChain.UnitTests
{
    public static partial class TestUtils
    {
        public static readonly Random TestRandom = new Random(1337); // use fixed seed for guaranteed determinism

        public static UInt256 RandomUInt256()
        {
            byte[] data = new byte[32];
            TestRandom.NextBytes(data);
            return new UInt256(data);
        }

        public static UInt160 RandomUInt160()
        {
            byte[] data = new byte[20];
            TestRandom.NextBytes(data);
            return new UInt160(data);
        }

        public static StorageKey CreateStorageKey(this NativeContract contract, byte prefix, ISerializable key = null)
        {
            var k = new KeyBuilder(contract.Id, prefix);
            if (key != null) k = k.Add(key);
            return k;
        }

        public static StorageKey CreateStorageKey(this NativeContract contract, byte prefix, uint value)
        {
            return new KeyBuilder(contract.Id, prefix).AddBigEndian(value);
        }

        public static byte[] GetByteArray(int length, byte firstByte)
        {
            byte[] array = new byte[length];
            array[0] = firstByte;
            for (int i = 1; i < length; i++)
            {
                array[i] = 0x20;
            }
            return array;
        }

        public static XEP6Wallet GenerateTestWallet(string password)
        {
            JObject wallet = new JObject();
            wallet["name"] = "noname";
            wallet["version"] = new Version("1.0").ToString();
            wallet["scrypt"] = new ScryptParameters(2, 1, 1).ToJson();
            wallet["accounts"] = new JArray();
            wallet["extra"] = null;
            wallet.ToString().Should().Be("{\"name\":\"noname\",\"version\":\"1.0\",\"scrypt\":{\"n\":2,\"r\":1,\"p\":1},\"accounts\":[],\"extra\":null}");
            return new XEP6Wallet(null, password, TestProtocolSettings.Default, wallet);
        }

        internal static StorageItem GetStorageItem(byte[] value)
        {
            return new StorageItem
            {
                Value = value
            };
        }

        internal static StorageKey GetStorageKey(int id, byte[] keyValue)
        {
            return new StorageKey
            {
                Id = id,
                Key = keyValue
            };
        }

        public static void StorageItemAdd(DataCache snapshot, int id, byte[] keyValue, byte[] value)
        {
            snapshot.Add(new StorageKey
            {
                Id = id,
                Key = keyValue
            }, new StorageItem(value));
        }

        public static void FillMemoryPool(DataCache snapshot, EpicChainSystem system, XEP6Wallet wallet, WalletAccount account)
        {
            for (int i = 0; i < system.Settings.MemoryPoolMaxTransactions; i++)
            {
                var tx = CreateValidTx(snapshot, wallet, account);
                system.MemPool.TryAdd(tx, snapshot);
            }
        }

        public static T CopyMsgBySerialization<T>(T serializableObj, T newObj) where T : ISerializable
        {
            MemoryReader reader = new(serializableObj.ToArray());
            newObj.Deserialize(ref reader);
            return newObj;
        }

        public static bool EqualsTo(this StorageItem item, StorageItem other)
        {
            return item.Value.Span.SequenceEqual(other.Value.Span);
        }
    }
}
