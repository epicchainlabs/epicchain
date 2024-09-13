// Copyright (C) 2021-2024 EpicChain Labs.

//
// ContractEventDescriptor.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.VM;
using EpicChain.VM.Types;
using System;
using System.Linq;
using Array = EpicChain.VM.Types.Array;

namespace EpicChain.SmartContract.Manifest
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
