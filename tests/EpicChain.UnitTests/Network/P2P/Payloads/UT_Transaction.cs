// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_Transaction.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Extensions;
using EpicChain.IO;
using EpicChain.Json;
using EpicChain.Ledger;
using EpicChain.Network.P2P.Payloads;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using EpicChain.Wallets;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;

namespace EpicChain.UnitTests.Network.P2P.Payloads
{
    [TestClass]
    public class UT_Transaction
    {
        Transaction uut;

        [TestInitialize]
        public void TestSetup()
        {
            uut = new Transaction();
        }

        [TestCleanup]
        public void Clean()
        {
            TestBlockchain.ResetStore();
        }

        [TestMethod]
        public void Script_Get()
        {
            uut.Script.IsEmpty.Should().BeTrue();
        }

        [TestMethod]
        public void FromStackItem()
        {
            Assert.ThrowsException<NotSupportedException>(() => ((IInteroperable)uut).FromStackItem(VM.Types.StackItem.Null));
        }

        [TestMethod]
        public void TestEquals()
        {
            Assert.IsTrue(uut.Equals(uut));
            Assert.IsFalse(uut.Equals(null));
        }

        [TestMethod]
        public void InventoryType_Get()
        {
            ((IInventory)uut).InventoryType.Should().Be(InventoryType.TX);
        }

        [TestMethod]
        public void Script_Set()
        {
            byte[] val = TestUtils.GetByteArray(32, 0x42);
            uut.Script = val;
            var span = uut.Script.Span;
            span.Length.Should().Be(32);
            for (int i = 0; i < val.Length; i++)
            {
                span[i].Should().Be(val[i]);
            }
        }

        [TestMethod]
        public void EpicPulse_Get()
        {
            uut.SystemFee.Should().Be(0);
        }

        [TestMethod]
        public void EpicPulse_Set()
        {
            long val = 4200000000;
            uut.SystemFee = val;
            uut.SystemFee.Should().Be(val);
        }

        [TestMethod]
        public void Size_Get()
        {
            uut.Script = TestUtils.GetByteArray(32, 0x42);
            uut.Signers = [];
            uut.Attributes = [];
            uut.Witnesses =
            [
                new Witness
                {
                    InvocationScript = Array.Empty<byte>(),
                    VerificationScript = Array.Empty<byte>()
                }
            ];

            uut.Version.Should().Be(0);
            uut.Script.Length.Should().Be(32);
            uut.Script.GetVarSize().Should().Be(33);
            uut.Size.Should().Be(63);
        }

        [TestMethod]
        public void FeeIsMultiSigContract()
        {
            var walletA = TestUtils.GenerateTestWallet("123");
            var walletB = TestUtils.GenerateTestWallet("123");
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();

            var a = walletA.CreateAccount();
            var b = walletB.CreateAccount();

            var multiSignContract = Contract.CreateMultiSigContract(2,
            [
                a.GetKey().PublicKey,
                b.GetKey().PublicKey
            ]);

            walletA.CreateAccount(multiSignContract, a.GetKey());
            var acc = walletB.CreateAccount(multiSignContract, b.GetKey());

            // Fake balance

            var key = NativeContract.EpicPulse.CreateStorageKey(20, acc.ScriptHash);
            var entry = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));

            entry.GetInteroperable<AccountState>().Balance = 10000 * NativeContract.EpicPulse.Factor;

            snapshotCache.Commit();

            // Make transaction

            var tx = walletA.MakeTransaction(snapshotCache, [
                new TransferOutput
                {
                    AssetId = NativeContract.EpicPulse.Hash,
                    ScriptHash = acc.ScriptHash,
                    Value = new BigDecimal(BigInteger.One, 8)
                }
            ], acc.ScriptHash);

            Assert.IsNotNull(tx);

            // Sign

