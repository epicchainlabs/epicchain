// Copyright (C) 2021-2024 EpicChain Labs.

//
// OracleEpicChainNovaProtocol.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Cryptography.ECC;
using EpicChain.FileStorage.API.Client;
using EpicChain.FileStorage.API.Cryptography;
using EpicChain.FileStorage.API.Refs;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Wallets;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using System.Web;
using Object = EpicChain.FileStorage.API.Object.Object;
using Range = EpicChain.FileStorage.API.Object.Range;

namespace EpicChain.Plugins.OracleService
{
    class OracleEpicChainNovaProtocol : IOracleProtocol
    {
        private readonly System.Security.Cryptography.ECDsa privateKey;

        public OracleEpicChainNovaProtocol(Wallet wallet, ECPoint[] oracles)
        {
            byte[] key = oracles.Select(p => wallet.GetAccount(p)).Where(p => p is not null && p.HasKey && !p.Lock).FirstOrDefault().GetKey().PrivateKey;
            privateKey = key.LoadPrivateKey();
        }

        public void Configure()
        {
        }

        public void Dispose()
        {
            privateKey.Dispose();
        }

        public async Task<(OracleResponseCode, string)> ProcessAsync(Uri uri, CancellationToken cancellation)
        {
            Utility.Log(nameof(OracleEpicChainNovaProtocol), LogLevel.Debug, $"Request: {uri.AbsoluteUri}");
            try
            {
                (OracleResponseCode code, string data) = await GetAsync(uri, Settings.Default.EpicChainNova.EndPoint, cancellation);
                Utility.Log(nameof(OracleEpicChainNovaProtocol), LogLevel.Debug, $"EpicChainNova result, code: {code}, data: {data}");
                return (code, data);
            }
            catch (Exception e)
            {
                Utility.Log(nameof(OracleEpicChainNovaProtocol), LogLevel.Debug, $"EpicChainNova result: error,{e.Message}");
                return (OracleResponseCode.Error, null);
            }
        }


        /// <summary>
        /// GetAsync returns epicchainnova object from the provided url.
        /// If Command is not provided, full object is requested.
        /// </summary>
        /// <param name="uri">URI scheme is "epicchainnova:ContainerID/ObjectID/Command/offset|length".</param>
        /// <param name="host">Client host.</param>
        /// <param name="cancellation">Cancellation token object.</param>
        /// <returns>Returns epicchainnova object.</returns>
        private async Task<(OracleResponseCode, string)> GetAsync(Uri uri, string host, CancellationToken cancellation)
        {
            string[] ps = uri.AbsolutePath.Split("/");
            if (ps.Length < 2) throw new FormatException("Invalid epicchainnova url");
            ContainerID containerID = ContainerID.FromString(ps[0]);
            ObjectID objectID = ObjectID.FromString(ps[1]);
            Address objectAddr = new()
            {
                ContainerId = containerID,
                ObjectId = objectID
            };
            using Client client = new(privateKey, host);
            var tokenSource = CancellationTokenSource.CreateLinkedTokenSource(cancellation);
            tokenSource.CancelAfter(Settings.Default.EpicChainNova.Timeout);
            if (ps.Length == 2)
                return GetPayload(client, objectAddr, tokenSource.Token);
            return ps[2] switch
            {
                "range" => await GetRangeAsync(client, objectAddr, ps.Skip(3).ToArray(), tokenSource.Token),
                "header" => (OracleResponseCode.Success, await GetHeaderAsync(client, objectAddr, tokenSource.Token)),
                "hash" => (OracleResponseCode.Success, await GetHashAsync(client, objectAddr, ps.Skip(3).ToArray(), tokenSource.Token)),
                _ => throw new Exception("invalid command")
            };
        }

        private static (OracleResponseCode, string) GetPayload(Client client, Address addr, CancellationToken cancellation)
        {
            var objReader = client.GetObjectInit(addr, options: new CallOptions { Ttl = 2 }, context: cancellation);
            var obj = objReader.ReadHeader();
            if (obj.PayloadSize > OracleResponse.MaxResultSize)
                return (OracleResponseCode.ResponseTooLarge, "");
            var payload = new byte[obj.PayloadSize];
            int offset = 0;
            while (true)
            {
                if ((ulong)offset > obj.PayloadSize) return (OracleResponseCode.ResponseTooLarge, "");
                (byte[] chunk, bool ok) = objReader.ReadChunk();
                if (!ok) break;
                Array.Copy(chunk, 0, payload, offset, chunk.Length);
                offset += chunk.Length;
            }
            return (OracleResponseCode.Success, Utility.StrictUTF8.GetString(payload));
        }

        private static async Task<(OracleResponseCode, string)> GetRangeAsync(Client client, Address addr, string[] ps, CancellationToken cancellation)
        {
            if (ps.Length == 0) throw new FormatException("missing object range (expected 'Offset|Length')");
            Range range = ParseRange(ps[0]);
            if (range.Length > OracleResponse.MaxResultSize) return (OracleResponseCode.ResponseTooLarge, "");
            var res = await client.GetObjectPayloadRangeData(addr, range, options: new CallOptions { Ttl = 2 }, context: cancellation);
            return (OracleResponseCode.Success, Utility.StrictUTF8.GetString(res));
        }

        private static async Task<string> GetHeaderAsync(Client client, Address addr, CancellationToken cancellation)
        {
            var obj = await client.GetObjectHeader(addr, options: new CallOptions { Ttl = 2 }, context: cancellation);
            return obj.ToString();
        }

        private static async Task<string> GetHashAsync(Client client, Address addr, string[] ps, CancellationToken cancellation)
        {
            if (ps.Length == 0 || ps[0] == "")
            {
                Object obj = await client.GetObjectHeader(addr, options: new CallOptions { Ttl = 2 }, context: cancellation);
                return $"\"{new UInt256(obj.PayloadChecksum.Sum.ToByteArray())}\"";
            }
            Range range = ParseRange(ps[0]);
            List<byte[]> hashes = await client.GetObjectPayloadRangeHash(addr, new List<Range>() { range }, ChecksumType.Sha256, Array.Empty<byte>(), new CallOptions { Ttl = 2 }, cancellation);
            if (hashes.Count == 0) throw new Exception("empty response, object range is invalid (expected 'Offset|Length')");
            return $"\"{new UInt256(hashes[0])}\"";
        }

        private static Range ParseRange(string s)
        {
            string url = HttpUtility.UrlDecode(s);
            int sepIndex = url.IndexOf("|");
            if (sepIndex < 0) throw new Exception("object range is invalid (expected 'Offset|Length')");
            ulong offset = ulong.Parse(url[..sepIndex]);
            ulong length = ulong.Parse(url[(sepIndex + 1)..]);
            return new Range() { Offset = offset, Length = length };
        }
    }
}