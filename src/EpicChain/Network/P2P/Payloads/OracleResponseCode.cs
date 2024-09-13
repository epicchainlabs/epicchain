// Copyright (C) 2021-2024 EpicChain Labs.

//
// OracleResponseCode.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


namespace EpicChain.Network.P2P.Payloads
{
    /// <summary>
    /// Represents the response code for the oracle request.
    /// </summary>
    public enum OracleResponseCode : byte
    {
        /// <summary>
        /// Indicates that the request has been successfully completed.
        /// </summary>
        Success = 0x00,

        /// <summary>
        /// Indicates that the protocol of the request is not supported.
        /// </summary>
        ProtocolNotSupported = 0x10,

        /// <summary>
        /// Indicates that the oracle nodes cannot reach a consensus on the result of the request.
        /// </summary>
        ConsensusUnreachable = 0x12,

        /// <summary>
        /// Indicates that the requested Uri does not exist.
        /// </summary>
        NotFound = 0x14,

        /// <summary>
        /// Indicates that the request was not completed within the specified time.
        /// </summary>
        Timeout = 0x16,

        /// <summary>
        /// Indicates that there is no permission to request the resource.
        /// </summary>
        Forbidden = 0x18,

        /// <summary>
        /// Indicates that the data for the response is too large.
        /// </summary>
        ResponseTooLarge = 0x1a,

        /// <summary>
        /// Indicates that the request failed due to insufficient balance.
        /// </summary>
        InsufficientFunds = 0x1c,

        /// <summary>
        /// Indicates that the content-type of the request is not supported.
        /// </summary>
        ContentTypeNotSupported = 0x1f,

        /// <summary>
        /// Indicates that the request failed due to other errors.
        /// </summary>
        Error = 0xff
    }
}
