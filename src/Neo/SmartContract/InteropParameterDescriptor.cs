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
// InteropParameterDescriptor.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.Cryptography.ECC;
using Neo.VM.Types;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Reflection;

namespace Neo.SmartContract
{
    /// <summary>
    /// Represents a descriptor of an interoperable service parameter.
    /// </summary>
    public class InteropParameterDescriptor
    {
        private readonly ValidatorAttribute[] validators;

        /// <summary>
        /// The name of the parameter.
        /// </summary>
        public string Name { get; }

        /// <summary>
        /// The type of the parameter.
        /// </summary>
        public Type Type { get; }

        /// <summary>
        /// The converter to convert the parameter from <see cref="StackItem"/> to <see cref="object"/>.
        /// </summary>
        public Func<StackItem, object> Converter { get; }

        /// <summary>
        /// Indicates whether the parameter is an enumeration.
        /// </summary>
        public bool IsEnum => Type.IsEnum;

        /// <summary>
        /// Indicates whether the parameter is an array.
        /// </summary>
        public bool IsArray => Type.IsArray && Type.GetElementType() != typeof(byte);

        /// <summary>
        /// Indicates whether the parameter is an <see cref="InteropInterface"/>.
        /// </summary>
        public bool IsInterface { get; }

        private static readonly Dictionary<Type, Func<StackItem, object>> converters = new()
        {
            [typeof(StackItem)] = p => p,
            [typeof(VM.Types.Pointer)] = p => p,
            [typeof(VM.Types.Array)] = p => p,
            [typeof(InteropInterface)] = p => p,
            [typeof(bool)] = p => p.GetBoolean(),
            [typeof(sbyte)] = p => (sbyte)p.GetInteger(),
            [typeof(byte)] = p => (byte)p.GetInteger(),
            [typeof(short)] = p => (short)p.GetInteger(),
            [typeof(ushort)] = p => (ushort)p.GetInteger(),
            [typeof(int)] = p => (int)p.GetInteger(),
            [typeof(uint)] = p => (uint)p.GetInteger(),
            [typeof(long)] = p => (long)p.GetInteger(),
            [typeof(ulong)] = p => (ulong)p.GetInteger(),
            [typeof(BigInteger)] = p => p.GetInteger(),
            [typeof(byte[])] = p => p.IsNull ? null : p.GetSpan().ToArray(),
            [typeof(string)] = p => p.IsNull ? null : p.GetString(),
            [typeof(UInt160)] = p => p.IsNull ? null : new UInt160(p.GetSpan()),
            [typeof(UInt256)] = p => p.IsNull ? null : new UInt256(p.GetSpan()),
            [typeof(ECPoint)] = p => p.IsNull ? null : ECPoint.DecodePoint(p.GetSpan(), ECCurve.Secp256r1),
        };

        internal InteropParameterDescriptor(ParameterInfo parameterInfo)
            : this(parameterInfo.ParameterType)
        {
            this.Name = parameterInfo.Name;
            this.validators = parameterInfo.GetCustomAttributes<ValidatorAttribute>(true).ToArray();
        }

        internal InteropParameterDescriptor(Type type)
        {
            this.Type = type;
            if (IsEnum)
            {
                Converter = converters[type.GetEnumUnderlyingType()];
            }
            else if (IsArray)
            {
                Converter = converters[type.GetElementType()];
            }
            else
            {
                IsInterface = !converters.TryGetValue(type, out var converter);
                if (IsInterface)
                    Converter = converters[typeof(InteropInterface)];
                else
                    Converter = converter;
            }
        }

        public void Validate(StackItem item)
        {
            foreach (ValidatorAttribute validator in validators)
                validator.Validate(item);
        }
    }
}
