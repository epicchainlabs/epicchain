// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_Wallet.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Cryptography.ECC;
using EpicChain.Network.P2P.Payloads;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.UnitTests.Cryptography;
using EpicChain.Wallets;
using System;
using System.Collections.Generic;
using System.Numerics;

namespace EpicChain.UnitTests.Wallets
{
    internal class MyWallet : Wallet
    {
        public override string Name => "MyWallet";

        public override Version Version => Version.Parse("0.0.1");

        private readonly Dictionary<UInt160, WalletAccount> accounts = new();

        public MyWallet() : base(null, TestProtocolSettings.Default)
        {
        }

        public override bool ChangePassword(string oldPassword, string newPassword)
        {
            throw new NotImplementedException();
        }

        public override bool Contains(UInt160 scriptHash)
        {
            return accounts.ContainsKey(scriptHash);
        }

        public void AddAccount(WalletAccount account)
        {
            accounts.Add(account.ScriptHash, account);
        }

        public override WalletAccount CreateAccount(byte[] privateKey)
        {
            KeyPair key = new(privateKey);
            var contract = new Contract
            {
                Script = Contract.CreateSignatureRedeemScript(key.PublicKey),
                ParameterList = new[] { ContractParameterType.Signature }
            };
            MyWalletAccount account = new(contract.ScriptHash);
            account.SetKey(key);
            account.Contract = contract;
            AddAccount(account);
            return account;
        }

        public override WalletAccount CreateAccount(Contract contract, KeyPair key = null)
        {
            MyWalletAccount account = new(contract.ScriptHash)
            {
                Contract = contract
            };
            account.SetKey(key);
            AddAccount(account);
            return account;
        }

        public override WalletAccount CreateAccount(UInt160 scriptHash)
        {
            MyWalletAccount account = new(scriptHash);
            AddAccount(account);
            return account;
        }

        public override void Delete()
        {
        }

        public override bool DeleteAccount(UInt160 scriptHash)
        {
            return accounts.Remove(scriptHash);
        }

        public override WalletAccount GetAccount(UInt160 scriptHash)
        {
            accounts.TryGetValue(scriptHash, out WalletAccount account);
            return account;
        }

        public override IEnumerable<WalletAccount> GetAccounts()
        {
            return accounts.Values;
        }

        public override bool VerifyPassword(string password)
        {
            return true;
        }

        public override void Save()
        {
        }
    }

    [TestClass]
    public class UT_Wallet
    {
        private static KeyPair glkey;
        private static string Xep2key;

        [ClassInitialize]
        public static void ClassInit(TestContext ctx)
        {
            glkey = UT_Crypto.GenerateCertainKey(32);
            Xep2key = glkey.Export("pwd", TestProtocolSettings.Default.AddressVersion, 2, 1, 1);
        }

        [TestMethod]
        public void TestContains()
        {
            MyWallet wallet = new();
            Action action = () => wallet.Contains(UInt160.Zero);
            action.Should().NotThrow();
        }

        [TestMethod]
        public void TestCreateAccount1()
        {
            MyWallet wallet = new();
            wallet.CreateAccount(new byte[32]).Should().NotBeNull();
        }

        [TestMethod]
        public void TestCreateAccount2()
        {
            MyWallet wallet = new();
            Contract contract = Contract.Create(new ContractParameterType[] { ContractParameterType.Boolean }, new byte[] { 1 });
            WalletAccount account = wallet.CreateAccount(contract, UT_Crypto.GenerateCertainKey(32).PrivateKey);
            account.Should().NotBeNull();

            wallet = new();
            account = wallet.CreateAccount(contract, (byte[])(null));
            account.Should().NotBeNull();
        }

        [TestMethod]
        public void TestCreateAccount3()
        {
            MyWallet wallet = new();
            Contract contract = Contract.Create(new ContractParameterType[] { ContractParameterType.Boolean }, new byte[] { 1 });
            wallet.CreateAccount(contract, glkey).Should().NotBeNull();
        }

        [TestMethod]
        public void TestCreateAccount4()
        {
            MyWallet wallet = new();
            wallet.CreateAccount(UInt160.Zero).Should().NotBeNull();
        }

        [TestMethod]
        public void TestGetName()
        {
            MyWallet wallet = new();
            wallet.Name.Should().Be("MyWallet");
        }

        [TestMethod]
        public void TestGetVersion()
        {
            MyWallet wallet = new();
            wallet.Version.Should().Be(Version.Parse("0.0.1"));
        }

