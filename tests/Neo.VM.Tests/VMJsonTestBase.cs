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
// VMJsonTestBase.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.Test.Extensions;
using Neo.Test.Types;
using Neo.VM;
using Neo.VM.Types;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;
using System;
using System.Collections.Generic;
using System.Text;

namespace Neo.Test
{
    public abstract class VMJsonTestBase
    {
        /// <summary>
        /// Execute this test
        /// </summary>
        /// <param name="ut">Test</param>
        public void ExecuteTest(VMUT ut)
        {
            foreach (var test in ut.Tests)
            {
                Assert.IsFalse(string.IsNullOrEmpty(test.Name), "Name is required");

                using TestEngine engine = new();
                Debugger debugger = new(engine);

                if (test.Script.Length > 0)
                {
                    engine.LoadScript(test.Script);
                }

                // Execute Steps

                if (test.Steps != null)
                {
                    foreach (var step in test.Steps)
                    {
                        // Actions

                        if (step.Actions != null) foreach (var run in step.Actions)
                            {
                                switch (run)
                                {
                                    case VMUTActionType.Execute: debugger.Execute(); break;
                                    case VMUTActionType.StepInto: debugger.StepInto(); break;
                                    case VMUTActionType.StepOut: debugger.StepOut(); break;
                                    case VMUTActionType.StepOver: debugger.StepOver(); break;
                                }
                            }

                        // Review results

                        var add = string.IsNullOrEmpty(step.Name) ? "" : "-" + step.Name;

                        AssertResult(step.Result, engine, $"{ut.Category}-{ut.Name}-{test.Name}{add}: ");
                    }
                }
            }
        }

        /// <summary>
        /// Assert result
        /// </summary>
        /// <param name="engine">Engine</param>
        /// <param name="result">Result</param>
        /// <param name="message">Message</param>
        private void AssertResult(VMUTExecutionEngineState result, TestEngine engine, string message)
        {
            AssertAreEqual(result.State.ToString().ToLowerInvariant(), engine.State.ToString().ToLowerInvariant(), message + "State is different");
            if (engine.State == VMState.FAULT)
            {
                if (result.ExceptionMessage != null)
                {
                    AssertAreEqual(result.ExceptionMessage, engine.FaultException.Message, message + " [Exception]");
                }
                return;
            }
            AssertResult(result.InvocationStack, engine.InvocationStack, message + " [Invocation stack]");
            AssertResult(result.ResultStack, engine.ResultStack, message + " [Result stack] ");
        }

        /// <summary>
        /// Assert invocation stack
        /// </summary>
        /// <param name="stack">Stack</param>
        /// <param name="result">Result</param>
        /// <param name="message">Message</param>
        private void AssertResult(VMUTExecutionContextState[] result, Stack<ExecutionContext> stack, string message)
        {
            AssertAreEqual(result == null ? 0 : result.Length, stack.Count, message + "Stack is different");

            int x = 0;
            foreach (var context in stack)
            {
                var opcode = context.InstructionPointer >= context.Script.Length ? OpCode.RET : context.Script[context.InstructionPointer];

                AssertAreEqual(result[x].NextInstruction, opcode, message + "Next instruction is different");
                AssertAreEqual(result[x].InstructionPointer, context.InstructionPointer, message + "Instruction pointer is different");

                // Check stack

                AssertResult(result[x].EvaluationStack, context.EvaluationStack, message + " [EvaluationStack]");

                // Check slots

                AssertResult(result[x].Arguments, context.Arguments, message + " [Arguments]");
                AssertResult(result[x].LocalVariables, context.LocalVariables, message + " [LocalVariables]");
                AssertResult(result[x].StaticFields, context.StaticFields, message + " [StaticFields]");

                x++;
            }
        }

        /// <summary>
        /// Assert result stack
        /// </summary>
        /// <param name="stack">Stack</param>
        /// <param name="result">Result</param>
        /// <param name="message">Message</param>
        private void AssertResult(VMUTStackItem[] result, EvaluationStack stack, string message)
        {
            AssertAreEqual(stack.Count, result == null ? 0 : result.Length, message + "Stack is different");

            for (int x = 0, max = stack.Count; x < max; x++)
            {
                AssertAreEqual(ItemToJson(stack.Peek(x)).ToString(Formatting.None), PrepareJsonItem(result[x]).ToString(Formatting.None), message + "Stack item is different");
            }
        }

        /// <summary>
        /// Assert result slot
        /// </summary>
        /// <param name="slot">Slot</param>
        /// <param name="result">Result</param>
        /// <param name="message">Message</param>
        private void AssertResult(VMUTStackItem[] result, Slot slot, string message)
        {
            AssertAreEqual(slot == null ? 0 : slot.Count, result == null ? 0 : result.Length, message + "Slot is different");

            for (int x = 0, max = slot == null ? 0 : slot.Count; x < max; x++)
            {
                AssertAreEqual(ItemToJson(slot[x]).ToString(Formatting.None), PrepareJsonItem(result[x]).ToString(Formatting.None), message + "Stack item is different");
            }
        }

