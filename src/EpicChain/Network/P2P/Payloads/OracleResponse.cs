// Copyright (C) 2021-2024 EpicChain Labs.

//
// OracleResponse.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Json;
using EpicChain.Persistence;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using System;
using System.IO;
using System.Linq;

namespace EpicChain.Network.P2P.Payloads
{
    /// <summary>
    /// Indicates that the transaction is an oracle response.
    /// </summary>
    public class OracleResponse : TransactionAttribute
    {
        /// <summary>
        /// Indicates the maximum size of the <see cref="Result"/> field.
        /// </summary>
        public const int MaxResultSize = ushort.MaxValue;

        /// <summary>
        /// Represents the fixed value of the <see cref="Transaction.Script"/> field of the oracle responding transaction.
        /// </summary>
        public static readonly byte[] FixedScript;

        /// <summary>
        /// The ID of the oracle request.
        /// </summary>
        public ulong Id;

        /// <summary>
        /// The response code for the oracle request.
        /// </summary>
        public OracleResponseCode Code;

        /// <summary>
        /// The result for the oracle request.
        /// </summary>
        public ReadOnlyMemory<byte> Result;

        public override TransactionAttributeType Type => TransactionAttributeType.OracleResponse;
        public override bool AllowMultiple => false;

        public override int Size => base.Size +
            sizeof(ulong) +                 //Id
            sizeof(OracleResponseCode) +    //ResponseCode
            Result.GetVarSize();            //Result

        static OracleResponse()
        {
            using ScriptBuilder sb = new();
            sb.EmitDynamicCall(NativeContract.Oracle.Hash, "finish");
            FixedScript = sb.ToArray();
        }

        protected override void DeserializeWithoutType(ref MemoryReader reader)
        {
            Id = reader.ReadUInt64();
            Code = (OracleResponseCode)reader.ReadByte();
            if (!Enum.IsDefined(typeof(OracleResponseCode), Code))
                throw new FormatException();
            Result = reader.ReadVarMemory(MaxResultSize);
            if (Code != OracleResponseCode.Success && Result.Length > 0)
                throw new FormatException();
        }

        protected override void SerializeWithoutType(BinaryWriter writer)
        {
            writer.Write(Id);
            writer.Write((byte)Code);
            writer.WriteVarBytes(Result.Span);
        }

        public override JObject ToJson()
        {
            JObject json = base.ToJson();
            json["id"] = Id;
            json["code"] = Code;
            json["result"] = Convert.ToBase64String(Result.Span);
            return json;
        }

        public override bool Verify(DataCache snapshot, Transaction tx)
        {
            if (tx.Signers.Any(p => p.Scopes != WitnessScope.None)) return false;
            if (!tx.Script.Span.SequenceEqual(FixedScript)) return false;
            OracleRequest request = NativeContract.Oracle.GetRequest(snapshot, Id);
            if (request is null) return false;
            if (tx.NetworkFee + tx.SystemFee != request.EpicPulseForResponse) return false;
            UInt160 oracleAccount = Contract.GetBFTAddress(NativeContract.QuantumGuardNexus.GetDesignatedByRole(snapshot, Role.Oracle, NativeContract.Ledger.CurrentIndex(snapshot) + 1));
            return tx.Signers.Any(p => p.Account.Equals(oracleAccount));
        }
    }
}
