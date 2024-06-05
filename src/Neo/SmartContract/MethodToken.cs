// Copyright (C) 2021-2024 The EpicChain Labs.
//
// MethodToken.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
using Neo.Json;
using System;
using System.IO;

namespace Neo.SmartContract
{
    /// <summary>
    /// Represents the methods that a contract will call statically.
    /// </summary>
    public class MethodToken : ISerializable
    {
        /// <summary>
        /// The hash of the contract to be called.
        /// </summary>
        public UInt160 Hash;

        /// <summary>
        /// The name of the method to be called.
        /// </summary>
        public string Method;

        /// <summary>
        /// The number of parameters of the method to be called.
        /// </summary>
        public ushort ParametersCount;

        /// <summary>
        /// Indicates whether the method to be called has a return value.
        /// </summary>
        public bool HasReturnValue;

        /// <summary>
        /// The <see cref="CallFlags"/> to be used to call the contract.
        /// </summary>
        public CallFlags CallFlags;

        public int Size =>
            UInt160.Length +        // Hash
            Method.GetVarSize() +   // Method
            sizeof(ushort) +        // ParametersCount
            sizeof(bool) +          // HasReturnValue
            sizeof(CallFlags);      // CallFlags

        void ISerializable.Deserialize(ref MemoryReader reader)
        {
            Hash = reader.ReadSerializable<UInt160>();
            Method = reader.ReadVarString(32);
            if (Method.StartsWith('_')) throw new FormatException();
            ParametersCount = reader.ReadUInt16();
            HasReturnValue = reader.ReadBoolean();
            CallFlags = (CallFlags)reader.ReadByte();
            if ((CallFlags & ~CallFlags.All) != 0) throw new FormatException();
        }

        void ISerializable.Serialize(BinaryWriter writer)
        {
            writer.Write(Hash);
            writer.WriteVarString(Method);
            writer.Write(ParametersCount);
            writer.Write(HasReturnValue);
            writer.Write((byte)CallFlags);
        }

        /// <summary>
        /// Converts the token to a JSON object.
        /// </summary>
        /// <returns>The token represented by a JSON object.</returns>
        public JObject ToJson()
        {
            return new JObject
            {
                ["hash"] = Hash.ToString(),
                ["method"] = Method,
                ["paramcount"] = ParametersCount,
                ["hasreturnvalue"] = HasReturnValue,
                ["callflags"] = CallFlags
            };
        }
    }
}