        private JObject PrepareJsonItem(VMUTStackItem item)
        {
            var ret = new JObject
            {
                ["type"] = item.Type.ToString(),
                ["value"] = item.Value
            };

            switch (item.Type)
            {
                case VMUTStackItemType.Null:
                    {
                        ret["type"] = VMUTStackItemType.Null.ToString();
                        ret.Remove("value");
                        break;
                    }
                case VMUTStackItemType.Pointer:
                    {
                        ret["type"] = VMUTStackItemType.Pointer.ToString();
                        ret["value"] = item.Value.Value<int>();
                        break;
                    }
                case VMUTStackItemType.String:
                    {
                        // Easy access

                        ret["type"] = VMUTStackItemType.ByteString.ToString();
                        ret["value"] = Encoding.UTF8.GetBytes(item.Value.Value<string>());
                        break;
                    }
                case VMUTStackItemType.ByteString:
                case VMUTStackItemType.Buffer:
                    {
                        var value = ret["value"].Value<string>();
                        Assert.IsTrue(string.IsNullOrEmpty(value) || value.StartsWith("0x"), $"'0x' prefix required for value: '{value}'");
                        ret["value"] = value.FromHexString();
                        break;
                    }
                case VMUTStackItemType.Integer:
                    {
                        // Ensure format

                        ret["value"] = ret["value"].Value<string>();
                        break;
                    }
                case VMUTStackItemType.Struct:
                case VMUTStackItemType.Array:
                    {
                        var array = (JArray)ret["value"];

                        for (int x = 0, m = array.Count; x < m; x++)
                        {
                            array[x] = PrepareJsonItem(JsonConvert.DeserializeObject<VMUTStackItem>(array[x].ToString()));
                        }

                        ret["value"] = array;
                        break;
                    }
                case VMUTStackItemType.Map:
                    {
                        var obj = (JObject)ret["value"];

                        foreach (var prop in obj.Properties())
                        {
                            obj[prop.Name] = PrepareJsonItem(JsonConvert.DeserializeObject<VMUTStackItem>(prop.Value.ToString()));
                        }

                        ret["value"] = obj;
                        break;
                    }
            }

            return ret;
        }

        private JToken ItemToJson(StackItem item)
        {
            if (item == null) return null;

            JToken value;
            string type = item.GetType().Name;

            switch (item)
            {
                case VM.Types.Null _:
                    {
                        return new JObject
                        {
                            ["type"] = type,
                        };
                    }
                case Pointer p:
                    {
                        return new JObject
                        {
                            ["type"] = type,
                            ["value"] = p.Position
                        };
                    }
                case VM.Types.Boolean v: value = new JValue(v.GetBoolean()); break;
                case VM.Types.Integer v: value = new JValue(v.GetInteger().ToString()); break;
                case VM.Types.ByteString v: value = new JValue(v.GetSpan().ToArray()); break;
                case VM.Types.Buffer v: value = new JValue(v.InnerBuffer.ToArray()); break;
                //case VM.Types.Struct v:
                case VM.Types.Array v:
                    {
                        var jarray = new JArray();

                        foreach (var entry in v)
                        {
                            jarray.Add(ItemToJson(entry));
                        }

                        value = jarray;
                        break;
                    }
                case VM.Types.Map v:
                    {
                        var jdic = new JObject();

                        foreach (var entry in v)
                        {
                            jdic.Add(entry.Key.GetSpan().ToArray().ToHexString(), ItemToJson(entry.Value));
                        }

                        value = jdic;
                        break;
                    }
                case VM.Types.InteropInterface v:
                    {
                        type = "Interop";
                        var obj = v.GetInterface<object>();

                        value = obj.GetType().Name.ToString();
                        break;
                    }
                default: throw new NotImplementedException();
            }

            return new JObject
            {
                ["type"] = type,
                ["value"] = value
            };
        }

        /// <summary>
        /// Assert with message
        /// </summary>
        /// <param name="expected">A</param>
        /// <param name="actual">B</param>
        /// <param name="message">Message</param>
        private static void AssertAreEqual(object expected, object actual, string message)
        {
            if (expected is byte[] ba) expected = ba.ToHexString().ToUpperInvariant();
            if (actual is byte[] bb) actual = bb.ToHexString().ToUpperInvariant();

            if (expected.ToJson() != actual.ToJson())
            {
                throw new Exception(message +
                    $"{Environment.NewLine}Expected:{Environment.NewLine + expected.ToString() + Environment.NewLine}Actual:{Environment.NewLine + actual.ToString()}");
            }
        }
    }
}
