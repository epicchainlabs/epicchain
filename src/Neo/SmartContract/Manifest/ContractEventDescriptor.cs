// Copyright (C) 2021-2024 The EpicChain Labs.
//
// ContractEventDescriptor.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
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
using Array = Neo.VM.Types.Array;

namespace Neo.SmartContract.Manifest
{
    /// <summary>
    /// Represents an event in a smart contract ABI.
    /// </summary>
    public class ContractEventDescriptor : IInteroperable
    {
        /// <summary>
        /// The name of the event or method.
        /// </summary>
        public string Name { get; set; }

        /// <summary>
        /// The parameters of the event or method.
        /// </summary>
        public ContractParameterDefinition[] Parameters { get; set; }

        public virtual void FromStackItem(StackItem stackItem)
        {
            Struct @struct = (Struct)stackItem;
            Name = @struct[0].GetString();
            Parameters = ((Array)@struct[1]).Select(p => p.ToInteroperable<ContractParameterDefinition>()).ToArray();
        }

        public virtual StackItem ToStackItem(ReferenceCounter referenceCounter)
        {
            return new Struct(referenceCounter)
            {
                Name,
                new Array(referenceCounter, Parameters.Select(p => p.ToStackItem(referenceCounter)))
            };
        }

        /// <summary>
        /// Converts the event from a JSON object.
        /// </summary>
        /// <param name="json">The event represented by a JSON object.</param>
        /// <returns>The converted event.</returns>
        public static ContractEventDescriptor FromJson(JObject json)
        {
            ContractEventDescriptor descriptor = new()
            {
                Name = json["name"].GetString(),
                Parameters = ((JArray)json["parameters"]).Select(u => ContractParameterDefinition.FromJson((JObject)u)).ToArray(),
            };
            if (string.IsNullOrEmpty(descriptor.Name)) throw new FormatException();
            _ = descriptor.Parameters.ToDictionary(p => p.Name);
            return descriptor;
        }

        /// <summary>
        /// Converts the event to a JSON object.
        /// </summary>
        /// <returns>The event represented by a JSON object.</returns>
        public virtual JObject ToJson()
        {
            var json = new JObject();
            json["name"] = Name;
            json["parameters"] = new JArray(Parameters.Select(u => u.ToJson()).ToArray());
            return json;
        }
    }
}
