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


using EpicChain.Json;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Network.RPC.Models;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace EpicChain.Network.RPC.Tests
{
    internal static class TestUtils
    {
        public readonly static List<RpcTestCase> RpcTestCases = ((JArray)JToken.Parse(File.ReadAllText("RpcTestCases.json"))).Select(p => RpcTestCase.FromJson((JObject)p)).ToList();

        public static Block GetBlock(int txCount)
        {
            return new Block
            {
                Header = new Header
                {
                    PrevHash = UInt256.Zero,
                    MerkleRoot = UInt256.Zero,
                    NextConsensus = UInt160.Zero,
                    Witness = new Witness
                    {
                        InvocationScript = new byte[0],
                        VerificationScript = new byte[0]
                    }
                },
                Transactions = Enumerable.Range(0, txCount).Select(p => GetTransaction()).ToArray()
            };
        }

        public static Header GetHeader()
        {
            return GetBlock(0).Header;
        }

        public static Transaction GetTransaction()
        {
            return new Transaction
            {
                Script = new byte[1],
                Signers = new Signer[] { new Signer { Account = UInt160.Zero } },
                Attributes = new TransactionAttribute[0],
                Witnesses = new Witness[]
                {
                    new Witness
                    {
                        InvocationScript = new byte[0],
                        VerificationScript = new byte[0]
                    }
                }
            };
        }
    }

    internal class RpcTestCase
    {
        public string Name { get; set; }
        public RpcRequest Request { get; set; }
        public RpcResponse Response { get; set; }

        public JObject ToJson()
        {
            return new JObject
            {
                ["Name"] = Name,
                ["Request"] = Request.ToJson(),
                ["Response"] = Response.ToJson(),
            };
        }

        public static RpcTestCase FromJson(JObject json)
        {
            return new RpcTestCase
            {
                Name = json["Name"].AsString(),
                Request = RpcRequest.FromJson((JObject)json["Request"]),
                Response = RpcResponse.FromJson((JObject)json["Response"]),
            };
        }

    }
}
