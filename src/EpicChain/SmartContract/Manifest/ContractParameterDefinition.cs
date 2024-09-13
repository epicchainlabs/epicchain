// Copyright (C) 2021-2024 EpicChain Labs.

//
// ContractParameterDefinition.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

namespace EpicChain.SmartContract.Manifest
{
    /// <summary>
    /// Represents a parameter of an event or method in ABI.
    /// </summary>
    public class ContractParameterDefinition : IInteroperable
    {
        /// <summary>
        /// The name of the parameter.
        /// </summary>
        public string Name { get; set; }

        /// <summary>
        /// The type of the parameter. It can be any value of <see cref="ContractParameterType"/> except <see cref="ContractParameterType.Void"/>.
        /// </summary>
        public ContractParameterType Type { get; set; }

        void IInteroperable.FromStackItem(StackItem stackItem)
        {
            Struct @struct = (Struct)stackItem;
            Name = @struct[0].GetString();
            Type = (ContractParameterType)(byte)@struct[1].GetInteger();
        }

        public StackItem ToStackItem(ReferenceCounter referenceCounter)
        {
            return new Struct(referenceCounter) { Name, (byte)Type };
        }

        /// <summary>
        /// Converts the parameter from a JSON object.
        /// </summary>
        /// <param name="json">The parameter represented by a JSON object.</param>
        /// <returns>The converted parameter.</returns>
        public static ContractParameterDefinition FromJson(JObject json)
        {
            ContractParameterDefinition parameter = new()
            {
                Name = json["name"].GetString(),
                Type = Enum.Parse<ContractParameterType>(json["type"].GetString())
            };
            if (string.IsNullOrEmpty(parameter.Name))
                throw new FormatException();
            if (!Enum.IsDefined(typeof(ContractParameterType), parameter.Type) || parameter.Type == ContractParameterType.Void)
                throw new FormatException();
            return parameter;
        }

        /// <summary>
        /// Converts the parameter to a JSON object.
        /// </summary>
        /// <returns>The parameter represented by a JSON object.</returns>
        public JObject ToJson()
        {
            var json = new JObject();
            json["name"] = Name;
            json["type"] = Type.ToString();
            return json;
        }
    }
}
