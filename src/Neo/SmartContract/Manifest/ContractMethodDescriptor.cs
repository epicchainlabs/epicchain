// Copyright (C) 2021-2024 The EpicChain Labs.
//
// ContractMethodDescriptor.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.Json;
using Neo.VM;
using Neo.VM.Types;
using System;
using System.Linq;

namespace Neo.SmartContract.Manifest
{
    /// <summary>
    /// Represents a method in a smart contract ABI.
    /// </summary>
    public class ContractMethodDescriptor : ContractEventDescriptor
    {
        /// <summary>
        /// Indicates the return type of the method. It can be any value of <see cref="ContractParameterType"/>.
        /// </summary>
        public ContractParameterType ReturnType { get; set; }

        /// <summary>
        /// The position of the method in the contract script.
        /// </summary>
        public int Offset { get; set; }

        /// <summary>
        /// Indicates whether the method is a safe method.
        /// If a method is marked as safe, the user interface will not give any warnings when it is called by other contracts.
        /// </summary>
        public bool Safe { get; set; }

        public override void FromStackItem(StackItem stackItem)
        {
            base.FromStackItem(stackItem);
            Struct @struct = (Struct)stackItem;
            ReturnType = (ContractParameterType)(byte)@struct[2].GetInteger();
            Offset = (int)@struct[3].GetInteger();
            Safe = @struct[4].GetBoolean();
        }

        public override StackItem ToStackItem(ReferenceCounter referenceCounter)
        {
            Struct @struct = (Struct)base.ToStackItem(referenceCounter);
            @struct.Add((byte)ReturnType);
            @struct.Add(Offset);
            @struct.Add(Safe);
            return @struct;
        }

        /// <summary>
        /// Converts the method from a JSON object.
        /// </summary>
        /// <param name="json">The method represented by a JSON object.</param>
        /// <returns>The converted method.</returns>
        public new static ContractMethodDescriptor FromJson(JObject json)
        {
            ContractMethodDescriptor descriptor = new()
            {
                Name = json["name"].GetString(),
                Parameters = ((JArray)json["parameters"]).Select(u => ContractParameterDefinition.FromJson((JObject)u)).ToArray(),
                ReturnType = Enum.Parse<ContractParameterType>(json["returntype"].GetString()),
                Offset = json["offset"].GetInt32(),
                Safe = json["safe"].GetBoolean()
            };
            if (string.IsNullOrEmpty(descriptor.Name)) throw new FormatException();
            _ = descriptor.Parameters.ToDictionary(p => p.Name);
            if (!Enum.IsDefined(typeof(ContractParameterType), descriptor.ReturnType)) throw new FormatException();
            if (descriptor.Offset < 0) throw new FormatException();
            return descriptor;
        }

        /// <summary>
        /// Converts the method to a JSON object.
        /// </summary>
        /// <returns>The method represented by a JSON object.</returns>
        public override JObject ToJson()
        {
            var json = base.ToJson();
            json["returntype"] = ReturnType.ToString();
            json["offset"] = Offset;
            json["safe"] = Safe;
            return json;
        }
    }
}
