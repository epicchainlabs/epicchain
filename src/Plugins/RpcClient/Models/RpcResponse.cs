// Copyright (C) 2021-2024 EpicChain Labs.

//
// RpcResponse.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

namespace EpicChain.Network.RPC.Models
{
    public class RpcResponse
    {
        public JToken Id { get; set; }

        public string JsonRpc { get; set; }

        public RpcResponseError Error { get; set; }

        public JToken Result { get; set; }

        public string RawResponse { get; set; }

        public static RpcResponse FromJson(JObject json)
        {
            RpcResponse response = new()
            {
                Id = json["id"],
                JsonRpc = json["jsonrpc"].AsString(),
                Result = json["result"]
            };

            if (json["error"] != null)
            {
                response.Error = RpcResponseError.FromJson((JObject)json["error"]);
            }

            return response;
        }

        public JObject ToJson()
        {
            JObject json = new();
            json["id"] = Id;
            json["jsonrpc"] = JsonRpc;
            json["error"] = Error?.ToJson();
            json["result"] = Result;
            return json;
        }
    }

    public class RpcResponseError
    {
        public int Code { get; set; }

        public string Message { get; set; }

        public JToken Data { get; set; }

        public static RpcResponseError FromJson(JObject json)
        {
            return new RpcResponseError
            {
                Code = (int)json["code"].AsNumber(),
                Message = json["message"].AsString(),
                Data = json["data"],
            };
        }

        public JObject ToJson()
        {
            JObject json = new();
            json["code"] = Code;
            json["message"] = Message;
            json["data"] = Data;
            return json;
        }
    }
}
