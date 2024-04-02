// EpicChain Copyright Project (2021-2024)
// 
// Copyright (c) 2021-2024 EpicChain
// 
// EpicChain is an innovative blockchain network developed and maintained by xmoohad. This copyright project outlines the rights and responsibilities associated with the EpicChain software and its related components.
// 
// 1. Copyright Holder:
//    - xmoohad
// 
// 2. Project Name:
//    - EpicChain
// 
// 3. Project Description:
//    - EpicChain is a decentralized blockchain network that aims to revolutionize the way digital assets are managed, traded, and secured. With its innovative features and robust architecture, EpicChain provides a secure and efficient platform for various decentralized applications (dApps) and digital asset management.
// 
// 4. Copyright Period:
//    - The copyright for the EpicChain software and its related components is valid from 2021 to 2024.
// 
// 5. Copyright Statement:
//    - All rights reserved. No part of the EpicChain software or its related components may be reproduced, distributed, or transmitted in any form or by any means, without the prior written permission of the copyright holder, except in the case of brief quotations embodied in critical reviews and certain other noncommercial uses permitted by copyright law.
// 
// 6. License:
//    - The EpicChain software is licensed under the EpicChain Software License, a custom license that governs the use, distribution, and modification of the software. The EpicChain Software License is designed to promote the free and open development of the EpicChain network while protecting the interests of the copyright holder.
// 
// 7. Open Source:
//    - EpicChain is an open-source project, and its source code is available to the public under the terms of the EpicChain Software License. Developers are encouraged to contribute to the development of EpicChain and create innovative applications on top of the EpicChain network.
// 
// 8. Disclaimer:
//    - The EpicChain software and its related components are provided "as is," without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall the copyright holder or contributors be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the EpicChain software or its related components.
// 
// 9. Contact Information:
//    - For inquiries regarding the EpicChain copyright project, please contact xmoohad at [email address].
// 
// 10. Updates:
//     - This copyright project may be updated or modified from time to time to reflect changes in the EpicChain project or to address new legal or regulatory requirements. Users and developers are encouraged to check the latest version of the copyright project periodically.

//
// ContractEventAttribute.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.SmartContract.Manifest;
using System;
using System.Diagnostics;

namespace Neo.SmartContract.Native
{
    [DebuggerDisplay("{Descriptor.Name}")]
    [AttributeUsage(AttributeTargets.Constructor, AllowMultiple = true)]
    internal class ContractEventAttribute : Attribute
    {
        public int Order { get; init; }
        public ContractEventDescriptor Descriptor { get; set; }
        public Hardfork? ActiveIn { get; init; } = null;

        public ContractEventAttribute(Hardfork activeIn, int order, string name,
            string arg1Name, ContractParameterType arg1Value) : this(order, name, arg1Name, arg1Value)
        {
            ActiveIn = activeIn;
        }

        public ContractEventAttribute(int order, string name, string arg1Name, ContractParameterType arg1Value)
        {
            Order = order;
            Descriptor = new ContractEventDescriptor()
            {
                Name = name,
                Parameters = new ContractParameterDefinition[]
                {
                    new ContractParameterDefinition()
                    {
                        Name = arg1Name,
                        Type = arg1Value
                    }
                }
            };
        }

        public ContractEventAttribute(Hardfork activeIn, int order, string name,
            string arg1Name, ContractParameterType arg1Value,
            string arg2Name, ContractParameterType arg2Value) : this(order, name, arg1Name, arg1Value, arg2Name, arg2Value)
        {
            ActiveIn = activeIn;
        }

        public ContractEventAttribute(int order, string name,
            string arg1Name, ContractParameterType arg1Value,
            string arg2Name, ContractParameterType arg2Value)
        {
            Order = order;
            Descriptor = new ContractEventDescriptor()
            {
                Name = name,
                Parameters = new ContractParameterDefinition[]
                {
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
                }
            };
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
                Parameters = new ContractParameterDefinition[]
                {
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
                }
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
                Parameters = new ContractParameterDefinition[]
                {
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
                }
            };
        }
    }
}
