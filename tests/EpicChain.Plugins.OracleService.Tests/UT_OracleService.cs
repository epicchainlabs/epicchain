// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_OracleService.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Akka.TestKit.Xunit2;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using EpicChain.Cryptography.ECC;
using EpicChain.Network.P2P.Payloads;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;

namespace EpicChain.Plugins.OracleService.Tests
{
    [TestClass]
    public class UT_OracleService : TestKit
    {
        [TestInitialize]
        public void TestSetup()
        {
            TestBlockchain.InitializeMockEpicChainSystem();
        }

        [TestMethod]
        public void TestFilter()
        {
            var json = @"{
  ""Stores"": [
    ""Lambton Quay"",
    ""Willis Street""
  ],
  ""Manufacturers"": [
    {
      ""Name"": ""Acme Co"",
      ""Products"": [
        {
          ""Name"": ""Anvil"",
          ""Price"": 50
        }
      ]
    },
    {
      ""Name"": ""Contoso"",
      ""Products"": [
        {
          ""Name"": ""Elbow Grease"",
          ""Price"": 99.95
        },
        {
          ""Name"": ""Headlight Fluid"",
          ""Price"": 4
        }
      ]
    }
  ]
}";

            Assert.AreEqual(@"[""Acme Co""]", Utility.StrictUTF8.GetString(OracleService.Filter(json, "$.Manufacturers[0].Name")));
            Assert.AreEqual("[50]", Utility.StrictUTF8.GetString(OracleService.Filter(json, "$.Manufacturers[0].Products[0].Price")));
            Assert.AreEqual(@"[""Elbow Grease""]", Utility.StrictUTF8.GetString(OracleService.Filter(json, "$.Manufacturers[1].Products[0].Name")));
            Assert.AreEqual(@"[{""Name"":""Elbow Grease"",""Price"":99.95}]", Utility.StrictUTF8.GetString(OracleService.Filter(json, "$.Manufacturers[1].Products[0]")));
        }

        [TestMethod]
        public void TestCreateOracleResponseTx()
        {
            var snapshotCache = TestBlockchain.GetTestSnapshotCache();

            var executionFactor = NativeContract.Policy.GetExecFeeFactor(snapshotCache);
            Assert.AreEqual(executionFactor, (uint)30);
            var feePerByte = NativeContract.Policy.GetFeePerByte(snapshotCache);
            Assert.AreEqual(feePerByte, 1000);

            OracleRequest request = new OracleRequest
            {
                OriginalTxid = UInt256.Zero,
                EpicPulseForResponse = 100000000 * 1,
                Url = "https://127.0.0.1/test",
                Filter = "",
                CallbackContract = UInt160.Zero,
                CallbackMethod = "callback",
                UserData = []
            };
            byte Prefix_Transaction = 11;
            snapshotCache.Add(NativeContract.Ledger.CreateStorageKey(Prefix_Transaction, request.OriginalTxid), new StorageItem(new TransactionState()
            {
                BlockIndex = 1,
                Transaction = new Transaction()
                {
                    ValidUntilBlock = 1
                }
            }));
            OracleResponse response = new OracleResponse() { Id = 1, Code = OracleResponseCode.Success, Result = new byte[] { 0x00 } };
            ECPoint[] oracleNodes = new ECPoint[] { ECCurve.Secp256r1.G };
            var tx = OracleService.CreateResponseTx(snapshotCache, request, response, oracleNodes, ProtocolSettings.Default);

            Assert.AreEqual(166, tx.Size);
            Assert.AreEqual(2198650, tx.NetworkFee);
            Assert.AreEqual(97801350, tx.SystemFee);

            // case (2) The size of attribute exceed the maximum limit

            request.EpicPulseForResponse = 0_10000000;
            response.Result = new byte[10250];
            tx = OracleService.CreateResponseTx(snapshotCache, request, response, oracleNodes, ProtocolSettings.Default);
            Assert.AreEqual(165, tx.Size);
            Assert.AreEqual(OracleResponseCode.InsufficientFunds, response.Code);
            Assert.AreEqual(2197650, tx.NetworkFee);
            Assert.AreEqual(7802350, tx.SystemFee);
        }
    }
}
