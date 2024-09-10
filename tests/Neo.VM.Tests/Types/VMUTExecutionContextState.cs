// Copyright (C) 2021-2024 EpicChain Labs.

//
// VMUTExecutionContextState.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Neo.Test.Converters;
using Neo.VM;
using Newtonsoft.Json;

namespace Neo.Test.Types
{
    public class VMUTExecutionContextState
    {
        [JsonProperty]
        public int InstructionPointer { get; set; }

        [JsonProperty, JsonConverter(typeof(UppercaseEnum))]
        public OpCode NextInstruction { get; set; }

        // Stacks

        [JsonProperty]
        public VMUTStackItem[] EvaluationStack { get; set; }

        // Slots

        [JsonProperty]
        public VMUTStackItem[] StaticFields { get; set; }

        [JsonProperty]
        public VMUTStackItem[] Arguments { get; set; }

        [JsonProperty]
        public VMUTStackItem[] LocalVariables { get; set; }
    }
}
