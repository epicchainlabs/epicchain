// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_Contract.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Network.P2P.Payloads;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using EpicChain.Wallets;
using System;
using System.Linq;
using System.Security.Cryptography;

namespace EpicChain.UnitTests.SmartContract
{
    [TestClass]
    public class UT_Contract
    {
        [TestMethod]
        public void TestGetScriptHash()
        {
            byte[] privateKey = new byte[32];
            RandomNumberGenerator rng = RandomNumberGenerator.Create();
            rng.GetBytes(privateKey);
            KeyPair key = new KeyPair(privateKey);
            Contract contract = Contract.CreateSignatureContract(key.PublicKey);
            byte[] expectedArray = new byte[40];
            expectedArray[0] = (byte)OpCode.PUSHDATA1;
            expectedArray[1] = 0x21;
            Array.Copy(key.PublicKey.EncodePoint(true), 0, expectedArray, 2, 33);
            expectedArray[35] = (byte)OpCode.SYSCALL;
            Array.Copy(BitConverter.GetBytes(ApplicationEngine.System_Crypto_CheckSig), 0, expectedArray, 36, 4);
            Assert.AreEqual(expectedArray.ToScriptHash(), contract.ScriptHash);
        }

        [TestMethod]
        public void TestCreate()
        {
            byte[] script = new byte[32];
            ContractParameterType[] parameterList = new ContractParameterType[] { ContractParameterType.Signature };
            Contract contract = Contract.Create(parameterList, script);
            Assert.AreEqual(contract.Script, script);
            Assert.AreEqual(1, contract.ParameterList.Length);
            Assert.AreEqual(ContractParameterType.Signature, contract.ParameterList[0]);
        }

        [TestMethod]
        public void TestCreateMultiSigContract()
        {
            byte[] privateKey1 = new byte[32];
            RandomNumberGenerator rng1 = RandomNumberGenerator.Create();
            rng1.GetBytes(privateKey1);
            KeyPair key1 = new KeyPair(privateKey1);
            byte[] privateKey2 = new byte[32];
            RandomNumberGenerator rng2 = RandomNumberGenerator.Create();
            rng2.GetBytes(privateKey2);
            KeyPair key2 = new KeyPair(privateKey2);
            EpicChain.Cryptography.ECC.ECPoint[] publicKeys = new EpicChain.Cryptography.ECC.ECPoint[2];
            publicKeys[0] = key1.PublicKey;
            publicKeys[1] = key2.PublicKey;
            publicKeys = publicKeys.OrderBy(p => p).ToArray();
            Contract contract = Contract.CreateMultiSigContract(2, publicKeys);
            byte[] expectedArray = new byte[77];
            expectedArray[0] = (byte)OpCode.PUSH2;
            expectedArray[1] = (byte)OpCode.PUSHDATA1;
            expectedArray[2] = 0x21;
            Array.Copy(publicKeys[0].EncodePoint(true), 0, expectedArray, 3, 33);
            expectedArray[36] = (byte)OpCode.PUSHDATA1;
            expectedArray[37] = 0x21;
            Array.Copy(publicKeys[1].EncodePoint(true), 0, expectedArray, 38, 33);
            expectedArray[71] = (byte)OpCode.PUSH2;
            expectedArray[72] = (byte)OpCode.SYSCALL;
            Array.Copy(BitConverter.GetBytes(ApplicationEngine.System_Crypto_CheckMultisig), 0, expectedArray, 73, 4);
            CollectionAssert.AreEqual(expectedArray, contract.Script);
            Assert.AreEqual(2, contract.ParameterList.Length);
            Assert.AreEqual(ContractParameterType.Signature, contract.ParameterList[0]);
            Assert.AreEqual(ContractParameterType.Signature, contract.ParameterList[1]);
        }

        [TestMethod]
        public void TestCreateMultiSigRedeemScript()
        {
            byte[] privateKey1 = new byte[32];
            RandomNumberGenerator rng1 = RandomNumberGenerator.Create();
            rng1.GetBytes(privateKey1);
            KeyPair key1 = new KeyPair(privateKey1);
            byte[] privateKey2 = new byte[32];
            RandomNumberGenerator rng2 = RandomNumberGenerator.Create();
            rng2.GetBytes(privateKey2);
            KeyPair key2 = new KeyPair(privateKey2);
            EpicChain.Cryptography.ECC.ECPoint[] publicKeys = new EpicChain.Cryptography.ECC.ECPoint[2];
            publicKeys[0] = key1.PublicKey;
            publicKeys[1] = key2.PublicKey;
            publicKeys = publicKeys.OrderBy(p => p).ToArray();
            Action action = () => Contract.CreateMultiSigRedeemScript(0, publicKeys);
            action.Should().Throw<ArgumentException>();
            byte[] script = Contract.CreateMultiSigRedeemScript(2, publicKeys);
            byte[] expectedArray = new byte[77];
            expectedArray[0] = (byte)OpCode.PUSH2;
            expectedArray[1] = (byte)OpCode.PUSHDATA1;
            expectedArray[2] = 0x21;
            Array.Copy(publicKeys[0].EncodePoint(true), 0, expectedArray, 3, 33);
            expectedArray[36] = (byte)OpCode.PUSHDATA1;
            expectedArray[37] = 0x21;
            Array.Copy(publicKeys[1].EncodePoint(true), 0, expectedArray, 38, 33);
            expectedArray[71] = (byte)OpCode.PUSH2;
            expectedArray[72] = (byte)OpCode.SYSCALL;
            Array.Copy(BitConverter.GetBytes(ApplicationEngine.System_Crypto_CheckMultisig), 0, expectedArray, 73, 4);
            CollectionAssert.AreEqual(expectedArray, script);
        }

