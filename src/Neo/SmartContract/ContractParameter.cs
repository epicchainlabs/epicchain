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
// ContractParameter.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.Cryptography.ECC;
using Neo.Json;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Text;

namespace Neo.SmartContract
{
    /// <summary>
    /// Represents a parameter of a contract method.
    /// </summary>
    public class ContractParameter
    {
        /// <summary>
        /// The type of the parameter.
        /// </summary>
        public ContractParameterType Type;

        /// <summary>
        /// The value of the parameter.
        /// </summary>
        public object Value;

        /// <summary>
        /// Initializes a new instance of the <see cref="ContractParameter"/> class.
        /// </summary>
        public ContractParameter() { }

        /// <summary>
        /// Initializes a new instance of the <see cref="ContractParameter"/> class with the specified type.
        /// </summary>
        /// <param name="type">The type of the parameter.</param>
        public ContractParameter(ContractParameterType type)
        {
            this.Type = type;
            this.Value = type switch
            {
                ContractParameterType.Any => null,
                ContractParameterType.Signature => new byte[64],
                ContractParameterType.Boolean => false,
                ContractParameterType.Integer => 0,
                ContractParameterType.Hash160 => new UInt160(),
                ContractParameterType.Hash256 => new UInt256(),
                ContractParameterType.ByteArray => Array.Empty<byte>(),
                ContractParameterType.PublicKey => ECCurve.Secp256r1.G,
                ContractParameterType.String => "",
                ContractParameterType.Array => new List<ContractParameter>(),
                ContractParameterType.Map => new List<KeyValuePair<ContractParameter, ContractParameter>>(),
                _ => throw new ArgumentException(null, nameof(type)),
            };
        }

        /// <summary>
        /// Converts the parameter from a JSON object.
        /// </summary>
        /// <param name="json">The parameter represented by a JSON object.</param>
        /// <returns>The converted parameter.</returns>
        public static ContractParameter FromJson(JObject json)
        {
            ContractParameter parameter = new()
            {
                Type = Enum.Parse<ContractParameterType>(json["type"].GetString())
            };
            if (json["value"] != null)
                parameter.Value = parameter.Type switch
                {
                    ContractParameterType.Signature or ContractParameterType.ByteArray => Convert.FromBase64String(json["value"].AsString()),
                    ContractParameterType.Boolean => json["value"].AsBoolean(),
                    ContractParameterType.Integer => BigInteger.Parse(json["value"].AsString()),
                    ContractParameterType.Hash160 => UInt160.Parse(json["value"].AsString()),
                    ContractParameterType.Hash256 => UInt256.Parse(json["value"].AsString()),
                    ContractParameterType.PublicKey => ECPoint.Parse(json["value"].AsString(), ECCurve.Secp256r1),
                    ContractParameterType.String => json["value"].AsString(),
                    ContractParameterType.Array => ((JArray)json["value"]).Select(p => FromJson((JObject)p)).ToList(),
                    ContractParameterType.Map => ((JArray)json["value"]).Select(p => new KeyValuePair<ContractParameter, ContractParameter>(FromJson((JObject)p["key"]), FromJson((JObject)p["value"]))).ToList(),
                    _ => throw new ArgumentException(null, nameof(json)),
                };
            return parameter;
        }

        /// <summary>
        /// Sets the value of the parameter.
        /// </summary>
        /// <param name="text">The <see cref="string"/> form of the value.</param>
        public void SetValue(string text)
        {
            switch (Type)
            {
                case ContractParameterType.Signature:
                    byte[] signature = text.HexToBytes();
                    if (signature.Length != 64) throw new FormatException();
                    Value = signature;
                    break;
                case ContractParameterType.Boolean:
                    Value = string.Equals(text, bool.TrueString, StringComparison.OrdinalIgnoreCase);
                    break;
                case ContractParameterType.Integer:
                    Value = BigInteger.Parse(text);
                    break;
                case ContractParameterType.Hash160:
                    Value = UInt160.Parse(text);
                    break;
                case ContractParameterType.Hash256:
                    Value = UInt256.Parse(text);
                    break;
                case ContractParameterType.ByteArray:
                    Value = text.HexToBytes();
                    break;
                case ContractParameterType.PublicKey:
                    Value = ECPoint.Parse(text, ECCurve.Secp256r1);
                    break;
                case ContractParameterType.String:
                    Value = text;
                    break;
                default:
                    throw new ArgumentException();
            }
        }

