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
// ScriptConverter.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.Test.Extensions;
using Neo.VM;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System;
using System.Linq;

namespace Neo.Test.Converters
{
    internal class ScriptConverter : JsonConverter
    {
        public override bool CanConvert(Type objectType)
        {
            return objectType == typeof(byte[]) || objectType == typeof(string);
        }

        public override object ReadJson(JsonReader reader, Type objectType, object existingValue, JsonSerializer serializer)
        {
            switch (reader.TokenType)
            {
                case JsonToken.String:
                    {
                        if (reader.Value is string str)
                        {
                            Assert.IsTrue(str.StartsWith("0x"), $"'0x' prefix required for value: '{str}'");
                            return str.FromHexString();
                        }
                        break;
                    }
                case JsonToken.Bytes:
                    {
                        if (reader.Value is byte[] data) return data;
                        break;
                    }
                case JsonToken.StartArray:
                    {
                        using var script = new ScriptBuilder();

                        foreach (var entry in JArray.Load(reader))
                        {
                            var mul = 1;
                            var value = entry.Value<string>();

                            if (Enum.IsDefined(typeof(OpCode), value) && Enum.TryParse<OpCode>(value, out var opCode))
                            {
                                for (int x = 0; x < mul; x++)
                                {
                                    script.Emit(opCode);
                                }
                            }
                            else
                            {
                                for (int x = 0; x < mul; x++)
                                {
                                    Assert.IsTrue(value.StartsWith("0x"), $"'0x' prefix required for value: '{value}'");
                                    script.EmitRaw(value.FromHexString());
                                }
                            }
                        }

                        return script.ToArray();
                    }
            }

            throw new FormatException();
        }

        public override void WriteJson(JsonWriter writer, object value, JsonSerializer serializer)
        {
            if (value is byte[] data)
            {
                int ip = 0;
                var array = new JArray();

                try
                {
                    for (ip = 0; ip < data.Length;)
                    {
                        var instruction = new Instruction(data, ip);

                        array.Add(instruction.OpCode.ToString().ToUpperInvariant());

                        // Operand Size

                        if (instruction.Size - 1 - instruction.Operand.Length > 0)
                        {
                            array.Add(data.Skip(ip + 1).Take(instruction.Size - 1 - instruction.Operand.Length).ToArray().ToHexString());
                        }

                        if (!instruction.Operand.IsEmpty)
                        {
                            // Data

                            array.Add(instruction.Operand.ToArray().ToHexString());
                        }

                        ip += instruction.Size;
                    }
                }
                catch
                {
                    // Something was wrong, but maybe it's intentioned

                    if (Enum.IsDefined(typeof(OpCode), data[ip]))
                    {
                        // Check if it was the content and not the opcode

                        array.Add(((OpCode)data[ip]).ToString().ToUpperInvariant());
                        array.Add(data[(ip + 1)..].ToHexString());
                    }
                    else
                    {
                        array.Add(data[ip..].ToHexString());
                    }
                }

                // Write the script

                writer.WriteStartArray();
                foreach (var entry in array) writer.WriteValue(entry.Value<string>());
                writer.WriteEndArray();

                // Double check - Ensure that the format is exactly the same

                using var script = new ScriptBuilder();

                foreach (var entry in array)
                {
                    if (Enum.TryParse<OpCode>(entry.Value<string>(), out var opCode))
                    {
                        script.Emit(opCode);
                    }
                    else
                    {
                        script.EmitRaw(entry.Value<string>().FromHexString());
                    }
                }

                if (script.ToArray().ToHexString() != data.ToHexString())
                {
                    throw new FormatException();
                }
            }
            else
            {
                throw new FormatException();
            }
        }
    }
}
