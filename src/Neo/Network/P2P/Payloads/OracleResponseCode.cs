// Copyright (C) 2021-2024 The EpicChain Labs.
//
// OracleResponseCode.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


namespace Neo.Network.P2P.Payloads
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