        [TestMethod]
        public void TestGetAccount1()
        {
            MyWallet wallet = new();
            wallet.CreateAccount(UInt160.Parse("0x7efe7ee0d3e349e085388c351955e5172605de66"));
            WalletAccount account = wallet.GetAccount(ECCurve.Secp256r1.G);
            account.ScriptHash.Should().Be(UInt160.Parse("0x7efe7ee0d3e349e085388c351955e5172605de66"));
        }

        [TestMethod]
        public void TestGetAccount2()
        {
            MyWallet wallet = new();
            Action action = () => wallet.GetAccount(UInt160.Zero);
            action.Should().NotThrow();
        }

        [TestMethod]
        public void TestGetAccounts()
        {
            MyWallet wallet = new();
            Action action = () => wallet.GetAccounts();
            action.Should().NotThrow();
        }

        [TestMethod]
        public void TestGetAvailable()
        {
            MyWallet wallet = new();
            Contract contract = Contract.Create(new ContractParameterType[] { ContractParameterType.Boolean }, new byte[] { 1 });
            WalletAccount account = wallet.CreateAccount(contract, glkey.PrivateKey);
            account.Lock = false;

            // Fake balance
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            var key = NativeContract.EpicPulse.CreateStorageKey(20, account.ScriptHash);
            var entry = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));
            entry.GetInteroperable<AccountState>().Balance = 10000 * NativeContract.EpicPulse.Factor;

            wallet.GetAvailable(snapshotCache, NativeContract.EpicPulse.Hash).Should().Be(new BigDecimal(new BigInteger(1000000000000M), 8));

