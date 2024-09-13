// Copyright (C) 2021-2024 EpicChain Labs.

//
// ContractEventAttribute.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.SmartContract.Manifest;
using System;
using System.Diagnostics;

namespace EpicChain.SmartContract.Native
{
    [DebuggerDisplay("{Descriptor.Name}")]
    [AttributeUsage(AttributeTargets.Constructor, AllowMultiple = true)]
    internal class ContractEventAttribute : Attribute, IHardforkActivable
    {
        public int Order { get; init; }
        public ContractEventDescriptor Descriptor { get; set; }
        public Hardfork? ActiveIn { get; init; } = null;
        public Hardfork? DeprecatedIn { get; init; } = null;

        public ContractEventAttribute(Hardfork activeIn, int order, string name,
            string arg1Name, ContractParameterType arg1Value) : this(order, name, arg1Name, arg1Value)
        {
            ActiveIn = activeIn;
        }

        public ContractEventAttribute(Hardfork activeIn, int order, string name,
            string arg1Name, ContractParameterType arg1Value, Hardfork deprecatedIn) : this(activeIn, order, name, arg1Name, arg1Value)
        {
            DeprecatedIn = deprecatedIn;
        }

        public ContractEventAttribute(int order, string name, string arg1Name, ContractParameterType arg1Value)
        {
            Order = order;
            Descriptor = new ContractEventDescriptor()
            {
                Name = name,
                Parameters =
                [
                    new ContractParameterDefinition()
                    {
                        Name = arg1Name,
                        Type = arg1Value
                    }
                ]
            };
        }

        public ContractEventAttribute(int order, string name, string arg1Name, ContractParameterType arg1Value, Hardfork deprecatedIn)
            : this(order, name, arg1Name, arg1Value)
        {
            DeprecatedIn = deprecatedIn;
        }

        public ContractEventAttribute(Hardfork activeIn, int order, string name,
            string arg1Name, ContractParameterType arg1Value,
            string arg2Name, ContractParameterType arg2Value) : this(order, name, arg1Name, arg1Value, arg2Name, arg2Value)
        {
            ActiveIn = activeIn;
        }

        public ContractEventAttribute(Hardfork activeIn, int order, string name,
            string arg1Name, ContractParameterType arg1Value,
            string arg2Name, ContractParameterType arg2Value, Hardfork deprecatedIn) : this(activeIn, order, name, arg1Name, arg1Value, arg2Name, arg2Value)
        {
            DeprecatedIn = deprecatedIn;
        }

        public ContractEventAttribute(int order, string name,
            string arg1Name, ContractParameterType arg1Value,
            string arg2Name, ContractParameterType arg2Value)
        {
            Order = order;
            Descriptor = new ContractEventDescriptor()
            {
                Name = name,
                Parameters =
                [
                    new ContractParameterDefinition()
                    {
                        Name = arg1Name,
                        Type = arg1Value
                    },
                    new ContractParameterDefinition()
                    {
                        Name = arg2Name,
                        Type = arg2Value
                    }
                ]
            };
        }

        public ContractEventAttribute(int order, string name,
           string arg1Name, ContractParameterType arg1Value,
           string arg2Name, ContractParameterType arg2Value, Hardfork deprecatedIn) : this(order, name, arg1Name, arg1Value, arg2Name, arg2Value)
        {
            DeprecatedIn = deprecatedIn;
        }


        public ContractEventAttribute(Hardfork activeIn, int order, string name,
            string arg1Name, ContractParameterType arg1Value,
            string arg2Name, ContractParameterType arg2Value,
            string arg3Name, ContractParameterType arg3Value) : this(order, name, arg1Name, arg1Value, arg2Name, arg2Value, arg3Name, arg3Value)
        {
            ActiveIn = activeIn;
        }

        public ContractEventAttribute(int order, string name,
           string arg1Name, ContractParameterType arg1Value,
           string arg2Name, ContractParameterType arg2Value,
           string arg3Name, ContractParameterType arg3Value
           )
        {
            Order = order;
            Descriptor = new ContractEventDescriptor()
            {
                Name = name,
                Parameters =
                [
                    new ContractParameterDefinition()
                    {
                        Name = arg1Name,
                        Type = arg1Value
                    },
                    new ContractParameterDefinition()
                    {
                        Name = arg2Name,
                        Type = arg2Value
                    },
                    new ContractParameterDefinition()
                    {
                        Name = arg3Name,
                        Type = arg3Value
                    }
                ]
            };
        }

        public ContractEventAttribute(Hardfork activeIn, int order, string name,
            string arg1Name, ContractParameterType arg1Value,
            string arg2Name, ContractParameterType arg2Value,
            string arg3Name, ContractParameterType arg3Value,
            string arg4Name, ContractParameterType arg4Value) : this(order, name, arg1Name, arg1Value, arg2Name, arg2Value, arg3Name, arg3Value, arg4Name, arg4Value)
        {
            ActiveIn = activeIn;
        }

        public ContractEventAttribute(int order, string name,
            string arg1Name, ContractParameterType arg1Value,
            string arg2Name, ContractParameterType arg2Value,
            string arg3Name, ContractParameterType arg3Value,
            string arg4Name, ContractParameterType arg4Value
            )
        {
            Order = order;
            Descriptor = new ContractEventDescriptor()
            {
                Name = name,
                Parameters =
                [
                    new ContractParameterDefinition()
                    {
                        Name = arg1Name,
                        Type = arg1Value
                    },
                    new ContractParameterDefinition()
                    {
                        Name = arg2Name,
                        Type = arg2Value
                    },
                    new ContractParameterDefinition()
                    {
                        Name = arg3Name,
                        Type = arg3Value
                    },
                    new ContractParameterDefinition()
                    {
                        Name = arg4Name,
                        Type = arg4Value
                    }
                ]
            };
        }
    }
}