        [TestMethod]
        public void TestCreateSignatureContract()
        {
            byte[] privateKey = new byte[32];
            RandomNumberGenerator rng = RandomNumberGenerator.Create();
            rng.GetBytes(privateKey);
            KeyPair key = new KeyPair(privateKey);
            Contract contract = Contract.CreateSignatureContract(key.PublicKey);
            byte[] expectedArray = new byte[40];
            expectedArray[0] = (byte)OpCode.PUSHDATA1;
            expectedArray[1] = 0x21;
            Array.Copy(key.PublicKey.EncodePoint(true), 0, expectedArray, 2, 33);
            expectedArray[35] = (byte)OpCode.SYSCALL;
            Array.Copy(BitConverter.GetBytes(ApplicationEngine.System_Crypto_CheckSig), 0, expectedArray, 36, 4);
            CollectionAssert.AreEqual(expectedArray, contract.Script);
            Assert.AreEqual(1, contract.ParameterList.Length);
            Assert.AreEqual(ContractParameterType.Signature, contract.ParameterList[0]);
        }

        [TestMethod]
        public void TestCreateSignatureRedeemScript()
        {
            byte[] privateKey = new byte[32];
            RandomNumberGenerator rng = RandomNumberGenerator.Create();
            rng.GetBytes(privateKey);
            KeyPair key = new KeyPair(privateKey);
            byte[] script = Contract.CreateSignatureRedeemScript(key.PublicKey);
            byte[] expectedArray = new byte[40];
            expectedArray[0] = (byte)OpCode.PUSHDATA1;
            expectedArray[1] = 0x21;
            Array.Copy(key.PublicKey.EncodePoint(true), 0, expectedArray, 2, 33);
            expectedArray[35] = (byte)OpCode.SYSCALL;
            Array.Copy(BitConverter.GetBytes(ApplicationEngine.System_Crypto_CheckSig), 0, expectedArray, 36, 4);
            CollectionAssert.AreEqual(expectedArray, script);
        }

        [TestMethod]
        public void TestSignatureRedeemScriptFee()
        {
            byte[] privateKey = new byte[32];
            RandomNumberGenerator rng = RandomNumberGenerator.Create();
            rng.GetBytes(privateKey);
            KeyPair key = new KeyPair(privateKey);
            byte[] verification = Contract.CreateSignatureRedeemScript(key.PublicKey);
            byte[] invocation = new ScriptBuilder().EmitPush(UInt160.Zero).ToArray();

            var fee = CovenantChain.DefaultExecFeeFactor * (ApplicationEngine.OpCodePriceTable[(byte)OpCode.PUSHDATA1] * 2 + ApplicationEngine.OpCodePriceTable[(byte)OpCode.SYSCALL] + ApplicationEngine.CheckSigPrice);

            using (ApplicationEngine engine = ApplicationEngine.Create(TriggerType.Verification, new Transaction { Signers = Array.Empty<Signer>(), Attributes = Array.Empty<TransactionAttribute>() }, null, settings: TestBlockchain.TheEpicChainSystem.Settings))
            {
                engine.LoadScript(invocation.Concat(verification).ToArray(), configureState: p => p.CallFlags = CallFlags.None);
                engine.Execute();
                engine.FeeConsumed.Should().Be(fee);
            }
        }

        [TestMethod]
        public void TestCreateMultiSigRedeemScriptFee()
        {
            byte[] privateKey1 = new byte[32];
            RandomNumberGenerator rng1 = RandomNumberGenerator.Create();
            rng1.GetBytes(privateKey1);
            KeyPair key1 = new KeyPair(privateKey1);
            byte[] privateKey2 = new byte[32];
            RandomNumberGenerator rng2 = RandomNumberGenerator.Create();
            rng2.GetBytes(privateKey2);
            KeyPair key2 = new KeyPair(privateKey2);
            EpicChain.Cryptography.ECC.ECPoint[] publicKeys = new EpicChain.Cryptography.ECC.ECPoint[2];
            publicKeys[0] = key1.PublicKey;
            publicKeys[1] = key2.PublicKey;
            publicKeys = publicKeys.OrderBy(p => p).ToArray();
            byte[] verification = Contract.CreateMultiSigRedeemScript(2, publicKeys);
            byte[] invocation = new ScriptBuilder().EmitPush(UInt160.Zero).EmitPush(UInt160.Zero).ToArray();

            long fee = CovenantChain.DefaultExecFeeFactor * (ApplicationEngine.OpCodePriceTable[(byte)OpCode.PUSHDATA1] * (2 + 2) + ApplicationEngine.OpCodePriceTable[(byte)OpCode.PUSHINT8] * 2 + ApplicationEngine.OpCodePriceTable[(byte)OpCode.SYSCALL] + ApplicationEngine.CheckSigPrice * 2);

            using (ApplicationEngine engine = ApplicationEngine.Create(TriggerType.Verification, new Transaction { Signers = Array.Empty<Signer>(), Attributes = Array.Empty<TransactionAttribute>() }, null, settings: TestBlockchain.TheEpicChainSystem.Settings))
            {
                engine.LoadScript(invocation.Concat(verification).ToArray(), configureState: p => p.CallFlags = CallFlags.None);
                engine.Execute();
                engine.FeeConsumed.Should().Be(fee);
            }
        }
    }
}
