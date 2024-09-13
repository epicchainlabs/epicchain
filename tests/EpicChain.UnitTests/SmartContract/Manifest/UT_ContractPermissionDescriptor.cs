// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_ContractPermissionDescriptor.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Json;
using EpicChain.SmartContract.Manifest;
using EpicChain.SmartContract.Native;
using EpicChain.Wallets;
using System;
using System.Security.Cryptography;

namespace EpicChain.UnitTests.SmartContract.Manifest
{
    [TestClass]
    public class UT_ContractPermissionDescriptor
    {
        [TestMethod]
        public void TestCreateByECPointAndIsWildcard()
        {
            byte[] privateKey = new byte[32];
            RandomNumberGenerator rng = RandomNumberGenerator.Create();
            rng.GetBytes(privateKey);
            KeyPair key = new KeyPair(privateKey);
            ContractPermissionDescriptor contractPermissionDescriptor = ContractPermissionDescriptor.Create(key.PublicKey);
            Assert.IsNotNull(contractPermissionDescriptor);
            Assert.AreEqual(key.PublicKey, contractPermissionDescriptor.Group);
            Assert.AreEqual(false, contractPermissionDescriptor.IsWildcard);
        }

        [TestMethod]
        public void TestContractPermissionDescriptorFromAndToJson()
        {
            byte[] privateKey = new byte[32];
            RandomNumberGenerator rng = RandomNumberGenerator.Create();
            rng.GetBytes(privateKey);
            KeyPair key = new KeyPair(privateKey);
            ContractPermissionDescriptor temp = ContractPermissionDescriptor.Create(key.PublicKey);
            ContractPermissionDescriptor result = ContractPermissionDescriptor.FromJson(temp.ToJson());
            Assert.AreEqual(null, result.Hash);
            Assert.AreEqual(result.Group, result.Group);
            Assert.ThrowsException<FormatException>(() => ContractPermissionDescriptor.FromJson(string.Empty));
        }

        [TestMethod]
        public void TestContractManifestFromJson()
        {
            Assert.ThrowsException<NullReferenceException>(() => ContractManifest.FromJson(new Json.JObject()));
            var jsonFiles = System.IO.Directory.GetFiles(System.IO.Path.Combine("SmartContract", "Manifest", "TestFile"));
            foreach (var item in jsonFiles)
            {
                var json = JObject.Parse(System.IO.File.ReadAllText(item)) as JObject;
                var manifest = ContractManifest.FromJson(json);
                manifest.ToJson().ToString().Should().Be(json.ToString());
            }
        }

        [TestMethod]
        public void TestEquals()
        {
            var descriptor1 = ContractPermissionDescriptor.CreateWildcard();
            var descriptor2 = ContractPermissionDescriptor.Create(QuantumVaultAsset.EpicChain.Hash);

            Assert.AreNotEqual(descriptor1, descriptor2);

            var descriptor3 = ContractPermissionDescriptor.Create(QuantumVaultAsset.EpicChain.Hash);

            Assert.AreEqual(descriptor2, descriptor3);
        }
    }
}
