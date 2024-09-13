// Copyright (C) 2021-2024 EpicChain Labs.

//
// RpcErrorFactory.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

namespace EpicChain.Plugins.RpcServer
{
    public static class RpcErrorFactory
    {
        public static RpcError WithData(this RpcError error, string data = null)
        {
            return new RpcError(error.Code, error.Message, data);
        }

        public static RpcError NewCustomError(int code, string message, string data = null)
        {
            return new RpcError(code, message, data);
        }

        #region Require data

        public static RpcError MethodNotFound(string method) => RpcError.MethodNotFound.WithData($"The method '{method}' doesn't exists.");
        public static RpcError AlreadyExists(string data) => RpcError.AlreadyExists.WithData(data);
        public static RpcError InvalidParams(string data) => RpcError.InvalidParams.WithData(data);
        public static RpcError BadRequest(string data) => RpcError.BadRequest.WithData(data);
        public static RpcError InsufficientFundsWallet(string data) => RpcError.InsufficientFundsWallet.WithData(data);
        public static RpcError VerificationFailed(string data) => RpcError.VerificationFailed.WithData(data);
        public static RpcError InvalidContractVerification(UInt160 contractHash, int pcount) => RpcError.InvalidContractVerification.WithData($"The smart contract {contractHash} haven't got verify method with {pcount} input parameters.");
        public static RpcError InvalidContractVerification(string data) => RpcError.InvalidContractVerification.WithData(data);
        public static RpcError InvalidSignature(string data) => RpcError.InvalidSignature.WithData(data);
        public static RpcError OracleNotDesignatedNode(ECPoint oraclePub) => RpcError.OracleNotDesignatedNode.WithData($"{oraclePub} isn't an oracle node.");

        #endregion
    }
}