        /// <summary>
        /// Converts the parameter to a JSON object.
        /// </summary>
        /// <returns>The parameter represented by a JSON object.</returns>
        public JObject ToJson()
        {
            return ToJson(this, null);
        }

        private static JObject ToJson(ContractParameter parameter, HashSet<ContractParameter> context)
        {
            JObject json = new();
            json["type"] = parameter.Type;
            if (parameter.Value != null)
                switch (parameter.Type)
                {
                    case ContractParameterType.Signature:
                    case ContractParameterType.ByteArray:
                        json["value"] = Convert.ToBase64String((byte[])parameter.Value);
                        break;
                    case ContractParameterType.Boolean:
                        json["value"] = (bool)parameter.Value;
                        break;
                    case ContractParameterType.Integer:
                    case ContractParameterType.Hash160:
                    case ContractParameterType.Hash256:
                    case ContractParameterType.PublicKey:
                    case ContractParameterType.String:
                        json["value"] = parameter.Value.ToString();
                        break;
                    case ContractParameterType.Array:
                        if (context is null)
                            context = new HashSet<ContractParameter>();
                        else if (context.Contains(parameter))
                            throw new InvalidOperationException();
                        context.Add(parameter);
                        json["value"] = new JArray(((IList<ContractParameter>)parameter.Value).Select(p => ToJson(p, context)));
                        break;
                    case ContractParameterType.Map:
                        if (context is null)
                            context = new HashSet<ContractParameter>();
                        else if (context.Contains(parameter))
                            throw new InvalidOperationException();
                        context.Add(parameter);
                        json["value"] = new JArray(((IList<KeyValuePair<ContractParameter, ContractParameter>>)parameter.Value).Select(p =>
                        {
                            JObject item = new();
                            item["key"] = ToJson(p.Key, context);
                            item["value"] = ToJson(p.Value, context);
                            return item;
                        }));
                        break;
                }
            return json;
        }

        public override string ToString()
        {
            return ToString(this, null);
        }

        private static string ToString(ContractParameter parameter, HashSet<ContractParameter> context)
        {
            switch (parameter.Value)
            {
                case null:
                    return "(null)";
                case byte[] data:
                    return data.ToHexString();
                case IList<ContractParameter> data:
                    if (context is null) context = new HashSet<ContractParameter>();
                    if (context.Contains(parameter))
                    {
                        return "(array)";
                    }
                    else
                    {
                        context.Add(parameter);
                        StringBuilder sb = new();
                        sb.Append('[');
                        foreach (ContractParameter item in data)
                        {
                            sb.Append(ToString(item, context));
                            sb.Append(", ");
                        }
                        if (data.Count > 0)
                            sb.Length -= 2;
                        sb.Append(']');
                        return sb.ToString();
                    }
                case IList<KeyValuePair<ContractParameter, ContractParameter>> data:
                    if (context is null) context = new HashSet<ContractParameter>();
                    if (context.Contains(parameter))
                    {
                        return "(map)";
                    }
                    else
                    {
                        context.Add(parameter);
                        StringBuilder sb = new();
                        sb.Append('[');
                        foreach (var item in data)
                        {
                            sb.Append('{');
                            sb.Append(ToString(item.Key, context));
                            sb.Append(',');
                            sb.Append(ToString(item.Value, context));
                            sb.Append('}');
                            sb.Append(", ");
                        }
                        if (data.Count > 0)
                            sb.Length -= 2;
                        sb.Append(']');
                        return sb.ToString();
                    }
                default:
                    return parameter.Value.ToString();
            }
        }
    }
}
