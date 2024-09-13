// Copyright (C) 2021-2024 EpicChain Labs.

//
// ContractAbi.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using System.Collections.Generic;
using System.Linq;
using Array = EpicChain.VM.Types.Array;

namespace EpicChain.SmartContract.Manifest
{
    /// <summary>
    /// Represents the ABI of a smart contract.
    /// </summary>
    /// <remarks>For more details, see XEP-14.</remarks>
    public class ContractAbi : IInteroperable
    {
        private IReadOnlyDictionary<(string, int), ContractMethodDescriptor> methodDictionary;

        /// <summary>
        /// Gets the methods in the ABI.
        /// </summary>
        public ContractMethodDescriptor[] Methods { get; set; }

        /// <summary>
        /// Gets the events in the ABI.
        /// </summary>
        public ContractEventDescriptor[] Events { get; set; }

        void IInteroperable.FromStackItem(StackItem stackItem)
        {
            Struct @struct = (Struct)stackItem;
            Methods = ((Array)@struct[0]).Select(p => p.ToInteroperable<ContractMethodDescriptor>()).ToArray();
            Events = ((Array)@struct[1]).Select(p => p.ToInteroperable<ContractEventDescriptor>()).ToArray();
        }

        public StackItem ToStackItem(ReferenceCounter referenceCounter)
        {
            return new Struct(referenceCounter)
            {
                new Array(referenceCounter, Methods.Select(p => p.ToStackItem(referenceCounter))),
                new Array(referenceCounter, Events.Select(p => p.ToStackItem(referenceCounter))),
            };
        }

        /// <summary>
        /// Converts the ABI from a JSON object.
        /// </summary>
        /// <param name="json">The ABI represented by a JSON object.</param>
        /// <returns>The converted ABI.</returns>
        public static ContractAbi FromJson(JObject json)
        {
            ContractAbi abi = new()
            {
                Methods = ((JArray)json!["methods"])?.Select(u => ContractMethodDescriptor.FromJson((JObject)u)).ToArray() ?? [],
                Events = ((JArray)json!["events"])?.Select(u => ContractEventDescriptor.FromJson((JObject)u)).ToArray() ?? []
            };
            if (abi.Methods.Length == 0) throw new FormatException();
            return abi;
        }

        /// <summary>
        /// Gets the method with the specified name.
        /// </summary>
        /// <param name="name">The name of the method.</param>
        /// <param name="pcount">The number of parameters of the method. It can be set to -1 to search for the method with the specified name and any number of parameters.</param>
        /// <returns>The method that matches the specified name and number of parameters. If <paramref name="pcount"/> is set to -1, the first method with the specified name will be returned.</returns>
        public ContractMethodDescriptor GetMethod(string name, int pcount)
        {
            if (pcount < -1 || pcount > ushort.MaxValue) throw new ArgumentOutOfRangeException(nameof(pcount));
            if (pcount >= 0)
            {
                methodDictionary ??= Methods.ToDictionary(p => (p.Name, p.Parameters.Length));
                methodDictionary.TryGetValue((name, pcount), out var method);
                return method;
            }
            else
            {
                return Methods.FirstOrDefault(p => p.Name == name);
            }
        }

        /// <summary>
        /// Converts the ABI to a JSON object.
        /// </summary>
        /// <returns>The ABI represented by a JSON object.</returns>
        public JObject ToJson()
        {
            var json = new JObject();
            json["methods"] = new JArray(Methods.Select(u => u.ToJson()).ToArray());
            json["events"] = new JArray(Events.Select(u => u.ToJson()).ToArray());
            return json;
        }
    }
}
