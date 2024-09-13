// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_ContractClient.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Microsoft.VisualStudio.TestTools.UnitTesting;
using Moq;
using EpicChain.Extensions;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Manifest;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using EpicChain.Wallets;
using System.Threading.Tasks;

namespace EpicChain.Network.RPC.Tests
{
    [TestClass]
    public class UT_ContractClient
    {
        Mock<RpcClient> rpcClientMock;
        KeyPair keyPair1;
        UInt160 sender;

        [TestInitialize]
        public void TestSetup()
        {
            keyPair1 = new KeyPair(Wallet.GetPrivateKeyFromWIF("KyXwTh1hB76RRMquSvnxZrJzQx7h9nQP2PCRL38v6VDb5ip3nf1p"));
            sender = Contract.CreateSignatureRedeemScript(keyPair1.PublicKey).ToScriptHash();
            rpcClientMock = UT_TransactionManager.MockRpcClient(sender, new byte[0]);
        }

        [TestMethod]
        public async Task TestInvoke()
        {
            byte[] testScript = NativeContract.EpicPulse.Hash.MakeScript("balanceOf", UInt160.Zero);
            UT_TransactionManager.MockInvokeScript(rpcClientMock, testScript, new ContractParameter { Type = ContractParameterType.ByteArray, Value = "00e057eb481b".HexToBytes() });

            ContractClient contractClient = new ContractClient(rpcClientMock.Object);
            var result = await contractClient.TestInvokeAsync(NativeContract.EpicPulse.Hash, "balanceOf", UInt160.Zero);

            Assert.AreEqual(30000000000000L, (long)result.Stack[0].GetInteger());
        }

        [TestMethod]
        public async Task TestDeployContract()
        {
            byte[] script;
            var manifest = new ContractManifest()
            {
                Permissions = new[] { ContractPermission.DefaultPermission },
                Abi = new ContractAbi()
                {
                    Events = new ContractEventDescriptor[0],
                    Methods = new ContractMethodDescriptor[0]
                },
                Groups = new ContractGroup[0],
                Trusts = WildcardContainer<ContractPermissionDescriptor>.Create(),
                SupportedStandards = new string[] { "XEP-10" },
                Extra = null,
            };
            using (ScriptBuilder sb = new ScriptBuilder())
            {
                sb.EmitDynamicCall(NativeContract.ContractManagement.Hash, "deploy", new byte[1], manifest.ToJson().ToString());
                script = sb.ToArray();
            }

            UT_TransactionManager.MockInvokeScript(rpcClientMock, script, new ContractParameter());

            ContractClient contractClient = new ContractClient(rpcClientMock.Object);
            var result = await contractClient.CreateDeployContractTxAsync(new byte[1], manifest, keyPair1);

            Assert.IsNotNull(result);
        }
    }
}