            var wrongData = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network + 1);
            Assert.IsFalse(walletA.Sign(wrongData));

            var data = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network);
            Assert.IsTrue(walletA.Sign(data));
            Assert.IsTrue(walletB.Sign(data));
            Assert.IsTrue(data.Completed);

            tx.Witnesses = data.GetWitnesses();

            // Fast check

            Assert.IsTrue(tx.VerifyWitnesses(TestProtocolSettings.Default, snapshotCache, tx.NetworkFee));

            // Check

            long verificationEpicPulse = 0;
            foreach (var witness in tx.Witnesses)
            {
                using ApplicationEngine engine = ApplicationEngine.Create(TriggerType.Verification, tx, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings, epicpulse: tx.NetworkFee);
                engine.LoadScript(witness.VerificationScript);
                engine.LoadScript(witness.InvocationScript);
                Assert.AreEqual(VMState.HALT, engine.Execute());
                Assert.AreEqual(1, engine.ResultStack.Count);
                Assert.IsTrue(engine.ResultStack.Pop().GetBoolean());
                verificationEpicPulse += engine.FeeConsumed;
            }

            var sizeEpicPulse = tx.Size * NativeContract.Policy.GetFeePerByte(snapshotCache);
            Assert.AreEqual(1967100, verificationEpicPulse);
            Assert.AreEqual(348000, sizeEpicPulse);
            Assert.AreEqual(2315100, tx.NetworkFee);
        }

        [TestMethod]
        public void FeeIsSignatureContractDetailed()
        {
            var wallet = TestUtils.GenerateTestWallet("123");
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            var acc = wallet.CreateAccount();

            // Fake balance

            var key = NativeContract.EpicPulse.CreateStorageKey(20, acc.ScriptHash);

            var entry = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));

            entry.GetInteroperable<AccountState>().Balance = 10000 * NativeContract.EpicPulse.Factor;

            snapshotCache.Commit();

            // Make transaction

            // self-transfer of 1e-8 EpicPulse
            var tx = wallet.MakeTransaction(snapshotCache, [
                new TransferOutput
                {
                    AssetId = NativeContract.EpicPulse.Hash,
                    ScriptHash = acc.ScriptHash,
                    Value = new BigDecimal(BigInteger.One, 8)
                }
            ], acc.ScriptHash);

            Assert.IsNotNull(tx);
            Assert.IsNull(tx.Witnesses);

            // check pre-computed network fee (already guessing signature sizes)
            tx.NetworkFee.Should().Be(1228520L);

            // ----
            // Sign
            // ----

            var data = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network);
            // 'from' is always required as witness
            // if not included on cosigner with a scope, its scope should be considered 'CalledByEntry'
            data.ScriptHashes.Count.Should().Be(1);
            data.ScriptHashes[0].Should().BeEquivalentTo(acc.ScriptHash);
            // will sign tx
            bool signed = wallet.Sign(data);
            Assert.IsTrue(signed);
            // get witnesses from signed 'data'
            tx.Witnesses = data.GetWitnesses();
            tx.Witnesses.Length.Should().Be(1);

            // Fast check

            Assert.IsTrue(tx.VerifyWitnesses(TestProtocolSettings.Default, snapshotCache, tx.NetworkFee));

            // Check

            long verificationEpicPulse = 0;
            foreach (var witness in tx.Witnesses)
            {
                using var engine = ApplicationEngine.Create(TriggerType.Verification, tx, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings, epicpulse: tx.NetworkFee);
                engine.LoadScript(witness.VerificationScript);
                engine.LoadScript(witness.InvocationScript);
                Assert.AreEqual(VMState.HALT, engine.Execute());
                Assert.AreEqual(1, engine.ResultStack.Count);
                Assert.IsTrue(engine.ResultStack.Pop().GetBoolean());
                verificationEpicPulse += engine.FeeConsumed;
            }

            // ------------------
            // check tx_size cost
            // ------------------
            Assert.AreEqual(245, tx.Size);

            // will verify tx size, step by step

            // Part I
            Assert.AreEqual(25, Transaction.HeaderSize);
            // Part II
            Assert.AreEqual(1, tx.Attributes.GetVarSize());
            Assert.AreEqual(0, tx.Attributes.Length);
            Assert.AreEqual(1, tx.Signers.Length);
            // Note that Data size and Usage size are different (because of first byte on GetVarSize())
            Assert.AreEqual(22, tx.Signers.GetVarSize());
            // Part III
            Assert.AreEqual(88, tx.Script.GetVarSize());
            // Part IV
            Assert.AreEqual(109, tx.Witnesses.GetVarSize());
            // I + II + III + IV
            Assert.AreEqual(25 + 22 + 1 + 88 + 109, tx.Size);

            Assert.AreEqual(1000, NativeContract.Policy.GetFeePerByte(snapshotCache));
            var sizeEpicPulse = tx.Size * NativeContract.Policy.GetFeePerByte(snapshotCache);

            // final check: verification_cost and tx_size
            Assert.AreEqual(245000, sizeEpicPulse);
            Assert.AreEqual(983520, verificationEpicPulse);

            // final assert
            Assert.AreEqual(tx.NetworkFee, verificationEpicPulse + sizeEpicPulse);
        }

        [TestMethod]
        public void FeeIsSignatureContract_TestScope_Global()
        {
            var wallet = TestUtils.GenerateTestWallet("");
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            var acc = wallet.CreateAccount();

            // Fake balance

            var key = NativeContract.EpicPulse.CreateStorageKey(20, acc.ScriptHash);

            var entry = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));

            entry.GetInteroperable<AccountState>().Balance = 10000 * NativeContract.EpicPulse.Factor;

            snapshotCache.Commit();

            // Make transaction
            // Manually creating script

            byte[] script;
            using (ScriptBuilder sb = new())
            {
                // self-transfer of 1e-8 EpicPulse
                var value = new BigDecimal(BigInteger.One, 8).Value;
                sb.EmitDynamicCall(NativeContract.EpicPulse.Hash, "transfer", acc.ScriptHash, acc.ScriptHash, value, null);
                sb.Emit(OpCode.ASSERT);
                script = sb.ToArray();
            }

            // trying global scope
            var signers = new[]{ new Signer
                {
                    Account = acc.ScriptHash,
                    Scopes = WitnessScope.Global
                } };

            // using this...

            var tx = wallet.MakeTransaction(snapshotCache, script, acc.ScriptHash, signers);

            Assert.IsNotNull(tx);
            Assert.IsNull(tx.Witnesses);

            // ----
            // Sign
            // ----

            var data = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network);
            bool signed = wallet.Sign(data);
            Assert.IsTrue(signed);

            // get witnesses from signed 'data'
            tx.Witnesses = data.GetWitnesses();
            tx.Witnesses.Length.Should().Be(1);

            // Fast check
            Assert.IsTrue(tx.VerifyWitnesses(TestProtocolSettings.Default, snapshotCache, tx.NetworkFee));

            // Check
            long verificationEpicPulse = 0;
            foreach (var witness in tx.Witnesses)
            {
                using ApplicationEngine engine = ApplicationEngine.Create(TriggerType.Verification, tx, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings, epicpulse: tx.NetworkFee);
                engine.LoadScript(witness.VerificationScript);
                engine.LoadScript(witness.InvocationScript);
                Assert.AreEqual(VMState.HALT, engine.Execute());
                Assert.AreEqual(1, engine.ResultStack.Count);
                Assert.IsTrue(engine.ResultStack.Pop().GetBoolean());
                verificationEpicPulse += engine.FeeConsumed;
            }
            // get sizeEpicPulse
            var sizeEpicPulse = tx.Size * NativeContract.Policy.GetFeePerByte(snapshotCache);
            // final check on sum: verification_cost + tx_size
            Assert.AreEqual(1228520, verificationEpicPulse + sizeEpicPulse);
            // final assert
            Assert.AreEqual(tx.NetworkFee, verificationEpicPulse + sizeEpicPulse);
        }

        [TestMethod]
        public void FeeIsSignatureContract_TestScope_CurrentHash_EpicPulse()
        {
            var wallet = TestUtils.GenerateTestWallet("");
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            var acc = wallet.CreateAccount();

            // Fake balance

            var key = NativeContract.EpicPulse.CreateStorageKey(20, acc.ScriptHash);

            var entry = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));

            entry.GetInteroperable<AccountState>().Balance = 10000 * NativeContract.EpicPulse.Factor;

            snapshotCache.Commit();

            // Make transaction
            // Manually creating script

            byte[] script;
            using (ScriptBuilder sb = new())
            {
                // self-transfer of 1e-8 EpicPulse
                BigInteger value = new BigDecimal(BigInteger.One, 8).Value;
                sb.EmitDynamicCall(NativeContract.EpicPulse.Hash, "transfer", acc.ScriptHash, acc.ScriptHash, value, null);
                sb.Emit(OpCode.ASSERT);
                script = sb.ToArray();
            }

            // trying global scope
            var signers = new[]{ new Signer
                {
                    Account = acc.ScriptHash,
                    Scopes = WitnessScope.CustomContracts,
                    AllowedContracts = [NativeContract.EpicPulse.Hash]
                } };

            // using this...

            var tx = wallet.MakeTransaction(snapshotCache, script, acc.ScriptHash, signers);

            Assert.IsNotNull(tx);
            Assert.IsNull(tx.Witnesses);

            // ----
            // Sign
            // ----

            var data = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network);
            bool signed = wallet.Sign(data);
            Assert.IsTrue(signed);

            // get witnesses from signed 'data'
            tx.Witnesses = data.GetWitnesses();
            tx.Witnesses.Length.Should().Be(1);

            // Fast check
            Assert.IsTrue(tx.VerifyWitnesses(TestProtocolSettings.Default, snapshotCache, tx.NetworkFee));

            // Check
            long verificationEpicPulse = 0;
            foreach (var witness in tx.Witnesses)
            {
                using ApplicationEngine engine = ApplicationEngine.Create(TriggerType.Verification, tx, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings, epicpulse: tx.NetworkFee);
                engine.LoadScript(witness.VerificationScript);
                engine.LoadScript(witness.InvocationScript);
                Assert.AreEqual(VMState.HALT, engine.Execute());
                Assert.AreEqual(1, engine.ResultStack.Count);
                Assert.IsTrue(engine.ResultStack.Pop().GetBoolean());
                verificationEpicPulse += engine.FeeConsumed;
            }
            // get sizeEpicPulse
            var sizeEpicPulse = tx.Size * NativeContract.Policy.GetFeePerByte(snapshotCache);
            // final check on sum: verification_cost + tx_size
            Assert.AreEqual(1249520, verificationEpicPulse + sizeEpicPulse);
            // final assert
            Assert.AreEqual(tx.NetworkFee, verificationEpicPulse + sizeEpicPulse);
        }

        [TestMethod]
        public void FeeIsSignatureContract_TestScope_CalledByEntry_Plus_EpicPulse()
        {
            var wallet = TestUtils.GenerateTestWallet("");
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            var acc = wallet.CreateAccount();

            // Fake balance

            var key = NativeContract.EpicPulse.CreateStorageKey(20, acc.ScriptHash);

            var entry = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));

            entry.GetInteroperable<AccountState>().Balance = 10000 * NativeContract.EpicPulse.Factor;

            snapshotCache.Commit();

            // Make transaction
            // Manually creating script

            byte[] script;
            using (ScriptBuilder sb = new())
            {
                // self-transfer of 1e-8 GAEpicPulseS
                var value = new BigDecimal(BigInteger.One, 8).Value;
                sb.EmitDynamicCall(NativeContract.EpicPulse.Hash, "transfer", acc.ScriptHash, acc.ScriptHash, value, null);
                sb.Emit(OpCode.ASSERT);
                script = sb.ToArray();
            }

            // trying CalledByEntry together with EpicPulse
            var signers = new[]{ new Signer
                {
                    Account = acc.ScriptHash,
                    // This combination is supposed to actually be an OR,
                    // where it's valid in both Entry and also for Custom hash provided (in any execution level)
                    // it would be better to test this in the future including situations where a deeper call level uses this custom witness successfully
                    Scopes = WitnessScope.CustomContracts | WitnessScope.CalledByEntry,
                    AllowedContracts = [NativeContract.EpicPulse.Hash]
                } };

            // using this...

            var tx = wallet.MakeTransaction(snapshotCache, script, acc.ScriptHash, signers);

            Assert.IsNotNull(tx);
            Assert.IsNull(tx.Witnesses);

            // ----
            // Sign
            // ----

            var data = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network);
            bool signed = wallet.Sign(data);
            Assert.IsTrue(signed);

            // get witnesses from signed 'data'
            tx.Witnesses = data.GetWitnesses();
            tx.Witnesses.Length.Should().Be(1);

            // Fast check
            Assert.IsTrue(tx.VerifyWitnesses(TestProtocolSettings.Default, snapshotCache, tx.NetworkFee));

            // Check
            long verificationEpicPulse = 0;
            foreach (var witness in tx.Witnesses)
            {
                using ApplicationEngine engine = ApplicationEngine.Create(TriggerType.Verification, tx, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings, epicpulse: tx.NetworkFee);
                engine.LoadScript(witness.VerificationScript);
                engine.LoadScript(witness.InvocationScript);
                Assert.AreEqual(VMState.HALT, engine.Execute());
                Assert.AreEqual(1, engine.ResultStack.Count);
                Assert.IsTrue(engine.ResultStack.Pop().GetBoolean());
                verificationEpicPulse += engine.FeeConsumed;
            }
            // get sizeEpicPulse
            var sizeEpicPulse = tx.Size * NativeContract.Policy.GetFeePerByte(snapshotCache);
            // final check on sum: verification_cost + tx_size
            Assert.AreEqual(1249520, verificationEpicPulse + sizeEpicPulse);
            // final assert
            Assert.AreEqual(tx.NetworkFee, verificationEpicPulse + sizeEpicPulse);
        }

        [TestMethod]
        public void FeeIsSignatureContract_TestScope_CurrentHash_EpicChain_FAULT()
        {
            var wallet = TestUtils.GenerateTestWallet("");
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            var acc = wallet.CreateAccount();

            // Fake balance

            var key = NativeContract.EpicPulse.CreateStorageKey(20, acc.ScriptHash);

            var entry = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));

            entry.GetInteroperable<AccountState>().Balance = 10000 * NativeContract.EpicPulse.Factor;

            // Make transaction
            // Manually creating script

            byte[] script;
            using (ScriptBuilder sb = new())
            {
                // self-transfer of 1e-8 EpicPulse
                BigInteger value = new BigDecimal(BigInteger.One, 8).Value;
                sb.EmitDynamicCall(NativeContract.EpicPulse.Hash, "transfer", acc.ScriptHash, acc.ScriptHash, value);
                sb.Emit(OpCode.ASSERT);
                script = sb.ToArray();
            }

            // trying global scope
            var signers = new[]{ new Signer
                {
                    Account = acc.ScriptHash,
                    Scopes = WitnessScope.CustomContracts,
                    AllowedContracts = [NativeContract.EpicChain.Hash]
                } };

            // using this...

            // expects FAULT on execution of 'transfer' Application script
            // due to lack of a valid witness validation
            Transaction tx = null;
            Assert.ThrowsException<InvalidOperationException>(() => tx = wallet.MakeTransaction(snapshotCache, script, acc.ScriptHash, signers));
            Assert.IsNull(tx);
        }

        [TestMethod]
        public void FeeIsSignatureContract_TestScope_CurrentHash_EpicChain_EpicPulse()
        {
            var wallet = TestUtils.GenerateTestWallet("");
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            var acc = wallet.CreateAccount();

            // Fake balance

            var key = NativeContract.EpicPulse.CreateStorageKey(20, acc.ScriptHash);

            var entry = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));

            entry.GetInteroperable<AccountState>().Balance = 10000 * NativeContract.EpicPulse.Factor;

            snapshotCache.Commit();

            // Make transaction
            // Manually creating script

            byte[] script;
            using (ScriptBuilder sb = new())
            {
                // self-transfer of 1e-8 EpicPulse
                BigInteger value = new BigDecimal(BigInteger.One, 8).Value;
                sb.EmitDynamicCall(NativeContract.EpicPulse.Hash, "transfer", acc.ScriptHash, acc.ScriptHash, value, null);
                sb.Emit(OpCode.ASSERT);
                script = sb.ToArray();
            }

            // trying two custom hashes, for same target account
            var signers = new[]{ new Signer
                {
                    Account = acc.ScriptHash,
                    Scopes = WitnessScope.CustomContracts,
                    AllowedContracts = [NativeContract.EpicChain.Hash, NativeContract.EpicPulse.Hash]
                } };

            // using this...

            var tx = wallet.MakeTransaction(snapshotCache, script, acc.ScriptHash, signers);

            Assert.IsNotNull(tx);
            Assert.IsNull(tx.Witnesses);

            // ----
            // Sign
            // ----

            var data = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network);
            bool signed = wallet.Sign(data);
            Assert.IsTrue(signed);

            // get witnesses from signed 'data'
            tx.Witnesses = data.GetWitnesses();
            // only a single witness should exist
            tx.Witnesses.Length.Should().Be(1);
            // no attributes must exist
            tx.Attributes.Length.Should().Be(0);
            // one cosigner must exist
            tx.Signers.Length.Should().Be(1);

            // Fast check
            Assert.IsTrue(tx.VerifyWitnesses(TestProtocolSettings.Default, snapshotCache, tx.NetworkFee));

            // Check
            long verificationEpicPulse = 0;
            foreach (var witness in tx.Witnesses)
            {
                using ApplicationEngine engine = ApplicationEngine.Create(TriggerType.Verification, tx, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings, epicpulse: tx.NetworkFee);
                engine.LoadScript(witness.VerificationScript);
                engine.LoadScript(witness.InvocationScript);
                Assert.AreEqual(VMState.HALT, engine.Execute());
                Assert.AreEqual(1, engine.ResultStack.Count);
                Assert.IsTrue(engine.ResultStack.Pop().GetBoolean());
                verificationEpicPulse += engine.FeeConsumed;
            }
            // get sizeEpicPulse
            var sizeEpicPulse = tx.Size * NativeContract.Policy.GetFeePerByte(snapshotCache);
            // final check on sum: verification_cost + tx_size
            Assert.AreEqual(1269520, verificationEpicPulse + sizeEpicPulse);
            // final assert
            Assert.AreEqual(tx.NetworkFee, verificationEpicPulse + sizeEpicPulse);
        }

        [TestMethod]
        public void FeeIsSignatureContract_TestScope_NoScopeFAULT()
        {
            var wallet = TestUtils.GenerateTestWallet("");
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            var acc = wallet.CreateAccount();

            // Fake balance

            var key = NativeContract.EpicPulse.CreateStorageKey(20, acc.ScriptHash);

            var entry = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));

            entry.GetInteroperable<AccountState>().Balance = 10000 * NativeContract.EpicPulse.Factor;

            // Make transaction
            // Manually creating script

            byte[] script;
            using (ScriptBuilder sb = new())
            {
                // self-transfer of 1e-8 EpicPulse
                BigInteger value = new BigDecimal(BigInteger.One, 8).Value;
                sb.EmitDynamicCall(NativeContract.EpicPulse.Hash, "transfer", acc.ScriptHash, acc.ScriptHash, value);
                sb.Emit(OpCode.ASSERT);
                script = sb.ToArray();
            }

            // trying with no scope
            var attributes = Array.Empty<TransactionAttribute>();
            var signers = new[]{ new Signer
                {
                    Account = acc.ScriptHash,
                    Scopes = WitnessScope.CustomContracts,
                    AllowedContracts = [NativeContract.EpicChain.Hash, NativeContract.EpicPulse.Hash]
                } };

            // using this...

            // expects FAULT on execution of 'transfer' Application script
            // due to lack of a valid witness validation
            Transaction tx = null;
            Assert.ThrowsException<InvalidOperationException>(() => tx = wallet.MakeTransaction(snapshotCache, script, acc.ScriptHash, signers, attributes));
            Assert.IsNull(tx);
        }

        [TestMethod]
        public void FeeIsSignatureContract_UnexistingVerificationContractFAULT()
        {
            var wallet = TestUtils.GenerateTestWallet("");
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            var acc = wallet.CreateAccount();

            // Fake balance

            var key = NativeContract.EpicPulse.CreateStorageKey(20, acc.ScriptHash);

            var entry = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));

            entry.GetInteroperable<AccountState>().Balance = 10000 * NativeContract.EpicPulse.Factor;

            snapshotCache.Commit();

            // Make transaction
            // Manually creating script

            byte[] script;
            using (ScriptBuilder sb = new())
            {
                // self-transfer of 1e-8 EpicPulse
                BigInteger value = new BigDecimal(BigInteger.One, 8).Value;
                sb.EmitDynamicCall(NativeContract.EpicPulse.Hash, "transfer", acc.ScriptHash, acc.ScriptHash, value, null);
                sb.Emit(OpCode.ASSERT);
                script = sb.ToArray();
            }

            // trying global scope
            var signers = new Signer[]{ new Signer
                {
                    Account = acc.ScriptHash,
                    Scopes = WitnessScope.Global
                } };

            // creating new wallet with missing account for test
            var walletWithoutAcc = TestUtils.GenerateTestWallet("");

            // using this...

            Transaction tx = null;
            // expects ArgumentException on execution of 'CalculateNetworkFee' due to
            // null witness_script (no account in the wallet, no corresponding witness
            // and no verification contract for the signer)
            Assert.ThrowsException<ArgumentException>(() => walletWithoutAcc.MakeTransaction(snapshotCache, script, acc.ScriptHash, signers));
            Assert.IsNull(tx);
        }

        [TestMethod]
        public void Transaction_Reverify_Hashes_Length_Unequal_To_Witnesses_Length()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            Transaction txSimple = new()
            {
                Version = 0x00,
                Nonce = 0x01020304,
                SystemFee = (long)BigInteger.Pow(10, 8), // 1 EpicPulse
                NetworkFee = 0x0000000000000001,
                ValidUntilBlock = 0x01020304,
                Attributes = Array.Empty<TransactionAttribute>(),
                Signers = new[]{
                    new Signer
                    {
                        Account = UInt160.Parse("0x0001020304050607080900010203040506070809"),
                        Scopes = WitnessScope.Global
                    }
                },
                Script = new byte[] { (byte)OpCode.PUSH1 },
                Witnesses = Array.Empty<Witness>()
            };
            UInt160[] hashes = txSimple.GetScriptHashesForVerifying(snapshotCache);
            Assert.AreEqual(1, hashes.Length);
            Assert.AreNotEqual(VerifyResult.Succeed, txSimple.VerifyStateDependent(TestProtocolSettings.Default, snapshotCache, new TransactionVerificationContext(), new List<Transaction>()));
        }

        [TestMethod]
        public void Transaction_Serialize_Deserialize_Simple()
        {
            // good and simple transaction
            Transaction txSimple = new()
            {
                Version = 0x00,
                Nonce = 0x01020304,
                SystemFee = (long)BigInteger.Pow(10, 8), // 1 EpicPulse
                NetworkFee = 0x0000000000000001,
                ValidUntilBlock = 0x01020304,
                Signers = [new Signer() { Account = UInt160.Zero }],
                Attributes = [],
                Script = new[] { (byte)OpCode.PUSH1 },
                Witnesses = [new Witness { InvocationScript = Array.Empty<byte>(), VerificationScript = Array.Empty<byte>() }
                ]
            };

            byte[] sTx = txSimple.ToArray();

            // detailed hexstring info (basic checking)
            sTx.ToHexString().Should().Be(
                "00" + // version
                "04030201" + // nonce
                "00e1f50500000000" + // system fee (1 EpicPulse)
                "0100000000000000" + // network fee (1 datoshi)
                "04030201" + // timelimit
                "01000000000000000000000000000000000000000000" + // empty signer
                "00" + // no attributes
                "0111" + // push1 script
                "010000"); // empty witnesses

            // try to deserialize
            var tx2 = sTx.AsSerializable<Transaction>();

            tx2.Version.Should().Be(0x00);
            tx2.Nonce.Should().Be(0x01020304);
            tx2.Sender.Should().Be(UInt160.Zero);
            tx2.SystemFee.Should().Be(0x0000000005f5e100); // 1 EpicPulse (long)BigInteger.Pow(10, 8)
            tx2.NetworkFee.Should().Be(0x0000000000000001);
            tx2.ValidUntilBlock.Should().Be(0x01020304);
            tx2.Attributes.Should().BeEquivalentTo(Array.Empty<TransactionAttribute>());
            tx2.Signers.Should().BeEquivalentTo([
                new Signer
                {
                    Account = UInt160.Zero,
                    AllowedContracts = [],
                    AllowedGroups = [],
                    Rules = []
                }
            ]);
            tx2.Script.Span.SequenceEqual([(byte)OpCode.PUSH1]).Should().BeTrue();
            tx2.Witnesses[0].InvocationScript.Span.IsEmpty.Should().BeTrue();
            tx2.Witnesses[0].VerificationScript.Span.IsEmpty.Should().BeTrue();
        }

        [TestMethod]
        public void Transaction_Serialize_Deserialize_DistinctCosigners()
        {
            // cosigners must be distinct (regarding account)

            Transaction txDoubleCosigners = new()
            {
                Version = 0x00,
                Nonce = 0x01020304,
                SystemFee = (long)BigInteger.Pow(10, 8), // 1 EpicPulse
                NetworkFee = 0x0000000000000001,
                ValidUntilBlock = 0x01020304,
                Attributes = [],
                Signers =
                [
                    new Signer
                    {
                        Account = UInt160.Parse("0x0001020304050607080900010203040506070809"),
                        Scopes = WitnessScope.Global
                    },
                    new Signer
                    {
                        Account = UInt160.Parse("0x0001020304050607080900010203040506070809"), // same account as above
                        Scopes = WitnessScope.CalledByEntry // different scope, but still, same account (cannot do that)
                    }
                ],
                Script = new[] { (byte)OpCode.PUSH1 },
                Witnesses = [new Witness { InvocationScript = Array.Empty<byte>(), VerificationScript = Array.Empty<byte>() }
                ]
            };

            var sTx = txDoubleCosigners.ToArray();

            // no need for detailed hexstring here (see basic tests for it)
            sTx.ToHexString().Should().Be("000403020100e1f5050000000001000000000000000403020102090807060504030201000908070605040302010080090807060504030201000908070605040302010001000111010000");

            // back to transaction (should fail, due to non-distinct cosigners)
            Transaction tx2 = null;
            Assert.ThrowsException<FormatException>(() =>
                tx2 = EpicChain.IO.Helper.AsSerializable<Transaction>(sTx)
            );
            Assert.IsNull(tx2);
        }


        [TestMethod]
        public void Transaction_Serialize_Deserialize_MaxSizeCosigners()
        {
            // cosigners must respect count

            int maxCosigners = 16;

            // --------------------------------------
            // this should pass (respecting max size)

            var cosigners1 = new Signer[maxCosigners];
            for (int i = 0; i < cosigners1.Length; i++)
            {
                string hex = i.ToString("X4");
                while (hex.Length < 40)
                    hex = hex.Insert(0, "0");
                cosigners1[i] = new Signer
                {
                    Account = UInt160.Parse(hex),
                    Scopes = WitnessScope.CalledByEntry
                };
            }

            Transaction txCosigners1 = new()
            {
                Version = 0x00,
                Nonce = 0x01020304,
                SystemFee = (long)BigInteger.Pow(10, 8), // 1 EpicPulse
                NetworkFee = 0x0000000000000001,
                ValidUntilBlock = 0x01020304,
                Attributes = [],
                Signers = cosigners1, // max + 1 (should fail)
                Script = new[] { (byte)OpCode.PUSH1 },
                Witnesses = [new Witness { InvocationScript = Array.Empty<byte>(), VerificationScript = Array.Empty<byte>() }
                ]
            };

            byte[] sTx1 = txCosigners1.ToArray();

            // back to transaction (should fail, due to non-distinct cosigners)
            Assert.ThrowsException<FormatException>(() => sTx1.AsSerializable<Transaction>());

            // ----------------------------
            // this should fail (max + 1)

            var cosigners = new Signer[maxCosigners + 1];
            for (var i = 0; i < maxCosigners + 1; i++)
            {
                var hex = i.ToString("X4");
                while (hex.Length < 40)
                    hex = hex.Insert(0, "0");
                cosigners[i] = new Signer
                {
                    Account = UInt160.Parse(hex)
                };
            }

            Transaction txCosigners = new()
            {
                Version = 0x00,
                Nonce = 0x01020304,
                SystemFee = (long)BigInteger.Pow(10, 8), // 1 EpicPulse
                NetworkFee = 0x0000000000000001,
                ValidUntilBlock = 0x01020304,
                Attributes = [],
                Signers = cosigners, // max + 1 (should fail)
                Script = new[] { (byte)OpCode.PUSH1 },
                Witnesses = [new Witness { InvocationScript = Array.Empty<byte>(), VerificationScript = Array.Empty<byte>() }
                ]
            };

            byte[] sTx2 = txCosigners.ToArray();

            // back to transaction (should fail, due to non-distinct cosigners)
            Transaction tx2 = null;
            Assert.ThrowsException<FormatException>(() =>
                tx2 = sTx2.AsSerializable<Transaction>()
            );
            Assert.IsNull(tx2);
        }

        [TestMethod]
        public void FeeIsSignatureContract_TestScope_FeeOnly_Default()
        {
            // Global is supposed to be default

            Signer cosigner = new();
            cosigner.Scopes.Should().Be(WitnessScope.None);

            var wallet = TestUtils.GenerateTestWallet("");
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            var acc = wallet.CreateAccount();

            // Fake balance

            var key = NativeContract.EpicPulse.CreateStorageKey(20, acc.ScriptHash);

            var entry = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));

            entry.GetInteroperable<AccountState>().Balance = 10000 * NativeContract.EpicPulse.Factor;

            snapshotCache.Commit();

            // Make transaction
            // Manually creating script

            byte[] script;
            using (ScriptBuilder sb = new())
            {
                // self-transfer of 1e-8 EpicPulse
                BigInteger value = new BigDecimal(BigInteger.One, 8).Value;
                sb.EmitDynamicCall(NativeContract.EpicPulse.Hash, "transfer", acc.ScriptHash, acc.ScriptHash, value, null);
                sb.Emit(OpCode.ASSERT);
                script = sb.ToArray();
            }

            // try to use fee only inside the smart contract
            var signers = new[]{ new Signer
                {
                    Account = acc.ScriptHash,
                    Scopes =  WitnessScope.None
                } };

            Assert.ThrowsException<InvalidOperationException>(() => wallet.MakeTransaction(snapshotCache, script, acc.ScriptHash, signers));

            // change to global scope
            signers[0].Scopes = WitnessScope.Global;

            var tx = wallet.MakeTransaction(snapshotCache, script, acc.ScriptHash, signers);

            Assert.IsNotNull(tx);
            Assert.IsNull(tx.Witnesses);

            // ----
            // Sign
            // ----

            var data = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network);
            bool signed = wallet.Sign(data);
            Assert.IsTrue(signed);

            // get witnesses from signed 'data'
            tx.Witnesses = data.GetWitnesses();
            tx.Witnesses.Length.Should().Be(1);

            // Fast check
            Assert.IsTrue(tx.VerifyWitnesses(TestProtocolSettings.Default, snapshotCache, tx.NetworkFee));

            // Check
            long verificationEpicPulse = 0;
            foreach (var witness in tx.Witnesses)
            {
                using ApplicationEngine engine = ApplicationEngine.Create(TriggerType.Verification, tx, snapshotCache, settings: TestBlockchain.TheEpicChainSystem.Settings, epicpulse: tx.NetworkFee);
                engine.LoadScript(witness.VerificationScript);
                engine.LoadScript(witness.InvocationScript);
                Assert.AreEqual(VMState.HALT, engine.Execute());
                Assert.AreEqual(1, engine.ResultStack.Count);
                Assert.IsTrue(engine.ResultStack.Pop().GetBoolean());
                verificationEpicPulse += engine.FeeConsumed;
            }
            // get sizeEpicPulse
            var sizeEpicPulse = tx.Size * NativeContract.Policy.GetFeePerByte(snapshotCache);
            // final check on sum: verification_cost + tx_size
            Assert.AreEqual(1228520, verificationEpicPulse + sizeEpicPulse);
            // final assert
            Assert.AreEqual(tx.NetworkFee, verificationEpicPulse + sizeEpicPulse);
        }

        [TestMethod]
        public void ToJson()
        {
            uut.Script = TestUtils.GetByteArray(32, 0x42);
            uut.SystemFee = 4200000000;
            uut.Signers = [new Signer { Account = UInt160.Zero }];
            uut.Attributes = [];
            uut.Witnesses =
            [
                new Witness
                {
                    InvocationScript = Array.Empty<byte>(),
                    VerificationScript = Array.Empty<byte>()
                }
            ];

            JObject jObj = uut.ToJson(ProtocolSettings.Default);
            jObj.Should().NotBeNull();
            jObj["hash"].AsString().Should().Be("0x0ab073429086d9e48fc87386122917989705d1c81fe4a60bf90e2fc228de3146");
            jObj["size"].AsNumber().Should().Be(84);
            jObj["version"].AsNumber().Should().Be(0);
            ((JArray)jObj["attributes"]).Count.Should().Be(0);
            jObj["netfee"].AsString().Should().Be("0");
            jObj["script"].AsString().Should().Be("QiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICA=");
            jObj["sysfee"].AsString().Should().Be("4200000000");
        }

        [TestMethod]
        public void Test_GetAttribute()
        {
            var tx = new Transaction
            {
                Attributes = [],
                NetworkFee = 0,
                Nonce = (uint)Environment.TickCount,
                Script = new byte[Transaction.MaxTransactionSize],
                Signers = [new Signer { Account = UInt160.Zero }],
                SystemFee = 0,
                ValidUntilBlock = 0,
                Version = 0,
                Witnesses = [],
            };

            Assert.IsNull(tx.GetAttribute<OracleResponse>());
            Assert.IsNull(tx.GetAttribute<HighPriorityAttribute>());

            tx.Attributes = [new HighPriorityAttribute()];

            Assert.IsNull(tx.GetAttribute<OracleResponse>());
            Assert.IsNotNull(tx.GetAttribute<HighPriorityAttribute>());
        }

        [TestMethod]
        public void Test_VerifyStateIndependent()
        {
            var tx = new Transaction
            {
                Attributes = [],
                NetworkFee = 0,
                Nonce = (uint)Environment.TickCount,
                Script = new byte[Transaction.MaxTransactionSize],
                Signers = [new Signer { Account = UInt160.Zero }],
                SystemFee = 0,
                ValidUntilBlock = 0,
                Version = 0,
                Witnesses =
                [
                    new Witness
                    {
                        InvocationScript = Array.Empty<byte>(),
                        VerificationScript = Array.Empty<byte>()
                    }
                ]
            };
            tx.VerifyStateIndependent(TestProtocolSettings.Default).Should().Be(VerifyResult.OverSize);
            tx.Script = Array.Empty<byte>();
            tx.VerifyStateIndependent(TestProtocolSettings.Default).Should().Be(VerifyResult.Succeed);

            var walletA = TestUtils.GenerateTestWallet("123");
            var walletB = TestUtils.GenerateTestWallet("123");
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();

            var a = walletA.CreateAccount();
            var b = walletB.CreateAccount();

            var multiSignContract = Contract.CreateMultiSigContract(2,
            [
                a.GetKey().PublicKey,
                b.GetKey().PublicKey
            ]);

            walletA.CreateAccount(multiSignContract, a.GetKey());
            var acc = walletB.CreateAccount(multiSignContract, b.GetKey());

            // Fake balance

            var key = NativeContract.EpicPulse.CreateStorageKey(20, acc.ScriptHash);
            var entry = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));

            entry.GetInteroperable<AccountState>().Balance = 10000 * NativeContract.EpicPulse.Factor;

            snapshotCache.Commit();

            // Make transaction

            tx = walletA.MakeTransaction(snapshotCache, [
                new TransferOutput
                {
                    AssetId = NativeContract.EpicPulse.Hash,
                    ScriptHash = acc.ScriptHash,
                    Value = new BigDecimal(BigInteger.One, 8)
                }
            ], acc.ScriptHash);

            // Sign

            var data = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network);
            Assert.IsTrue(walletA.Sign(data));
            Assert.IsTrue(walletB.Sign(data));
            Assert.IsTrue(data.Completed);

            tx.Witnesses = data.GetWitnesses();
            tx.VerifyStateIndependent(TestProtocolSettings.Default).Should().Be(VerifyResult.Succeed);

            // Different hash

            tx.Witnesses[0] = new Witness()
            {
                VerificationScript = walletB.GetAccounts().First().Contract.Script,
                InvocationScript = tx.Witnesses[0].InvocationScript.ToArray()
            };
            tx.VerifyStateIndependent(TestProtocolSettings.Default).Should().Be(VerifyResult.Invalid);
        }

        [TestMethod]
        public void Test_VerifyStateDependent()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();
            var height = NativeContract.Ledger.CurrentIndex(snapshotCache);
            var tx = new Transaction()
            {
                Attributes = [],
                NetworkFee = 55000,
                Nonce = (uint)Environment.TickCount,
                Script = Array.Empty<byte>(),
                Signers = [new Signer() { Account = UInt160.Zero }],
                SystemFee = 0,
                ValidUntilBlock = height + 1,
                Version = 0,
                Witnesses =
                [
                    new Witness { InvocationScript = Array.Empty<byte>(), VerificationScript = Array.Empty<byte>() },
                    new Witness { InvocationScript = Array.Empty<byte>(), VerificationScript = new byte[1] }
                ]
            };

            // Fake balance

            var key = NativeContract.EpicPulse.CreateStorageKey(20, tx.Sender);
            var balance = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));
            balance.GetInteroperable<AccountState>().Balance = tx.NetworkFee;
            var conflicts = new List<Transaction>();

            tx.VerifyStateDependent(ProtocolSettings.Default, snapshotCache, new TransactionVerificationContext(), conflicts).Should().Be(VerifyResult.Invalid);
            balance.GetInteroperable<AccountState>().Balance = 0;
            tx.SystemFee = 10;
            tx.VerifyStateDependent(ProtocolSettings.Default, snapshotCache, new TransactionVerificationContext(), conflicts).Should().Be(VerifyResult.InsufficientFunds);

            var walletA = TestUtils.GenerateTestWallet("123");
            var walletB = TestUtils.GenerateTestWallet("123");

            var a = walletA.CreateAccount();
            var b = walletB.CreateAccount();

            var multiSignContract = Contract.CreateMultiSigContract(2,
            [
                a.GetKey().PublicKey,
                b.GetKey().PublicKey
            ]);

            walletA.CreateAccount(multiSignContract, a.GetKey());
            var acc = walletB.CreateAccount(multiSignContract, b.GetKey());

            // Fake balance

            snapshotCache = TestBlockchain.GetTestSnapshotCache();
            key = NativeContract.EpicPulse.CreateStorageKey(20, acc.ScriptHash);
            balance = snapshotCache.GetAndChange(key, () => new StorageItem(new AccountState()));
            balance.GetInteroperable<AccountState>().Balance = 10000 * NativeContract.EpicPulse.Factor;

            // Make transaction

            snapshotCache.Commit();
            tx = walletA.MakeTransaction(snapshotCache, new[]
            {
                    new TransferOutput()
                    {
                         AssetId = NativeContract.EpicPulse.Hash,
                         ScriptHash = acc.ScriptHash,
                         Value = new BigDecimal(BigInteger.One,8)
                    }
            }, acc.ScriptHash);

            // Sign

            var data = new ContractParametersContext(snapshotCache, tx, TestProtocolSettings.Default.Network);
            Assert.IsTrue(walletA.Sign(data));
            Assert.IsTrue(walletB.Sign(data));
            Assert.IsTrue(data.Completed);

            tx.Witnesses = data.GetWitnesses();
            tx.VerifyStateDependent(TestProtocolSettings.Default, snapshotCache, new TransactionVerificationContext(), new List<Transaction>()).Should().Be(VerifyResult.Succeed);
        }

        [TestMethod]
        public void Test_VerifyStateInDependent_Multi()
        {
            var txData = Convert.FromBase64String(
                "AHXd31W0NlsAAAAAAJRGawAAAAAA3g8CAAGSs5x3qmDym1fBc87ZF/F/0yGm6wEAXwsDAOQLVAIAAAAMFLqZBJj+L0XZPXNHHM9MBfCza5HnDBSSs5x3qmDym1fBc87ZF/F/0yGm6xTAHwwIdHJhbnNmZXIMFM924ovQBixKR47jVWEBExnzz6TSQWJ9W1I5Af1KAQxAnZvOQOCdkM+j22dS5SdEncZVYVVi1F26MhheNzNImTD4Ekw5kFR6Fojs7gD57Bdeuo8tLS1UXpzflmKcQ3pniAxAYvGgxtokrk6PVdduxCBwVbdfie+ZxiaDsjK0FYregl24cDr2v5cTLHrURVfJJ1is+4G6Jaer7nB1JrDrw+Qt6QxATA5GdR4rKFPPPQQ24+42OP2tz0HylG1LlANiOtIdag3ZPkUfZiBfEGoOteRD1O0UnMdJP4Su7PFhDuCdHu4MlwxAuGFEk2m/rdruleBGYz8DIzExJtwb/TsFxZdHxo4VV8ktv2Nh71Fwhg2bhW2tq8hV6RK2GFXNAU72KAgf/Qv6BQxA0j3srkwY333KvGNtw7ZvSG8X36Tqu000CEtDx4SMOt8qhVYGMr9PClsUVcYFHdrJaodilx8ewXDHNIq+OnS7SfwVDCEDAJt1QOEPJWLl/Y+snq7CUWaliybkEjSP9ahpJ7+sIqIMIQMCBenO+upaHfxYCvIMjVqiRouwFI8aXkYF/GIsgOYEugwhAhS68M7qOmbxfn4eg56iX9i+1s2C5rtuaCUBiQZfRP8BDCECPpsy6om5TQZuZJsST9UOOW7pE2no4qauGxHBcNAiJW0MIQNAjc1BY5b2R4OsWH6h4Vk8V9n+qIDIpqGSDpKiWUd4BgwhAqeDS+mzLimB0VfLW706y0LP0R6lw7ECJNekTpjFkQ8bDCECuixw9ZlvNXpDGYcFhZ+uLP6hPhFyligAdys9WIqdSr0XQZ7Q3Do=");

            var tx = new Transaction();
            MemoryReader reader = new(txData);
            ((ISerializable)tx).Deserialize(ref reader);

            var settings = new ProtocolSettings() { Network = 844378958 };
            var result = tx.VerifyStateIndependent(settings);
            Assert.AreEqual(VerifyResult.Succeed, result);
        }
    }
}
