// Copyright (C) 2021-2024 EpicChain Labs.

//
// OracleRequest.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.IO;
using EpicChain.VM;
using EpicChain.VM.Types;
using Array = EpicChain.VM.Types.Array;

namespace EpicChain.SmartContract.Native
{
    /// <summary>
    /// Represents an Oracle request in smart contracts.
    /// </summary>
    public class OracleRequest : IInteroperable
    {
        /// <summary>
        /// The original transaction that sent the related request.
        /// </summary>
        public UInt256 OriginalTxid;

        /// <summary>
        /// The maximum amount of EpicPulse that can be used when executing response callback.
        /// </summary>
        public long EpicPulseForResponse;

        /// <summary>
        /// The url of the request.
        /// </summary>
        public string Url;

        /// <summary>
        /// The filter for the response.
        /// </summary>
        public string Filter;

        /// <summary>
        /// The hash of the callback contract.
        /// </summary>
        public UInt160 CallbackContract;

        /// <summary>
        /// The name of the callback method.
        /// </summary>
        public string CallbackMethod;

        /// <summary>
        /// The user-defined object that will be passed to the callback.
        /// </summary>
        public byte[] UserData;

        public void FromStackItem(StackItem stackItem)
        {
            Array array = (Array)stackItem;
            OriginalTxid = new UInt256(array[0].GetSpan());
            EpicPulseForResponse = (long)array[1].GetInteger();
            Url = array[2].GetString();
            Filter = array[3].GetString();
            CallbackContract = new UInt160(array[4].GetSpan());
            CallbackMethod = array[5].GetString();
            UserData = array[6].GetSpan().ToArray();
        }

        public StackItem ToStackItem(ReferenceCounter referenceCounter)
        {
            return new Array(referenceCounter)
            {
                OriginalTxid.ToArray(),
                EpicPulseForResponse,
                Url,
                Filter ?? StackItem.Null,
                CallbackContract.ToArray(),
                CallbackMethod,
                UserData
            };
        }
    }
}
