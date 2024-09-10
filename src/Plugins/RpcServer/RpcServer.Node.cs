// Copyright (C) 2021-2024 EpicChain Labs.

//
// RpcServer.Node.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Akka.Actor;
using Neo.IO;
using Neo.Json;
using Neo.Ledger;
using Neo.Network.P2P;
using Neo.Network.P2P.Payloads;
using System;
using System.Linq;
using static Neo.Ledger.Blockchain;

namespace Neo.Plugins.RpcServer
{
    partial class RpcServer
    {
        [RpcMethod]
        protected internal virtual JToken GetConnectionCount(JArray _params)
        {
            return localNode.ConnectedCount;
        }

        [RpcMethod]
        protected internal virtual JToken GetPeers(JArray _params)
        {
            JObject json = new();
            json["unconnected"] = new JArray(localNode.GetUnconnectedPeers().Select(p =>
            {
                JObject peerJson = new();
                peerJson["address"] = p.Address.ToString();
                peerJson["port"] = p.Port;
                return peerJson;
            }));
            json["bad"] = new JArray(); //badpeers has been removed
            json["connected"] = new JArray(localNode.GetRemoteNodes().Select(p =>
            {
                JObject peerJson = new();
                peerJson["address"] = p.Remote.Address.ToString();
                peerJson["port"] = p.ListenerTcpPort;
                return peerJson;
            }));
            return json;
        }

        private static JObject GetRelayResult(VerifyResult reason, UInt256 hash)
        {

            switch (reason)
            {
                case VerifyResult.Succeed:
                    {
                        var ret = new JObject();
                        ret["hash"] = hash.ToString();
                        return ret;
                    }
                case VerifyResult.AlreadyExists:
                    {
                        throw new RpcException(RpcError.AlreadyExists.WithData(reason.ToString()));
                    }
                case VerifyResult.AlreadyInPool:
                    {
                        throw new RpcException(RpcError.AlreadyInPool.WithData(reason.ToString()));
                    }
                case VerifyResult.OutOfMemory:
                    {
                        throw new RpcException(RpcError.MempoolCapReached.WithData(reason.ToString()));
                    }
                case VerifyResult.InvalidScript:
                    {
                        throw new RpcException(RpcError.InvalidScript.WithData(reason.ToString()));
                    }
                case VerifyResult.InvalidAttribute:
                    {
                        throw new RpcException(RpcError.InvalidAttribute.WithData(reason.ToString()));
                    }
                case VerifyResult.InvalidSignature:
                    {
                        throw new RpcException(RpcError.InvalidSignature.WithData(reason.ToString()));
                    }
                case VerifyResult.OverSize:
                    {
                        throw new RpcException(RpcError.InvalidSize.WithData(reason.ToString()));
                    }
                case VerifyResult.Expired:
                    {
                        throw new RpcException(RpcError.ExpiredTransaction.WithData(reason.ToString()));
                    }
                case VerifyResult.InsufficientFunds:
                    {
                        throw new RpcException(RpcError.InsufficientFunds.WithData(reason.ToString()));
                    }
                case VerifyResult.PolicyFail:
                    {
                        throw new RpcException(RpcError.PolicyFailed.WithData(reason.ToString()));
                    }
                default:
                    {
                        throw new RpcException(RpcError.VerificationFailed.WithData(reason.ToString()));
                    }
            }
        }

        [RpcMethod]
        protected internal virtual JToken GetVersion(JArray _params)
        {
            JObject json = new();
            json["tcpport"] = localNode.ListenerTcpPort;
            json["nonce"] = LocalNode.Nonce;
            json["useragent"] = LocalNode.UserAgent;
            // rpc settings
            JObject rpc = new();
            rpc["maxiteratorresultitems"] = settings.MaxIteratorResultItems;
            rpc["sessionenabled"] = settings.SessionEnabled;
            // protocol settings
            JObject protocol = new();
            protocol["addressversion"] = system.Settings.AddressVersion;
            protocol["network"] = system.Settings.Network;
            protocol["validatorscount"] = system.Settings.ValidatorsCount;
            protocol["msperblock"] = system.Settings.MillisecondsPerBlock;
            protocol["maxtraceableblocks"] = system.Settings.MaxTraceableBlocks;
            protocol["maxvaliduntilblockincrement"] = system.Settings.MaxValidUntilBlockIncrement;
            protocol["maxtransactionsperblock"] = system.Settings.MaxTransactionsPerBlock;
            protocol["memorypoolmaxtransactions"] = system.Settings.MemoryPoolMaxTransactions;
            protocol["InitialEpicPulseDistribution"] = system.Settings.InitialEpicPulseDistribution;
            protocol["hardforks"] = new JArray(system.Settings.Hardforks.Select(hf =>
            {
                JObject forkJson = new();
                // Strip "HF_" prefix.
                forkJson["name"] = StripPrefix(hf.Key.ToString(), "HF_");
                forkJson["blockheight"] = hf.Value;
                return forkJson;
            }));
            protocol["standbycommittee"] = new JArray(system.Settings.StandbyCommittee.Select(u => new JString(u.ToString())));
            protocol["seedlist"] = new JArray(system.Settings.SeedList.Select(u => new JString(u)));
            json["rpc"] = rpc;
            json["protocol"] = protocol;
            return json;
        }

        private static string StripPrefix(string s, string prefix)
        {
            return s.StartsWith(prefix) ? s.Substring(prefix.Length) : s;
        }

        [RpcMethod]
        protected internal virtual JToken SendRawTransaction(JArray _params)
        {
            Transaction tx = Result.Ok_Or(() => Convert.FromBase64String(_params[0].AsString()).AsSerializable<Transaction>(), RpcError.InvalidParams.WithData($"Invalid Transaction Format: {_params[0]}"));
            RelayResult reason = system.Blockchain.Ask<RelayResult>(tx).Result;
            return GetRelayResult(reason.Result, tx.Hash);
        }

        [RpcMethod]
        protected internal virtual JToken SubmitBlock(JArray _params)
        {
            Block block = Result.Ok_Or(() => Convert.FromBase64String(_params[0].AsString()).AsSerializable<Block>(), RpcError.InvalidParams.WithData($"Invalid Block Format: {_params[0]}"));
            RelayResult reason = system.Blockchain.Ask<RelayResult>(block).Result;
            return GetRelayResult(reason.Result, block.Hash);
        }
    }
}
