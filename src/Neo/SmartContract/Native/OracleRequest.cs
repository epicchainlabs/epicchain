// Copyright (C) 2021-2024 The EpicChain Labs.
//
// OracleRequest.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.IO;
using Neo.VM;
using Neo.VM.Types;
using Array = Neo.VM.Types.Array;

namespace Neo.SmartContract.Native
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
        /// The maximum amount of GAS that can be used when executing response callback.
        /// </summary>
        public long GasForResponse;

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
            GasForResponse = (long)array[1].GetInteger();
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
                GasForResponse,
                Url,
                Filter ?? StackItem.Null,
                CallbackContract.ToArray(),
                CallbackMethod,
                UserData
            };
        }
    }
}