            entry = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));
            entry.GetInteroperable<AccountState>().Balance = 0;
        }

        [TestMethod]
        public void TestGetBalance()
        {
            MyWallet wallet = new();
            Contract contract = Contract.Create(new ContractParameterType[] { ContractParameterType.Boolean }, new byte[] { 1 });
            WalletAccount account = wallet.CreateAccount(contract, glkey.PrivateKey);
            account.Lock = false;

            // Fake balance
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            var key = NativeContract.EpicPulse.CreateStorageKey(20, account.ScriptHash);
            var entry = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));
            entry.GetInteroperable<AccountState>().Balance = 10000 * NativeContract.EpicPulse.Factor;

            wallet.GetBalance(snapshotCache, UInt160.Zero, new UInt160[] { account.ScriptHash }).Should().Be(new BigDecimal(BigInteger.Zero, 0));
            wallet.GetBalance(snapshotCache, NativeContract.EpicPulse.Hash, new UInt160[] { account.ScriptHash }).Should().Be(new BigDecimal(new BigInteger(1000000000000M), 8));

            entry = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));
            entry.GetInteroperable<AccountState>().Balance = 0;
        }

        [TestMethod]
        public void TestGetPrivateKeyFromXEP2()
        {
            Action action = () => Wallet.GetPrivateKeyFromXEP2("3vQB7B6MrGQZaxCuFg4oh", "TestGetPrivateKeyFromXEP2", ProtocolSettings.Default.AddressVersion, 2, 1, 1);
            action.Should().Throw<FormatException>();

            action = () => Wallet.GetPrivateKeyFromXEP2(Xep2key, "Test", ProtocolSettings.Default.AddressVersion, 2, 1, 1);
            action.Should().Throw<FormatException>();

            Wallet.GetPrivateKeyFromXEP2(Xep2key, "pwd", ProtocolSettings.Default.AddressVersion, 2, 1, 1).Should().BeEquivalentTo(new byte[] { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31 });
        }

        [TestMethod]
        public void TestGetPrivateKeyFromWIF()
        {
            Action action = () => Wallet.GetPrivateKeyFromWIF(null);
            action.Should().Throw<ArgumentNullException>();

            action = () => Wallet.GetPrivateKeyFromWIF("3vQB7B6MrGQZaxCuFg4oh");
            action.Should().Throw<FormatException>();

            Wallet.GetPrivateKeyFromWIF("L3tgppXLgdaeqSGSFw1Go3skBiy8vQAM7YMXvTHsKQtE16PBncSU").Should().BeEquivalentTo(new byte[] { 199, 19, 77, 111, 216, 231, 61, 129, 158, 130, 117, 92, 100, 201, 55, 136, 216, 219, 9, 97, 146, 158, 2, 90, 83, 54, 60, 76, 192, 42, 105, 98 });
        }

        [TestMethod]
        public void TestImport1()
        {
            MyWallet wallet = new();
            wallet.Import("L3tgppXLgdaeqSGSFw1Go3skBiy8vQAM7YMXvTHsKQtE16PBncSU").Should().NotBeNull();
        }

        [TestMethod]
        public void TestImport2()
        {
            MyWallet wallet = new();
            wallet.Import(Xep2key, "pwd", 2, 1, 1).Should().NotBeNull();
        }

        [TestMethod]
        public void TestMakeTransaction1()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            MyWallet wallet = new();
            Contract contract = Contract.Create(new ContractParameterType[] { ContractParameterType.Boolean }, new byte[] { 1 });
            WalletAccount account = wallet.CreateAccount(contract, glkey.PrivateKey);
            account.Lock = false;

            Action action = () => wallet.MakeTransaction(snapshotCache, new TransferOutput[]
            {
                new TransferOutput()
                {
                     AssetId = NativeContract.EpicPulse.Hash,
                     ScriptHash = account.ScriptHash,
                     Value = new BigDecimal(BigInteger.One,8),
                     Data = "Dec 12th"
                }
            }, UInt160.Zero);
            action.Should().Throw<InvalidOperationException>();

            action = () => wallet.MakeTransaction(snapshotCache, new TransferOutput[]
            {
                new TransferOutput()
                {
                     AssetId = NativeContract.EpicPulse.Hash,
                     ScriptHash = account.ScriptHash,
                     Value = new BigDecimal(BigInteger.One,8),
                     Data = "Dec 12th"
                }
            }, account.ScriptHash);
            action.Should().Throw<InvalidOperationException>();

            action = () => wallet.MakeTransaction(snapshotCache, new TransferOutput[]
            {
                new TransferOutput()
                {
                     AssetId = UInt160.Zero,
                     ScriptHash = account.ScriptHash,
                     Value = new BigDecimal(BigInteger.One,8),
                     Data = "Dec 12th"
                }
            }, account.ScriptHash);
            action.Should().Throw<InvalidOperationException>();

            // Fake balance
            var key = NativeContract.EpicPulse.CreateStorageKey(20, account.ScriptHash);
            var entry1 = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));
            entry1.GetInteroperable<AccountState>().Balance = 10000 * NativeContract.EpicPulse.Factor;

            key = NativeContract.EpicChain.CreateStorageKey(20, account.ScriptHash);
            var entry2 = snapshotCache.GetAndChange(key, () => new StorageItem(new EpicChain.EpicChainAccountState()));
            entry2.GetInteroperable<EpicChain.EpicChainAccountState>().Balance = 10000 * NativeContract.EpicChain.Factor;

            var tx = wallet.MakeTransaction(snapshotCache, new TransferOutput[]
            {
                new TransferOutput()
                {
                     AssetId = NativeContract.EpicPulse.Hash,
                     ScriptHash = account.ScriptHash,
                     Value = new BigDecimal(BigInteger.One,8)
                }
            });
            tx.Should().NotBeNull();

            tx = wallet.MakeTransaction(snapshotCache, new TransferOutput[]
            {
                new TransferOutput()
                {
                     AssetId = NativeContract.EpicChain.Hash,
                     ScriptHash = account.ScriptHash,
                     Value = new BigDecimal(BigInteger.One,8),
                     Data = "Dec 12th"
                }
            });
            tx.Should().NotBeNull();

            entry1 = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));
            entry2 = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));
            entry1.GetInteroperable<AccountState>().Balance = 0;
            entry2.GetInteroperable<EpicChain.EpicChainAccountState>().Balance = 0;
        }

        [TestMethod]
        public void TestMakeTransaction2()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            MyWallet wallet = new();
            Action action = () => wallet.MakeTransaction(snapshotCache, Array.Empty<byte>(), null, null, Array.Empty<TransactionAttribute>());
            action.Should().Throw<InvalidOperationException>();

            Contract contract = Contract.Create(new ContractParameterType[] { ContractParameterType.Boolean }, new byte[] { 1 });
            WalletAccount account = wallet.CreateAccount(contract, glkey.PrivateKey);
            account.Lock = false;

            // Fake balance
            var key = NativeContract.EpicPulse.CreateStorageKey(20, account.ScriptHash);
            var entry = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));
            entry.GetInteroperable<AccountState>().Balance = 1000000 * NativeContract.EpicPulse.Factor;

            var tx = wallet.MakeTransaction(snapshotCache, Array.Empty<byte>(), account.ScriptHash, new[]{ new Signer()
            {
                Account = account.ScriptHash,
                Scopes = WitnessScope.CalledByEntry
            }}, Array.Empty<TransactionAttribute>());

            tx.Should().NotBeNull();

            tx = wallet.MakeTransaction(snapshotCache, Array.Empty<byte>(), null, null, Array.Empty<TransactionAttribute>());
            tx.Should().NotBeNull();

            entry = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));
            entry.GetInteroperable<AccountState>().Balance = 0;
        }

        [TestMethod]
        public void TestVerifyPassword()
        {
            MyWallet wallet = new();
            Action action = () => wallet.VerifyPassword("Test");
            action.Should().NotThrow();
        }
    }
}
